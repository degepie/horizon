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

No harness: no parity harness targets item-predicate component keys; the unit test is the check. Here
`tools/parity.py --changed-files <advancements.rs> --list` selected 416 of 417 harnesses (no `.local/` records
to narrow with), so CI's parity job decides which ones run.

## Other projects' code

none

## Self-check

**pr_ready** (`tools/pr_ready.py --quick --base upstream/main` on vm, against upstream/main at 1989b1b59, 3 min 24 s)

| Check | Result |
| --- | --- |
| Scope | ✅ 3 files, +156 −5 (includes cloud-notes; the code diff is 1 file) |
| Cargo.lock | ✅ unchanged |
| Public tree | ✅ |
| Python tests | ✅ 644 passed (1 min 27 s) |
| rustfmt | ✅ (14 s) |
| Rust | ⚠️ 1.97.0 is older than CI's stable (1.99): clippy may flag lints CI doesn't, update with `rustup update` |
| clippy | ❌ failed; likely the old Rust above (39 s) |
| tests: horizon-client-lib | ✅ 1773 passed (2 min 27 s) |
| doc tests | ✅ 0 passed (3 s) |
| content root tests | ➖ content root untouched |
| Parity | ➖ not run (--quick): CI's parity job picks the affected harnesses |

The clippy failure is `clippy::manual_range_contains` at `crates/horizon-world/src/worldgen/aquifer/noise_based.rs:476`,
which this branch does not touch, flagged by the old 1.97 clippy. `cargo clippy -p horizon-client-lib --all-targets
-- -D warnings -A clippy::manual_range_contains` is clean. To be rerun on current stable before publishing.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
