> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

## What and why

When a chicken's egg timer ran out, Horizon queued the egg sound and the `chicken_lay` gift loot as two
separate events. So the sound played even when the table dropped nothing, and no `minecraft:entity_place`
game event was posted: a sculk sensor never heard a laid egg.

`chicken_egg_step` now raises one `MobEvent::LayEgg`. Its handler follows `Chicken.aiStep` (expected): it rolls
the gift table, and only when something dropped does it play `entity.chicken.egg` and post
`minecraft:entity_place` at the chicken, with the chicken as source. That is the order the armadillo's
`ShedScute` handler already uses. To support this, `drop_gift_loot` now returns whether it dropped anything
(`LivingEntity.dropFromGiftLootTable`'s boolean). The other `GiftLoot` users (turtle growth, panda sneeze) are
unchanged.

Test: `a_laid_egg_leaves_the_sound_to_the_drop` (`mobs/farm.rs`) checks that the step raises only `LayEgg`, with
the pitch and the next timer drawn from the chicken's random as before.

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

`Chicken.aiStep` (the egg branch), `LivingEntity.dropFromGiftLootTable`, `GameEvent.ENTITY_PLACE`.

No harness: `chicken_egg` (`tools/test_chicken_egg_parity.py`) checks the timer and the egg item but, as its docstring says, makes no sound or vibration claim.

## Other projects' code

none

## Self-check

**pr_ready** (`tools/pr_ready.py --quick --base upstream/main` on vm, against upstream/main at 1f90d6f9a, 8 min 47 s)

| Check | Result |
| --- | --- |
| Scope | ✅ 4 files, +70 −5 |
| Cargo.lock | ✅ unchanged |
| Public tree | ✅ |
| Python tests | ✅ 644 passed (2 min 3 s) |
| rustfmt | ✅ (17 s) |
| clippy | ✅ (3 min 59 s) |
| tests: horizon-client, horizon-client-lib | ✅ 2659 passed (4 min 26 s) |
| doc tests | ✅ 0 passed (5 s) |
| content root tests | ➖ content root untouched |
| tokens.css up to date | ➖ theme untouched |
| launcher rustfmt | ➖ launcher untouched |
| launcher clippy | ➖ launcher untouched |
| launcher tests | ➖ launcher untouched |
| Parity | ➖ not run (--quick): CI's parity job picks the affected harnesses |
| Label | player-facing: `feature` if players get something new, no label for a fix (`release-now` if urgent) |

Scope counts `cloud-notes/chicken-egg-event/ISSUE.md` too; the code change is `creatures.rs`, `mobs.rs` and `mobs/farm.rs` (+44 −5).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
