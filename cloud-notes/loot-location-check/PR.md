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
the level, chests rolled from a chunk's data pass a view that knows only the biome (other fields fail there).
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

SELF_CHECK_PLACEHOLDER

🤖 Generated with [Claude Code](https://claude.com/claude-code)
