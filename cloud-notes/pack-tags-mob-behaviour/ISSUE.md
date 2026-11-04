<!-- Title: Mobs, spawning and combat ignore a world's data pack tags (entity type, biome, damage type) -->
> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

Follow-up of #9402 (advancement predicates). The same gap exists outside advancements: everything mobs,
spawning, potions, shields and enchantment conditions test against an entity type, biome or damage type tag
reads only the game's own tags, never the world's data packs.

## Minimal repro

A data pack `ghost` in a new world (`datapacks/ghost/`):

```
pack.mcmeta                                          {"pack":{"pack_format":<26.3's>,"description":""}}
data/minecraft/tags/entity_type/freeze_immune_entity_types.json   {"values":["minecraft:zombie"]}
data/minecraft/tags/entity_type/inverted_healing_and_harm.json    {"values":["minecraft:pig"]}
data/minecraft/tags/damage_type/is_projectile.json                {"values":["minecraft:mob_attack"]}
```

1. `/summon zombie` into powder snow: it still freezes and takes freeze damage.
2. Shoot a pig with an arrow of Healing: the effect heals it instead of hurting it.
3. Same for biome tags: a pack that adds `minecraft:plains` to `#minecraft:without_wandering_trader_spawns`
   still gets wandering traders there.

## Expected

The world's packs' tags over the game's own, for every registry (expected from
`TagLoader.loadTagsForExistingRegistries`, which `ReloadableServerResources` runs over the world's pack stack):
the zombie does not freeze, the Healing effect hurts the pig, no wandering trader in plains.

## Actual

The game's own tags only:

- `crates/horizon-client/src/mobs/data.rs` (`MobData::load`): `entity_tags` and `biome_tags` loaded with
  `PackOverlay::NONE`; read by mob AI through `MobWorld.data` and by gameplay through `res.mob_data()`.
- `crates/horizon-items/src/enchant.rs` (`Enchantments::load_with`): `entity_tags` and `damage_tags` read from
  `data/minecraft/tags/*` only, without the overlay it gets; read by potions, potent sulfur, bells, the dragon,
  shields and the player's projectile check through `stations().enchantments` (often `res.stations()`, which
  ignores the world's tables).
