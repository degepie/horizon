# loot-location-check: report for the coordinator

advisor: available and used (before the design, and before finishing).

## Prior work

No upstream issue or PR on loot `location_check` (searched issues/PRs for `location_check`, `LocationPredicate`,
open `loot`/`predicate`/`location` PRs; `git log upstream/main`). No claim to make on an existing issue:
ISSUE.md is a draft to open and `/claim`.

Overlap to watch (same file, different functions):
- **#9405** (advancement block predicates test `nbt`) edits `advancements.rs` `fn block`. This branch moves
  that body to `horizon-world/src/location_predicate.rs` (`fn block`), so whichever lands second rebases:
  the `nbt` test would go into the shared `block` (and needs a block-entity query on `PredicateWorld`).
- **#9403** (advancement predicates read a world's data-pack tags) touches `World::in_tag` callers in
  `game/advancements.rs`. `LootView` here forwards `in_tag` to `View`, so it inherits that fix.

## Root cause

`loot.rs` had its own `location_check` that read only `predicate.biomes` from a biome string passed in by the
caller: no origin position, no offsets, no other field, no biome tags (LootData did not load `worldgen/biome`).

## Change and how the code is shared

- New `horizon-world/src/location_predicate.rs`: the `LocationPredicate`/`BlockPredicate` bodies moved there
  unchanged from `advancements.rs` (plus `range_f`, `range_i`, `full`, `id_matches`), over a `PredicateWorld`
  trait. Position queries return `Option`: a view that cannot answer fails the field instead of guessing.
- `advancements.rs` keeps every public signature; `location`, `block`, `id_matches` wrap the shared ones
  through `Shared(&dyn World)` (an adapter: the orphan rule forbids a blanket impl of a horizon-world trait for
  horizon-client's `World`). `full`/`range_i` are re-exported. No behaviour change for advancements.
- `loot.rs`: `roll`/`fill`/`fill_with` take `Option<LootOrigin>` (`pos` + `&dyn PredicateWorld`) instead of a
  biome string. `location_check`: no origin → false; else the predicate at origin + integer offsets.
  `in_tag` asks the loot data's tags first (now also `worldgen/biome`, `worldgen/structure`, `block`, `fluid`,
  packs included), then the world. `exploration_map`'s "has an origin" and a native mod's `biome` follow the origin.
- Callers: fishing (hook position), `/data`-set container tables and chest minecarts (game `LootView`: the
  real level, biome read as before via `biome_name_at`); chests rolled from a generating or loading chunk
  (`structure_blocks::block_entities`, `save::roll_loaded_loot`) only know the biome (`KnownBiome`), so
  any other field fails there, as before. Container origin is the block centre (expected from
  `RandomizableContainer.unpackLootTable`).

## Verified

- Unit tests, one per predicate field (`location_predicate::tests`), and the loot wiring
  (`loot::location_check_tests`: no origin fails, offsets move position and block, full predicate,
  biome tag through loot data tags, biome-only view fails what it cannot tell).
- `cargo fmt` clean; `cargo test -p horizon-world --lib` 360 ok; `cargo test -p horizon-client -p horizon-client-lib --lib`
  892 + 1716 ok; pr_ready --quick: all green except clippy, which fails only on a pre-existing
  `manual_range_contains` in untouched `worldgen/aquifer/noise_based.rs` (local Rust 1.97 < CI's 1.99; not updated
  to save disk). Nothing flagged in changed files.

## Not verified (no jar here)

- Vanilla behaviour is "expected from `LocationCheck.test` / `LocationPredicate.matches`" — not run against the
  26.3 jar. No parity harness covers loot `location_check`; `loot_fill` (examples/loot_fill.rs) still rolls
  without an origin.
- No in-game check (fishing in a jungle still gives bamboo: covered only by unit tests).

## Gaps left (out of scope)

- `roll_entity` (mob death loot) still rolls without an origin; vanilla's entity context has `ORIGIN`
  (expected from `LootContextParamSets.ENTITY`). Its `location_check` keeps failing as before.
- `horizon-items/src/items.rs` (block-drop engine) answers `location_check` with a flat `true`.
- `loot.rs` reads a pool's/entry's `condition` key; `items.rs` reads both `condition` and `conditions`. I could
  not check which key 26.3's data uses; worth a look by someone with `.local/`.
