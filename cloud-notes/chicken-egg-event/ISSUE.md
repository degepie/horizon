<!-- Title: A chicken plays its egg sound and skips entity_place when the egg loot table drops nothing -->
> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

## Repro

Read from the code, not tried in game. A way to see it:

1. Make a data pack whose `data/minecraft/loot_table/gameplay/chicken_lay.json` has no pools (`{"type":"minecraft:gift","pools":[]}`), load it in a world.
2. `/summon minecraft:chicken ~ ~ ~ {EggLayTime:20}` next to a sculk sensor (within 8 blocks).
3. Wait a second.

## Expected

Expected from `Chicken.aiStep` (with `LivingEntity.dropFromGiftLootTable` returning whether it dropped anything): when the egg timer runs out, the gift table is rolled; only if it dropped something does the chicken play `entity.chicken.egg` and post `GameEvent.ENTITY_PLACE` (at the chicken, the chicken as source). The timer resets either way. With the stock table an egg always drops, so: one egg, the sound, and a sculk sensor that picks up `minecraft:entity_place`.

## Actual

- The sound plays even when nothing drops (it is pushed before the loot roll).
- No `minecraft:entity_place` game event is posted at all, so a sculk sensor never reacts to a laid egg, even with the stock table.

## Where

- `crates/horizon-client/src/mobs/farm.rs:81` (`chicken_egg_step`): pushes `MobEvent::Sound("entity.chicken.egg", …)` and `MobEvent::GiftLoot` unconditionally.
- `crates/horizon-client/src/game/creatures.rs:3058` (`MobEvent::GiftLoot` handler) and `:1855` (`drop_gift_loot`): drop the loot, return nothing, post no game event.

Mentioned by a reviewer on #7881.
