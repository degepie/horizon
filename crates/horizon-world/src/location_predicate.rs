//! `LocationPredicate` and the `BlockPredicate` it holds, shared by the two
//! places that test them: advancement criteria (horizon-client's
//! `advancements.rs`) and the loot engine's `location_check` (`loot.rs`).
//! Both ask the world through [`PredicateWorld`].
use serde_json::Value;
use std::collections::BTreeMap;

/// What a location predicate may ask about the world at a position.
///
/// A view that cannot tell a field at a position (a chest rolled while its
/// chunk generates knows the biome and little else) answers `None`, and the
/// predicate field that needs it fails rather than guess (a made-up light 0
/// or open sky would let `{"can_see_sky": false}` pass anywhere).
pub trait PredicateWorld {
    /// The dimension's id (`minecraft:overworld`).
    fn dimension(&self) -> Option<&str>;
    fn biome(&self, pos: [i32; 3]) -> Option<String>;
    /// The structures whose pieces hold the position.
    fn structures(&self, pos: [i32; 3]) -> Option<Vec<String>>;
    /// Block name and properties.
    fn block(&self, pos: [i32; 3]) -> Option<(String, BTreeMap<String, String>)>;
    /// The fluid's id; `None` for no fluid (or an unknown one).
    fn fluid(&self, pos: [i32; 3]) -> Option<String>;
    fn can_see_sky(&self, pos: [i32; 3]) -> Option<bool>;
    /// `Level.getMaxLocalRawBrightness`.
    fn light(&self, pos: [i32; 3]) -> Option<i32>;
    /// `CampfireBlock.isSmokeyPos`.
    fn smokey(&self, pos: [i32; 3]) -> Option<bool>;
    /// Whether the registry's tag (a full id, no `#`) holds `id`.
    fn in_tag(&self, registry: &str, tag: &str, id: &str) -> bool;
}

/// A `MinMaxBounds`: an exact number or `{min, max}`; absent is any.
pub fn range_f(v: &Value, x: f64) -> bool {
    match v {
        Value::Null => true,
        Value::Number(n) => n.as_f64().is_some_and(|n| (n - x).abs() < 1e-9),
        Value::Object(o) => {
            o.get("min").and_then(Value::as_f64).is_none_or(|m| x >= m)
                && o.get("max").and_then(Value::as_f64).is_none_or(|m| x <= m)
        }
        _ => false,
    }
}

pub fn range_i(v: &Value, x: i64) -> bool {
    range_f(v, x as f64)
}

/// `Identifier.parse`: an id written without a namespace is a `minecraft:` one.
pub fn full(id: &str) -> std::borrow::Cow<'_, str> {
    if id.contains(':') {
        id.into()
    } else {
        format!("minecraft:{id}").into()
    }
}

/// A registry id, `#tag`, or list matched against one id.
pub fn id_matches(v: &Value, registry: &str, id: &str, w: &dyn PredicateWorld) -> bool {
    match v {
        Value::Null => true,
        Value::String(s) => match s.strip_prefix('#') {
            Some(tag) => w.in_tag(registry, &full(tag), id),
            None => full(s) == id,
        },
        Value::Array(a) => a.iter().any(|x| id_matches(x, registry, id, w)),
        _ => false,
    }
}

/// `BlockPredicate`.
pub fn block(v: &Value, pos: [i32; 3], w: &dyn PredicateWorld) -> bool {
    if v.is_null() {
        return true;
    }
    let Some((name, props)) = w.block(pos) else {
        return false;
    };
    if !id_matches(&v["blocks"], "block", &name, w) {
        return false;
    }
    if let Some(state) = v["state"].as_object() {
        for (k, want) in state {
            let have = props.get(k).cloned().unwrap_or_default();
            let ok = match want {
                Value::String(s) => have == *s,
                Value::Bool(b) => have == b.to_string(),
                Value::Number(n) => have == n.to_string(),
                Value::Object(_) => have.parse::<f64>().is_ok_and(|h| range_f(want, h)),
                _ => false,
            };
            if !ok {
                return false;
            }
        }
    }
    true
}

