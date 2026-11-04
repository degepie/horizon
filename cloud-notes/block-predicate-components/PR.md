> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

## What and why

Advancement `BlockPredicate`s ignored `components` and `predicates`, so a criterion
that names them matched any block entity, or a plain block with none. They are now
tested against the block entity's collected components
(`block_entity_item::collected_components`, Horizon's `BlockEntity.collectComponents`):

- `components`: each listed component must be present and equal. JSON is compared
  with the saved NBT by value: number widths, booleans as bytes, ids with or without
  `minecraft:`, and a bare-string text component.
- `predicates`: `minecraft:container` (a collection predicate of item predicates over
  the non-empty stacks) and `minecraft:custom_data` (an NBT predicate). Any other
  partial type does not match. Block entities do not carry those components.
- Either key fails where there is no block entity.

`adv::World` gets a `block_entity_components` method (default `None`). The game's
`View` implements it from `saved_block_entity`, which becomes `pub(super)`. This
composes with #9405 (`nbt`): each check reads its own `World` method.

Fixes #TBD

## Kind

- [x] parity: Horizon now does what vanilla 26.3 does
- [ ] bug: a crash or a breakage
- [ ] perf: faster, or less memory
- [ ] feature: something new for players (listed under New in the release notes)
- [ ] extra: Horizon-only, off by default
- [ ] tooling: tools, CI, tests, harnesses
- [ ] docs

## Vanilla reference

Expected from `BlockPredicate.matches` → `BlockPredicate.matchesBlockEntity` →
`DataComponentMatchers.test` (`DataComponentExactPredicate.test`,
`DataComponentPredicate.Single`, `ContainerPredicate`, `CustomDataPredicate`,
`CollectionPredicate`) on `BlockEntity.collectComponents`. This was not checked
against the jar here.

No harness: no parity harness covers advancement block predicates. Unit tests in
`advancements::tests` cover a match, a near miss and the case with no block entity,
for both keys.

## Other projects' code

none

## Self-check

**pr_ready** (`tools/pr_ready.py --quick --base upstream/main` on vm, against upstream/main at 1989b1b59, 3 min 42 s)

| Check | Result |
| --- | --- |
| Scope | ✅ 3 files, +332 −1 |
| Cargo.lock | ✅ unchanged |
| Public tree | ✅ |
| Python tests | ✅ 644 passed (1 min 23 s) |
| rustfmt | ✅ (15 s) |
| Rust | ⚠️ 1.97.0 is older than CI's stable (1.99): clippy may flag lints CI doesn't, update with `rustup update` |
| clippy | ❌ failed; likely the old Rust above (39 s) |
| tests: horizon-client, horizon-client-lib | ✅ 2677 passed (2 min 42 s) |
| doc tests | ✅ 0 passed (7 s) |
| content root tests | ➖ content root untouched |
| tokens.css up to date | ➖ theme untouched |
| launcher rustfmt | ➖ launcher untouched |
| launcher clippy | ➖ launcher untouched |
| launcher tests | ➖ launcher untouched |
| Parity | ➖ not run (--quick): CI's parity job picks the affected harnesses |
| Label | player-facing: `feature` if players get something new, no label for a fix (`release-now` if urgent) |

The clippy failure is a lint in unchanged code (`horizon-world/src/worldgen/aquifer/noise_based.rs:476`,
`manual_range_contains`) under the local Rust 1.97. `cargo clippy -p horizon-client-lib -p horizon-client --all-targets`
reports no warnings in the changed files.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
