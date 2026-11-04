<!-- Title: /data merge and /data modify on a minecart or boat answer "Nothing changed" -->
> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

### What happens

`/data get entity` reads minecarts and boats, but `/data merge entity` and `/data modify entity` refuse to
write them and answer `commands.data.merge.failed` ("Nothing changed. The specified properties already have
these values"), whatever the change is.

### Repro

```
/summon minecraft:minecart ~ ~ ~2 {Tags:["c"]}
/data merge entity @e[tag=c,limit=1] {DisplayState:{Name:"minecraft:furnace"},DisplayOffset:3,CustomName:"Cart"}
/data get entity @e[tag=c,limit=1] DisplayOffset
/summon minecraft:oak_boat ~2 ~ ~2 {Tags:["b"]}
/data merge entity @e[tag=b,limit=1] {CustomName:"Boat",NoGravity:1b}
```

Expected, from `DataCommands` → `EntityDataAccessor.setData` → `Entity.load` (any entity but a player): each
merge succeeds ("Modified entity data of …"), the cart shows a furnace 3/16 up, and `/data get` reads back
`3`.

Actual, read from the code (not yet tried in game): both merges answer "Nothing changed", and the cart keeps
its old data.

### Where

`crates/horizon-client/src/game/commands/data.rs:313` `apply_entity_nbt`: it has arms for `Mob`, `Item`,
`Fangs`, `Stand`, `Frame` and `Other` (markers). Every other target falls through to `_ => Err(stale())`.
That also covers a hosted world's other players (`Target::Guest`): vanilla refuses them with
`commands.data.entity.invalid`, the error the local player already gets.
