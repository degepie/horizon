# data-merge-vehicles: report for the coordinator

## Prior work (checked on 2026-11-05, base upstream/main 1f90d6f9a)
- Searched upstream issues and PRs through the REST search API for "data merge entity/minecart/boat",
  "data modify entity", `apply_entity_nbt` and "Nothing changed". `gh issue view` and `gh pr list` don't
  work from this session.
- Nearby, all closed: #8929 / PR #8940 (markers got their `apply_entity_nbt` arm), #8967, #8752, #8998.
  Open #9363 ("Mob NBT edits clear the internal damage cooldown") touches the Mob arm only; this change
  doesn't touch that arm. No issue or PR covers carts or boats.
- Task 1 (`cloud/minecart-display-offsets`) and this branch both edit `minecarts.rs`, in different functions
  (`default_display_offset` there; `from_nbt`/`load` and a test here). Expect at most a trivial rebase.

## Root cause
`apply_entity_nbt` (`game/commands/data.rs`) had arms only for Mob, Item, Fangs, Stand, Frame and Other
(markers). Any other `Target` fell through to `commands.data.merge.failed`, the "Nothing changed" message.

## Change
- `Minecart::from_nbt` = `Minecart::new` + the new `Minecart::load`, which reads the saved fields onto a live
  cart. The cart's own reads didn't change, only where they run. `load` returns `None` before changing
  anything when there is no position.
- New Cart arm: `load` on a clone, then `forget_unreadable_display` and `roll_cart_loot` (as `load_cart`
  does), then the clone replaces the cart.
- New Boat arm: position, previous, rotation, motion, NoGravity, and a chest boat's `Items`.
- Both arms keep the merged compound in `persist.raw` with the entity's UUID, so `CustomName`, `Tags` and the
  other kept keys read back and save.
- `Target::Guest` (other players of a hosted world) now gets `commands.data.entity.invalid`, like the local
  player, instead of falling into "Nothing changed". Expected from `EntityDataAccessor.setData`, which refuses
  players.

## Kinds `/data get entity` reads but merge/modify still can't write
Painting, Orb, Projectile (arrows and the rest), Trident, Rocket (firework), Tnt (primed), Falling (falling
block), Crystal, Dragon, Cloud (area effect cloud), Spawner (ominous item spawner). Each still answers
"Nothing changed". Other displays and interactions are `Other` and also still fail: the Other arm accepts only
markers.

## Verified
- `cargo test -p horizon-client --lib minecarts::`: 19 passed, including the new `data_merge_loads_into_a_live_cart`.
- `pr_ready.py --quick --base upstream/main` (Rust 1.99): fmt, clippy, 897 horizon-client tests and 644
  Python tests pass. Block in PR.md.
- The Boat and Guest arms have no unit test: they need a whole `Game`, and there is no test fixture for one.

## Not verified (needs your machines)
- In game: the ISSUE.md repro, on both single-player paths and on LAN.
  - The cart should show the furnace at offset 3, and `/data get` should read 3.
  - The boat should keep `CustomName` and `NoGravity`.
  - A client should see the new display state. The tracker resends entity data that differs
    (`lan/entity.rs:422`), but I didn't run it.
- Vanilla fields that `Entity.load` reads and these arms don't apply:
  - boat: `leash`, which I left out because the live leash lives elsewhere and re-reading `leash_pending`
    could re-attach a stale holder;
  - both: `Fire`, `Air`, `Invulnerable`, `Silent`, `Glowing`, `PortalCooldown` and the rest of the generic
    `Entity` fields. Horizon doesn't model these per vehicle beyond the kept compound, so they read back but
    don't act.
- `tools/cart_portal_player_e2e.py:89` runs `data merge entity @e[tag=L0,limit=1] {Motion:[0.4d,0.0d,0.0d]}`
  on a boat. Until now that merge did nothing; now it pushes the boat. Run that e2e: it may behave differently
  (better, presumably, since the comment there expects the push).

## Risks
- Low to medium. `Minecart::load` is now also the path for every saved cart. Its reads are unchanged and the
  minecart unit tests pass.
- A merge sets `previous = position` (no interpolation from the old spot), as the Mob arm does.
