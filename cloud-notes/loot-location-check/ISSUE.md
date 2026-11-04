<!-- Title: Loot `location_check` only tests the biome (and fails on biome tags) -->
> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

## Repro

A data pack with a fishing (or chest) table entry guarded by any `location_check` field other than `biomes`, e.g.

```json
{"type": "minecraft:location_check", "offsetY": -1, "predicate": {"block": {"blocks": "minecraft:sand"}}}
```

or a biome tag: `{"type": "minecraft:location_check", "predicate": {"biomes": "#minecraft:is_jungle"}}`.

## Expected

Expected from `LocationCheck.test` → `LocationPredicate.matches`: the whole predicate (position, dimension, biomes,
structures, block, fluid, light, smokey, can_see_sky) is tested at `ORIGIN` moved by the offset; a context
without `ORIGIN` fails. Biome tags resolve like any other tag.

## Actual

`crates/horizon-world/src/loot.rs` (`Context::condition`, the `"minecraft:location_check"` arm) reads only
`predicate.biomes`; every other field and the offsets are ignored (the entry drops anywhere), and a `#tag` biome
always fails ("Biome tags are not loaded here"). Advancements already test the full `LocationPredicate`
(`crates/horizon-client/src/advancements.rs`, `fn location`), but the loot engine cannot reach it.