/// `LocationPredicate.matches` at `p`: the position bounds test `p` itself,
/// the other fields the block that holds it.
pub fn location(v: &Value, p: [f64; 3], w: &dyn PredicateWorld) -> bool {
    if v.is_null() {
        return true;
    }
    let pos = [
        p[0].floor() as i32,
        p[1].floor() as i32,
        p[2].floor() as i32,
    ];
    let posv = &v["position"];
    if !range_f(&posv["x"], p[0]) || !range_f(&posv["y"], p[1]) || !range_f(&posv["z"], p[2]) {
        return false;
    }
    if let Some(d) = v["dimension"].as_str()
        && w.dimension().is_none_or(|have| full(d) != have)
    {
        return false;
    }
    if !v["biomes"].is_null()
        && !w
            .biome(pos)
            .is_some_and(|b| id_matches(&v["biomes"], "worldgen/biome", &b, w))
    {
        return false;
    }
    if !v["structures"].is_null() {
        let here = w.structures(pos).unwrap_or_default();
        if !here
            .iter()
            .any(|s| id_matches(&v["structures"], "worldgen/structure", s, w))
        {
            return false;
        }
    }
    if let Some(b) = v["can_see_sky"].as_bool()
        && w.can_see_sky(pos) != Some(b)
    {
        return false;
    }
    if let Some(b) = v["smokey"].as_bool()
        && w.smokey(pos) != Some(b)
    {
        return false;
    }
    if !v["light"].is_null()
        && !w
            .light(pos)
            .is_some_and(|l| range_i(&v["light"]["light"], i64::from(l)))
    {
        return false;
    }
    if !v["block"].is_null() && !block(&v["block"], pos, w) {
        return false;
    }
    if !v["fluid"].is_null() {
        let Some(f) = w.fluid(pos) else {
            return false;
        };
        if !id_matches(&v["fluid"]["fluids"], "fluid", &f, w) {
            return false;
        }
    }
    true
}

/// A view that knows only the biome (and the dimension, when given) at
/// every position: a container rolled from a chunk's data, with no level
/// to ask. Every other field fails.
pub struct KnownBiome<'a> {
    pub biome: &'a str,
    pub dimension: Option<&'a str>,
}

