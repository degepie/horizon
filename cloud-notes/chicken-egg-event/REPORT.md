# chicken-egg-event: report for the coordinator

## Prior work (checked on 2026-11-05, base upstream/main 1f90d6f9a)
- REST search of upstream issues and PRs for "chicken egg", `entity_place`, `chicken_lay`, "egg sound" and
  `dropFromGiftLootTable`. Nearby, all closed: #7855 / PR #7881 (NoAI chickens tick their egg timer, where the
  reviewer raised this), PR #8041, PR #2137, PR #2226 (armadillo `entity_place`).
- No issue or PR posts `entity_place` for a chicken or makes the sound depend on the drop. I didn't diff every
  one of the ~98 open PRs. A search for `GiftLoot` and `chicken_lay` finds no open one.

## Root cause
`mobs/farm.rs` `chicken_egg_step` pushed `Sound("entity.chicken.egg")` and `GiftLoot` unconditionally. The
`GiftLoot` handler (`game/creatures.rs`) dropped the loot without reporting whether anything dropped and
without posting a game event.

## Change
- New `MobEvent::LayEgg { mob, at, pitch }`, raised by `chicken_egg_step` instead of those two events.
- `drop_gift_loot` returns `bool`; the existing `GiftLoot` arm ignores it.
- The `LayEgg` arm: drop; if dropped, play the sound (same `audio.play` path the `Sound` event took), then
  `post_game_event("minecraft:entity_place", at, who_context(Some(Who::Mob(mob)), None))`, as `ShedScute`
  does.

## Verified
- New unit test plus the existing farm, creatures and mobs tests: `pr_ready --quick --base upstream/main`
  passes (fmt, clippy, 2659 tests in horizon-client and horizon-client-lib, 644 Python tests). Block in PR.md.

## Not verified (needs the jar or a game)
- `Chicken.aiStep`'s exact order and condition (drop, then sound, then game event; timer reset either way).
- In game: a sculk sensor near a laying chicken should now activate (vibration frequency of `entity_place`),
  on both single-player paths and on LAN. Run `chicken_egg` and `sculk` parity.

## Known remaining difference (random order)
The pitch's two `nextFloat`s are still drawn from the chicken's random in `chicken_egg_step`, before the loot
roll and before `nextInt(6000)` for the timer. If vanilla evaluates the pitch inside the "dropped" branch (it
is an argument of `playSound` there, expected), then with a table that drops nothing vanilla skips those two
draws and Horizon doesn't. With the stock table an egg always drops, so the order matches. Fixing it would
mean drawing the pitch and the timer after the roll, a bigger restructure; I left it alone.

## Overlaps and risks
- Low risk. Only the chicken path changes. Turtle and panda gift loot are untouched.
