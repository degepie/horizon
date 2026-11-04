> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

## What and why

Fixes #TBD

No `minecraft:entity_killed_player` criterion could be met: nothing fired the trigger when a player died
(BlazeandCave's Advancements Pack has 17 advancements / 68 criteria on it, e.g. "We're Going on a Bear Hunt",
"Kung Fu Panda").

`guest_die` (every server-side player death: single player's server thread, LAN guests, dedicated servers)
now fires it for the dying player, between the deathCount scores and the `deaths` statistic. The `entity` is
the kill credit (the entity that last hurt the player within 100 ticks), the `killing_blow` the last combat
entry: for a skeleton's arrow, the skeleton is the causing entity and the arrow the direct one. With no kill
credit, nothing fires.

The killer lookup and the blow the sculk catalyst trigger built for the same death move into shared helpers
(`guest_entity_view`, `guest_killing_blow`, `advancements::recorded_blow`), and KilledTrigger's matching into
`advancements::killed`, both unchanged in behaviour.

Known gap: a killer that has already left the mob list (a creeper that exploded) is not resolved, so its kill
does not fire the trigger yet; the combat entry keeps only the attacker's id and name.

## Kind

- [x] parity: Horizon now does what vanilla 26.3 does
- [ ] bug: a crash or a breakage
- [ ] perf: faster, or less memory
- [ ] feature: something new for players (listed under New in the release notes)
- [ ] extra: Horizon-only, off by default
- [ ] tooling: tools, CI, tests, harnesses
- [ ] docs

## Vanilla reference

Expected from `ServerPlayer.die` → `LivingEntity.getKillCredit` → `Entity.awardKillScore` →
`CriteriaTriggers.ENTITY_KILLED_PLAYER` (`KilledTrigger.trigger`, with the damage source as `killing_blow`).
Not checked against the jar here.

No harness: no parity harness covers KilledTrigger on a player's death; unit tests in `advancements.rs` cover
the criterion matching for a mob's melee kill, a skeleton's arrow and a blow with no killer.

## Other projects' code

none

## Self-check

`python3 tools/pr_ready.py --quick --base upstream/main` (no `.local/` resources here):

```
SELF_CHECK_PLACEHOLDER
```

clippy fails on upstream/main's own lint at `crates/horizon-world/src/worldgen/aquifer/noise_based.rs:476`
(`manual_range_contains`, clippy 1.97), not on this change. `cargo clippy -p horizon-client -p horizon-client-lib
--all-targets` reports no warnings in the changed files. `cargo test -p horizon-client-lib --lib
advancements::tests::`: 27 passed, 1 ignored (needs the prepared resources).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
