<!-- Title: Advancements: the entity_killed_player trigger never fires -->
> 🤖 Filed by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

No advancement criterion of trigger `minecraft:entity_killed_player` can be met: nothing fires the
trigger when a player dies. BlazeandCave's Advancements Pack has 17 advancements (68 criteria) built on
it, for example "We're Going on a Bear Hunt" and "Kung Fu Panda".

## Repro

A data pack with `data/t/advancement/zombie.json`:

```json
{
  "criteria": {
    "killed": {
      "trigger": "minecraft:entity_killed_player",
      "conditions": { "entity": { "minecraft:entity_type": "minecraft:zombie" } }
    }
  }
}
```

1. `/reload`, `/gamemode survival`, `/difficulty hard`.
2. `/summon minecraft:zombie ~ ~ ~2` and let it kill you.
3. `/advancement test @s t:zombie` (or look at the advancement screen).

## Expected

The advancement is granted. Expected from `ServerPlayer.die`: it calls `awardKillScore` on the entity
`LivingEntity.getKillCredit` names, and `Entity.awardKillScore` fires
`CriteriaTriggers.ENTITY_KILLED_PLAYER` (a `KilledTrigger`) for the dying player, with that entity as
`entity` and the death's damage source as `killing_blow`. For a skeleton's arrow, the skeleton is the
entity and the arrow is the damage source's direct entity.

## Actual

Not granted. `trigger_killed_as` is only called for `player_killed_entity` and
`kill_mob_near_sculk_catalyst` (`crates/horizon-client/src/game/advancements.rs:1000`,
`crates/horizon-client/src/game/creatures.rs:1378`, `crates/horizon-client/src/game/lan_actions.rs:4081`).
`guest_die` (`crates/horizon-client/src/game/lan_actions.rs:3844`), the server-side death of every
player (single player on its server thread, LAN guests, dedicated servers), never fires it.