impl PredicateWorld for KnownBiome<'_> {
    fn dimension(&self) -> Option<&str> {
        self.dimension
    }
    fn biome(&self, _: [i32; 3]) -> Option<String> {
        Some(self.biome.to_owned())
    }
    fn structures(&self, _: [i32; 3]) -> Option<Vec<String>> {
        None
    }
    fn block(&self, _: [i32; 3]) -> Option<(String, BTreeMap<String, String>)> {
        None
    }
    fn fluid(&self, _: [i32; 3]) -> Option<String> {
        None
    }
    fn can_see_sky(&self, _: [i32; 3]) -> Option<bool> {
        None
    }
    fn light(&self, _: [i32; 3]) -> Option<i32> {
        None
    }
    fn smokey(&self, _: [i32; 3]) -> Option<bool> {
        None
    }
    fn in_tag(&self, _: &str, _: &str, _: &str) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// A world with one stone block at the origin, under water from y 10,
    /// lit 7, smoky over y 0, inside a plains village of the overworld.
    struct Fixed;
    impl PredicateWorld for Fixed {
        fn dimension(&self) -> Option<&str> {
            Some("minecraft:overworld")
        }
        fn biome(&self, _: [i32; 3]) -> Option<String> {
            Some("minecraft:plains".into())
        }
        fn structures(&self, p: [i32; 3]) -> Option<Vec<String>> {
            Some(if p[0] < 100 {
                vec!["minecraft:village_plains".into()]
            } else {
                Vec::new()
            })
        }
        fn block(&self, p: [i32; 3]) -> Option<(String, BTreeMap<String, String>)> {
            if p == [0, 0, 0] {
                let props = BTreeMap::from([
                    ("axis".to_owned(), "y".to_owned()),
                    ("age".to_owned(), "3".to_owned()),
                ]);
                Some(("minecraft:oak_log".into(), props))
            } else {
                Some(("minecraft:air".into(), BTreeMap::new()))
            }
        }
        fn fluid(&self, p: [i32; 3]) -> Option<String> {
            (p[1] >= 10).then(|| "minecraft:water".into())
        }
        fn can_see_sky(&self, p: [i32; 3]) -> Option<bool> {
            Some(p[1] > 60)
        }
        fn light(&self, _: [i32; 3]) -> Option<i32> {
            Some(7)
        }
        fn smokey(&self, p: [i32; 3]) -> Option<bool> {
            Some(p[1] == 0)
        }
        fn in_tag(&self, registry: &str, tag: &str, id: &str) -> bool {
            match (registry, tag) {
                ("worldgen/biome", "minecraft:is_overworld") => id == "minecraft:plains",
                ("worldgen/structure", "minecraft:village") => id == "minecraft:village_plains",
                ("block", "minecraft:logs") => id == "minecraft:oak_log",
                ("fluid", "minecraft:water") => id == "minecraft:water",
                _ => false,
            }
        }
    }

    fn at(v: Value, p: [f64; 3]) -> bool {
        location(&v, p, &Fixed)
    }

    #[test]
    fn position_bounds_test_the_point_itself() {
        let p = json!({"position": {"x": {"min": 0.5, "max": 1.0}, "y": 3.25}});
        assert!(at(p.clone(), [0.5, 3.25, 9.0]));
        assert!(!at(p.clone(), [0.25, 3.25, 9.0]));
        assert!(!at(p, [0.5, 3.0, 9.0]));
        assert!(at(
            json!({"position": {"z": {"max": -2}}}),
            [0.0, 0.0, -2.0]
        ));
    }

    #[test]
    fn dimension_reads_bare_and_namespaced_ids() {
        assert!(at(json!({"dimension": "minecraft:overworld"}), [0.0; 3]));
        assert!(at(json!({"dimension": "overworld"}), [0.0; 3]));
        assert!(!at(json!({"dimension": "the_nether"}), [0.0; 3]));
    }

    #[test]
    fn biomes_take_an_id_a_tag_or_a_list() {
        assert!(at(json!({"biomes": "plains"}), [0.0; 3]));
        assert!(at(json!({"biomes": "#minecraft:is_overworld"}), [0.0; 3]));
        assert!(at(
            json!({"biomes": ["desert", "minecraft:plains"]}),
            [0.0; 3]
        ));
        assert!(!at(json!({"biomes": ["desert", "#is_nether"]}), [0.0; 3]));
    }

    #[test]
    fn structures_match_any_structure_holding_the_block() {
        assert!(at(json!({"structures": "village_plains"}), [0.0; 3]));
        assert!(at(json!({"structures": "#village"}), [5.0, 0.0, 0.0]));
        assert!(!at(json!({"structures": "#village"}), [150.0, 0.0, 0.0]));
        assert!(!at(json!({"structures": "minecraft:stronghold"}), [0.0; 3]));
    }

    #[test]
    fn block_tests_the_floored_position_name_tag_and_state() {
        let log = |b: Value| json!({"block": b});
        assert!(at(log(json!({"blocks": "oak_log"})), [0.9, 0.2, 0.7]));
        assert!(!at(log(json!({"blocks": "oak_log"})), [-0.1, 0.0, 0.0]));
        assert!(at(log(json!({"blocks": "#minecraft:logs"})), [0.0; 3]));
        assert!(at(log(json!({"state": {"axis": "y"}})), [0.0; 3]));
        assert!(!at(log(json!({"state": {"axis": "x"}})), [0.0; 3]));
        assert!(at(
            log(json!({"state": {"age": {"min": 2, "max": 3}}})),
            [0.0; 3]
        ));
        assert!(at(log(json!({"state": {"age": 3}})), [0.0; 3]));
        assert!(!at(log(json!({"state": {"age": {"min": 4}}})), [0.0; 3]));
    }

    #[test]
    fn fluid_needs_a_fluid_that_matches() {
        let water = json!({"fluid": {"fluids": "water"}});
        assert!(at(water.clone(), [0.0, 10.0, 0.0]));
        assert!(!at(water, [0.0, 9.0, 0.0]));
        assert!(at(json!({"fluid": {"fluids": "#water"}}), [0.0, 12.0, 0.0]));
        assert!(!at(json!({"fluid": {"fluids": "lava"}}), [0.0, 12.0, 0.0]));
        // Any fluid at all.
        assert!(!at(json!({"fluid": {}}), [0.0, 0.0, 0.0]));
    }

    #[test]
    fn light_is_a_range_on_the_brightness() {
        assert!(at(json!({"light": {"light": 7}}), [0.0; 3]));
        assert!(at(json!({"light": {"light": {"min": 5}}}), [0.0; 3]));
        assert!(!at(json!({"light": {"light": {"max": 6}}}), [0.0; 3]));
    }

    #[test]
    fn smokey_and_can_see_sky_compare_both_ways() {
        assert!(at(json!({"smokey": true}), [0.0, 0.5, 0.0]));
        assert!(!at(json!({"smokey": true}), [0.0, 1.0, 0.0]));
        assert!(at(json!({"smokey": false}), [0.0, 1.0, 0.0]));
        assert!(at(json!({"can_see_sky": true}), [0.0, 61.0, 0.0]));
        assert!(!at(json!({"can_see_sky": true}), [0.0, 60.0, 0.0]));
        assert!(at(json!({"can_see_sky": false}), [0.0, 60.0, 0.0]));
    }

    #[test]
    fn every_field_must_hold() {
        let p = json!({
            "dimension": "overworld",
            "biomes": "plains",
            "block": {"blocks": "oak_log"},
            "light": {"light": 7},
        });
        assert!(at(p.clone(), [0.0; 3]));
        assert!(!at(p, [1.0, 0.0, 0.0]));
        assert!(at(Value::Null, [0.0; 3]));
        assert!(at(json!({}), [0.0; 3]));
    }

    #[test]
    fn a_view_that_cannot_answer_fails_the_field() {
        let w = KnownBiome {
            biome: "minecraft:desert",
            dimension: None,
        };
        let t = |v: Value| location(&v, [0.0; 3], &w);
        assert!(t(json!({"biomes": "desert"})));
        assert!(t(json!({"position": {"x": 0}})));
        for unknown in [
            json!({"dimension": "overworld"}),
            json!({"structures": "village_plains"}),
            json!({"block": {}}),
            json!({"fluid": {}}),
            json!({"light": {"light": {"min": 0}}}),
            json!({"smokey": false}),
            json!({"can_see_sky": false}),
        ] {
            assert!(!t(unknown.clone()), "{unknown}");
        }
    }
}
