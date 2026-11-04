> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

## What and why

`/data merge entity` and `/data modify entity` answered "Nothing changed" for minecarts and boats, though
`/data get entity` reads them. Vanilla's `EntityDataAccessor.setData` runs `Entity.load` on any entity but a
player. `apply_entity_nbt` now handles both kinds:

- **Minecarts:** `Minecart::from_nbt` is split into `new` plus a new `Minecart::load`, which reads the saved
  fields over the live cart: position, motion, rotation, NoGravity, items, furnace push and fuel, TNT fuse,
  DisplayState and DisplayOffset, the command block, the spawner. Riders, lerp and hurt state stay, and the
  kind never changes. The merge then does what loading a cart from a save does: an unreadable `DisplayState`
  is dropped and a `LootTable` is rolled (`roll_cart_loot`, now `pub(in crate::game)`).
- **Boats:** position, rotation, motion, NoGravity, and a chest boat's items.
- For both, the merged compound becomes the entity's kept compound, as for the other kinds, so
  `CustomName`, `Tags` and the rest come back on the next `/data get` and save. The UUID stays.
- A hosted world's other players (`Target::Guest`) now get `commands.data.entity.invalid`, like the local
  player, instead of "Nothing changed".

`/data get` is unchanged. Clients see the change because the entity tracker resends entity data that differs
(`lan/entity.rs`).

Test: `data_merge_loads_into_a_live_cart` merges `DisplayState`, `DisplayOffset`, `CustomName` and `Pos`
into a live chest cart, reads them back, and checks that a compound without a position changes nothing.

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

`DataCommands` (merge / modify) → `EntityDataAccessor.setData` → `Entity.load`;
`AbstractMinecart.readAdditionalSaveData` and its subclasses'; `AbstractBoat.readAdditionalSaveData`,
`AbstractChestBoat.readAdditionalSaveData` (`ContainerEntity.readChestVehicleSaveData`).

No harness: `command_exec` (`tools/command_exec_corpus.py`) runs `/data merge` and `/data modify` on mobs, items, armor stands and fangs, but not on a minecart or boat. The cart's load is unit-tested.

## Other projects' code

none

## Self-check

**pr_ready** (`tools/pr_ready.py --quick --base upstream/main` on vm, against upstream/main at 1f90d6f9a, 9 min 40 s)

| Check | Result |
| --- | --- |
| Scope | ✅ 6 files, +239 −4 |
| Cargo.lock | ✅ unchanged |
| Public tree | ✅ (1 s) |
| Python tests | ✅ 644 passed (2 min 33 s) |
| rustfmt | ✅ (20 s) |
| clippy | ✅ (6 min 8 s) |
| tests: horizon-client | ✅ 897 passed (3 min 9 s) |
| doc tests | ✅ 0 passed (4 s) |
| content root tests | ➖ content root untouched |
| tokens.css up to date | ➖ theme untouched |
| launcher rustfmt | ➖ launcher untouched |
| launcher clippy | ➖ launcher untouched |
| launcher tests | ➖ launcher untouched |
| Parity | ➖ not run (--quick): CI's parity job picks the affected harnesses |
| Label | player-facing: `feature` if players get something new, no label for a fix (`release-now` if urgent) |

Scope counts the three `cloud-notes/` files too; the code change is `commands/data.rs`, `minecarts.rs` and `persist.rs`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
