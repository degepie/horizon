<!-- Title: Advancements: item predicate component keys without a namespace are ignored -->
> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

#8662 made bare ids, tags and keys in advancement predicates read as `minecraft:` ones, but listed one known
difference: the keys inside an item predicate's `components` and `predicates` objects are still looked up only
in their full form.

## Repro

A data pack advancement:

```json
{
  "criteria": {
    "drink": {
      "trigger": "minecraft:consume_item",
      "conditions": {"item": {"components": {"potion_contents": "minecraft:swiftness"}}}
    }
  }
}
```

Drink any potion (or eat bread).

## Expected

Only a Potion of Swiftness grants it. Expected from `ItemPredicate`'s codec: `components` is a
`DataComponentExactPredicate` keyed by `DataComponentType` registry ids and `predicates` a map keyed by the
sub-predicate type registry; both read their keys with `Identifier.parse`, where a missing namespace is
`minecraft`. The same holds for `damage` (components and predicates) and `enchantments` (predicates).

## Actual

The key `potion_contents` is not recognised and the check is skipped: any consumed item grants it.

## Where

`crates/horizon-client/src/advancements.rs`, `fn item`: `comps["minecraft:potion_contents"]`,
`comps["minecraft:damage"]`, `preds["minecraft:enchantments"]`, `preds["minecraft:damage"]`.
