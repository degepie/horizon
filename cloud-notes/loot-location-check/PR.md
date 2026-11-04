> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

## What and why

Loot `location_check` (`crates/horizon-world/src/loot.rs`) read only `predicate.biomes` from a biome string:
offsets, position, dimension, structures, block, fluid, light, smokey and can_see_sky were ignored (the entry
dropped anywhere) and biome tags always failed. It now tests the full `LocationPredicate` at the roll's origin
moved by the offset, and a roll without an origin fails the check.

The predicate is shared, not copied: `LocationPredicate` and `BlockPredicate` moved from horizon-client's
`advancements.rs` to a new `horizon-world/src/location_predicate.rs` over a `PredicateWorld` trait;
advancements keep their signatures and wrap it through an adapter. The loot engine takes a `LootOrigin`
(position + world view) instead of a biome string; fishing, `/data`-set containers and chest minecarts pass
the level, chests rolled from a chunk's data pass a view that knows only the biome (other fields fail there, where they passed before).
Biome, structure, block and fluid tags now load with the loot data, packs included.

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

Expected from `LocationCheck.test` → `LocationPredicate.matches` (origin plus the block offset; no `ORIGIN`
fails) and `RandomizableContainer.unpackLootTable` (a container's origin is its block centre); not checked
against the jar in this session.
No harness: no parity harness covers loot `location_check`; unit tests cover each predicate field and the loot wiring.

## Other projects' code

none

## Self-check

**pr_ready** (`tools/pr_ready.py --quick --base upstream/main` on vm, against upstream/main at 1989b1b59, 2 min 12 s)

| Check | Result |
| --- | --- |
| Scope | ⚠️ 13 files, +926 −145; 941 lines of code: over 500, split it if it can ship in parts |
| Cargo.lock | ✅ unchanged |
| Public tree | ✅ |
| Python tests | ✅ 644 passed (1 min 16 s) |
| rustfmt | ✅ (13 s) |
| Rust | ⚠️ 1.97.0 is older than CI's stable (1.99): clippy may flag lints CI doesn't, update with `rustup update` |
| clippy | ❌ failed; likely the old Rust above (30 s) |
| tests: horizon-client, horizon-client-lib, horizon-world | ✅ 3127 passed (1 min 22 s) |
| doc tests | ✅ 0 passed (7 s) |
| content root tests | ➖ content root untouched |
| tokens.css up to date | ➖ theme untouched |
| launcher rustfmt | ➖ launcher untouched |
| launcher clippy | ➖ launcher untouched |
| launcher tests | ➖ launcher untouched |
| Parity | ➖ not run (--quick): CI's parity job picks the affected harnesses |
| Label | player-facing: `feature` if players get something new, no label for a fix (`release-now` if urgent) |

clippy ❌ above is `crates/horizon-world/src/worldgen/aquifer/noise_based.rs:476` (`manual_range_contains`), a file
this branch does not touch, flagged by this session's Rust 1.97; `cargo clippy -p horizon-world -p horizon-client
--all-targets` reports nothing in the changed files (its other warnings are the same lint in untouched
`dragon.rs` and `camera.rs`). Scope counts the `cloud-notes/` files; the code diff is ~560 lines, about half tests.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
