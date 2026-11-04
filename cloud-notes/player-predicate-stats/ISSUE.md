<!-- Title: Advancements: the player predicate ignores stats, advancements, recipes and level; inventory_changed ignores its player conditions -->
> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

## What happens

1. The `type_specific/player` entity sub-predicate tests only `gamemode`, `looking_at` and `input`
   (`crates/horizon-client/src/advancements.rs`, the `"minecraft:type_specific/player"` arm of `entity`).
   Its `stats`, `advancements`, `recipes` and `level` fields are read by nothing, so a criterion that uses them
   is granted as soon as its trigger fires.
2. `inventory_changed` never tests the criterion's `player` conditions
   (`crates/horizon-client/src/game/advancements.rs`, `check_inventory_trigger`), so a criterion that also asks
   something of the player (sneaking, a game mode, a statistic…) is granted on the item alone.

BlazeandCave's Advancements Pack uses these in about 120 advancements (every `*_milestone`, `65_hours_of_walking`,
`level_up`, `iron_man`, `diamond_clad`…): they are granted to anyone.

## Repro

A data pack with `data/t/advancement/walk.json`:

```json
{"criteria": {"c": {"trigger": "minecraft:tick", "conditions": {"player": [{"condition": "minecraft:entity_properties",
  "entity": "this", "predicate": {"type_specific/player": {"stats": [{"type": "minecraft:custom",
  "stat": "minecraft:walk_one_cm", "value": {"min": 100000000}}]}}}]}}}}
```

and `data/t/advancement/sneak_dirt.json`:

```json
{"criteria": {"c": {"trigger": "minecraft:inventory_changed", "conditions": {"items": [{"items": "minecraft:dirt"}],
  "player": [{"condition": "minecraft:entity_properties", "entity": "this", "predicate": {"flags": {"is_sneaking": true}}}]}}}}
```

Open a world with the pack. `t:walk` is granted on the first tick; `t:sneak_dirt` is granted when picking up dirt
while standing.

## Expected

Neither is granted (until 1 000 km walked / until dirt is picked up while sneaking). Expected from
`PlayerPredicate.matches` (level `MinMaxBounds`, each `StatMatcher` against the player's `ServerStatsCounter`,
each recipe known or not in the recipe book, each `AdvancementPredicate` — done or per-criterion — against
`PlayerAdvancements`) and from `InventoryChangeTrigger.TriggerInstance` whose `player` is tested by
`SimpleCriterionTrigger.trigger`.
