> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

## What and why

Fixes #TBD (see ISSUE.md). Builds on #9403: its commit comes first here, unchanged; this PR's own diff is the second commit.

Mobs, spawning, potions, shields and enchantment conditions tested entity type, biome and damage type tags against the game's own tags only: `MobData` loaded them with `PackOverlay::NONE`, and `Enchantments::load_with` read `data/minecraft/tags/{entity_type,damage_type}` without its overlay. A data pack that adds a mob to `#freeze_immune_entity_types` or `#inverted_healing_and_harm`, or a biome to `#without_wandering_trader_spawns`, changed nothing.

- `game.rs`: `Game::{entity_tags, biome_tags, damage_tags, entity_has, biome_has}` (and `world_tags!` for field-level borrows) read #9403's `WorldTables::pack_tags` when the world has packs, the game's own tables otherwise.
- `MobWorld` gets `entity_tags` / `biome_tags` and `entity_has`, filled from the world in `mob_world_base`; mob AI (freezing, powder snow, floating while ridden, axolotl targets) and the spawn rules' biome check read them.
- Gameplay callers moved off `res.mob_data()` / `res.stations()`: farm-animal variants, golden dandelion, the player's `#is_projectile` check, beehives, boats, wandering traders, drowned, potions, potent sulfur, bells, the dragon, shields.
- `Enchantments::load_with` reads entity and damage type tags with the world's overlay.

Left on purpose: F3 tag lines (client side, #8497), `/execute if biome` (already reads packs), structure mobs in generation workers, the hardcoded `DamageKind::bypasses_*` and the player damage tag catalog (#8917).

## Kind

- [x] parity: Horizon now does what vanilla 26.3 does
- [ ] bug: a crash or a breakage
- [ ] perf: faster, or less memory
- [ ] feature: something new for players (listed under New in the release notes)
- [ ] extra: Horizon-only, off by default
- [ ] tooling: tools, CI, tests, harnesses
- [ ] docs

## Vanilla reference

Expected from `TagLoader.loadTagsForExistingRegistries` (run by `ReloadableServerResources` over the world's pack stack): every registry's tags come from the world's packs over the game's own. Not checked against the jar in this session.

No harness: no parity harness loads a world data pack into these tag reads; the unit test `enchant::tests::conditions_read_the_world_pack_tags` covers the enchantment conditions.

## Other projects' code

none

## Self-check

**pr_ready** (`tools/pr_ready.py --quick --base upstream/main` on vm, against upstream/main at 1989b1b59, 4 min 5 s)

| Check | Result |
| --- | --- |
| Scope | ✅ 23 files, +209 −69 |
| Cargo.lock | ✅ unchanged |
| Public tree | ✅ |
| Python tests | ✅ 644 passed (1 min 33 s) |
| rustfmt | ✅ (15 s) |
| Rust | ⚠️ 1.97.0 is older than CI's stable (1.99): clippy may flag lints CI doesn't, update with `rustup update` |
| clippy | ❌ failed; likely the old Rust above (25 s) |
| tests: horizon-client, horizon-client-lib, horizon-items | ✅ 2738 passed (3 min 17 s) |
| doc tests | ✅ 0 passed (8 s) |
| content root tests | ✅ 25 passed |
| tokens.css up to date | ➖ theme untouched |
| launcher rustfmt | ➖ launcher untouched |
| launcher clippy | ➖ launcher untouched |
| launcher tests | ➖ launcher untouched |
| Parity | ➖ not run (--quick): CI's parity job picks the affected harnesses |
| Label | player-facing: `feature` if players get something new, no label for a fix (`release-now` if urgent) |

clippy fails only on `crates/horizon-world/src/worldgen/aquifer/noise_based.rs:476` (`manual_range_contains`), untouched by this PR, from the local 1.97 toolchain; no clippy finding on changed lines.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
