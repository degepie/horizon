<!-- Title: Chest minecarts draw their chest 2/16 too low (default display offset 6 instead of 8) -->
> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

### What happens

A chest minecart draws its chest lower than vanilla does. `default_display_offset` gives every cart kind
except the hopper cart the base offset of 6 sixteenths, but `MinecartChest` overrides
`getDefaultDisplayOffset` and returns 8.

### Repro

```
/summon minecraft:chest_minecart ~ ~ ~2
/summon minecraft:minecart ~1 ~ ~2 {DisplayState:{Name:"minecraft:chest"}}
```

Expected, from `MinecartChest.getDefaultDisplayOffset` (8; the value was checked in the 26.3 jar): the chest
cart's chest sits 2/16 at the cart's 0.75 scale (0.09375 blocks) higher than the chest in the plain cart,
which uses `AbstractMinecart.getDefaultDisplayOffset` (6).

Actual, read from the code (not yet compared in game): both chests are drawn at offset 6, the same height.

The same default also decides:
- whether `DisplayOffset` is saved (`addAdditionalSaveData` writes it only off the default): a chest cart
  summoned with `{DisplayOffset:8}` saves it, and one with `{DisplayOffset:6}` doesn't, the reverse of vanilla;
- the `DATA_ID_DISPLAY_OFFSET` value sent to clients and the cull box's height.

### Where

`crates/horizon-client/src/game/minecarts.rs:184` `default_display_offset`. The unit test
`display_offset_matches_vanilla_for_each_cart_kind` (same file) asserts 6 for the chest cart.
