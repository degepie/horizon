# Report: pack tags for mob behaviour and combat

advisor: rate-limited in this cloud session at the approach check; not consulted (see end).

## Prior work (2026-11-04, REST search; the GraphQL `gh pr list` is blocked here)
- #9403 (degepie, open, `fix/dp-entity-tags`) / issue #9402: advancement predicates only. **This branch carries
  #9403's commit unchanged as its first commit** (`4b37e295c`, cherry-pick of `aac36f65d`) and builds on its
  `WorldTables::pack_tags`. Merge #9403 first; then this PR's own diff is the second commit (`7aa1beaff`).
  Alternatively open this PR stacked on `fix/dp-entity-tags`.
- #8917 (Delete228, open, unassigned) "Player damage: load custom world damage types and tags", its PR #9076 closed
  unmerged. Overlap: damage *types* and the player damage catalog (`health.rs`, `damage_source.rs`,
  `damage-tags.json`). Left alone here (see below); only the `damage_type` tag reads in shields, the projectile check
  and enchantment conditions changed.
- #8497 / #8547 / #7843 (Akirabane, open drafts): a *client's* tags from the server's `UpdateTags`, and movement
  block tags. Different side (client) and registry (block/item); no shared functions. #9442 (open) reads an entity tag
  for copper golems' breathing (diff not inspected); if it adds a `w.data.entity_has` call it should become
  `w.entity_has` after this lands.
- No issue covers mobs/combat → `ISSUE.md` drafted (no `/claim` done: coordinator's call).

## Root cause
`MobData::load` loads `entity_type` and `worldgen/biome` tags with `PackOverlay::NONE`; `Enchantments::load_with`
reads `data/minecraft/tags/{entity_type,damage_type}` and ignores its overlay. Gameplay reads them through
`res.mob_data()`, `MobWorld.data` and `res.stations()` / `stations().enchantments`. Expected from
`TagLoader.loadTagsForExistingRegistries` (not checked against the jar here): all registries' tags come from the
world's pack stack.

## Change
- `game.rs`: `world_pack_tags!` / `world_tags!` macros (field-level borrows) and `Game::{entity_tags, biome_tags,
  damage_tags, entity_has, biome_has}`: #9403's `pack_tags` when the world has packs, else the game's own tables.
- `MobWorld` gets `entity_tags` and `biome_tags` (+ `MobWorld::entity_has`); `mob_world_base` fills them from the
  world. Test/probe constructions use `MobData`'s.
- `Enchantments::load_with`: entity/damage type tags through `items::load_tags(root, kind, overlay)` (also brings
  content-mod namespaces, like `MobData` already does), old reader as fallback on a parse error.

### Callers routed through the world's tags
mobs: `ai.rs` can_freeze (`freeze_immune_entity_types`), `goals.rs` + `shulker.rs` (`powder_snow_walkable_mobs`),
`living.rs` (`can_float_while_ridden`), `animal_brain.rs` (axolotl targets), `spawn_rules.rs`
`MobWorld::biome_tag` (spawn rules, natural spawning, finalizeSpawn and raids go through it).
game: `creatures.rs` biome_facts (farm-animal variants), golden dandelion (`cannot_be_age_locked`), player hurt
`#is_projectile`; `beehives.rs`; `boat_passengers.rs` ×2; `wandering_trader.rs`; `passengers.rs` (drowned);
`potions.rs` ×5 (`inverted_healing_and_harm`, undead); `potent_sulfur.rs` ×2; `block_entity_anims.rs` ×2
(`raiders`); `dragon.rs`; `shield.rs` (blocks_attacks damage tags); enchantment conditions (`enchant.rs`).

### Callers deliberately left
- `game/debug_lines.rs:93` F3 entity tag lines: drawn by the window client; client tags from the server are #8497.
- `game/advancements.rs` `in_tag` fallback: #9403 already reads `pack_tags` first.
- `game/commands/execute.rs` `biome_tag_has`: already reads the world's packs (`tag_entries`).
- `vanillagen/structure_mobs.rs:97`: structure mobs finalized in the generation workers, which have no world tables.
- `mod_content.rs:2279`: content-mod build-time biome tags, not world packs.
- `mobs.rs` `DamageKind::bypasses_*`, `Damage::bypasses_armor` etc.: hardcoded per-kind matches, not tag lookups;
  making mob damage data-driven is a bigger change (and #8917's ground).
- `health.rs` / `horizon-world/src/damage_source.rs` player damage tags (`damage-tags.json` catalog): #8917.
- `examples/probe/*_fixture.rs`: harness probes with no world packs.

## Verified
- `cargo test -p horizon-items --lib enchant::` incl. new `conditions_read_the_world_pack_tags` (pack member joins,
  `replace` drops the game's, a pack's own nested tag, game-only load unchanged).
- `cargo test -p horizon-client --no-default-features --lib`: 892 passed, 18 ignored.
  `cargo test -p horizon-client-lib --no-default-features --lib mobs::`: 505 passed.
- `cargo clippy -p horizon-items --all-targets`, `-p horizon-client --no-default-features --all-targets`: only
  pre-existing `manual_range_contains` warnings from the local 1.97 toolchain (none on changed lines).
- `cargo fmt --all`: clean. pr_ready --quick: all pass (2738 tests, 644 Python) except clippy on the untouched horizon-world aquifer lint (old toolchain); block in PR.md.

## Not verified (cloud: no `.local/`, no jar, no game)
- No MobWorld/Game-level test: those need the prepared resources (`MobData::load`, `with_test_world` return early
  without `.local/`). The routing is by construction, not by an end-to-end test.
- Vanilla behaviour not checked against the jar; no in-game run of the repro. Parity harnesses not run
  (candidates: `python3 tools/parity.py --changed upstream/main`, e.g. mob_ai, natural_spawn, potions).

## Risks
- Worlds with packs hold entity/damage tags twice (`pack_tags` and `stations.enchantments`); small maps.
- Enchantment tags now include content-mod namespaces (`c`, mods) even without world packs.
- Single player: the window client (remote) keeps the game's own tags; its mob prediction is unaffected until
  #8497-style client tags land.
