<!-- Title: Advancement block predicates ignore `components` and `predicates` -->
> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

A `BlockPredicate` (`location.block` in a `location` / `item_used_on_block` /
`placed_block` / `enter_block`-style criterion) that names `components` or
`predicates` matches whatever block entity is there, or none at all. The keys are
not read.

## Repro

A data pack with `data/t/advancement/named_chest.json`:

```json
{
  "criteria": {
    "c": {
      "trigger": "minecraft:item_used_on_block",
      "conditions": {
        "location": [{
          "condition": "minecraft:location_check",
          "predicate": {"block": {
            "blocks": "minecraft:chest",
            "components": {"minecraft:custom_name": "Vault"}
          }}
        }]
      }
    }
  }
}
```

1. Place a chest without a name and right-click it holding any item.
2. Rename a chest "Vault" in an anvil, place it, right-click it.

## Expected

Only step 2 grants `t:named_chest`. Expected from `BlockPredicate.matches` →
`BlockPredicate.matchesBlockEntity`: when `nbt` or `components`/`predicates` is set,
the position needs a block entity, and its collected components
(`BlockEntity.collectComponents`) must pass `DataComponentMatchers.test`. That means
every exact component is present with an equal value
(`DataComponentExactPredicate.test`), and every partial predicate
(`DataComponentPredicate`, e.g. `minecraft:container`, `minecraft:custom_data`)
holds.

## Actual

Step 1 grants it already. `adv::block` (`crates/horizon-client/src/advancements.rs`,
`pub fn block`) checks `blocks` and `state` only.

Related: #9405 does the same for `nbt`.
