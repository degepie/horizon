> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

## What and why

Fixes #TBD

#8662 made bare ids, tags and keys in advancement predicates read as `minecraft:` ones, and listed one known
difference: the keys inside an item predicate's `components` and `predicates` objects were still looked up only
in their full form. A data pack writing `{"components": {"potion_contents": "minecraft:swiftness"}}` had its key
ignored, so the check was skipped and every stack matched.

`fn item` (`crates/horizon-client/src/advancements.rs`) now finds those entries through a small `keyed` helper
that compares `full(key)` with the registry id, for the four keys it reads: `components.potion_contents`,
`components.damage`, `predicates.enchantments` and `predicates.damage`. The `potion_contents` value is a registry
id too, so a bare potion id now matches as well, the way the entity `components` check already compares
`full(v) == full(want)`.

A unit test checks each key written without a namespace: a matching value passes and a non-matching value fails.
Before the fix, the failing case let every stack through.

## Kind

- [x] parity: Horizon now does what vanilla 26.3 does
- [ ] bug: a crash or a breakage
- [ ] perf: faster, or less memory
- [ ] feature: something new for players (listed under New in the release notes)
- [ ] extra: Horizon-only, off by default
- [ ] tooling: tools, CI, tests, harnesses
- [ ] docs

## Vanilla reference

Expected from `ItemPredicate`'s codec: `components` is a `DataComponentExactPredicate` keyed by `DataComponentType`
registry ids, and `predicates` is a map keyed by the `DataComponentPredicate` type registry. Both read their keys
with `Identifier.parse`, where a missing namespace is `minecraft`. The `potion_contents` value goes through the
potion registry's id codec the same way. I could not check this against the 26.3 jar from this session.

Known difference: if a pack writes the same component as both `damage` and `minecraft:damage`, vanilla is
expected to reject the duplicate key when it loads the pack, while Horizon uses the first match.

No harness: the change is in advancement predicate matching, which the unit test covers; no parity harness drives
item predicates with bare keys.

## Other projects' code

none

## Self-check

SELF_CHECK_PLACEHOLDER

🤖 Generated with [Claude Code](https://claude.com/claude-code)
