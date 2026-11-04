# entity_killed_player — report for the coordinator

Branch `cloud/entity-killed-player`, based on upstream/main `63eccfc35`. (The session header named
`claude/entity-killed-player-trigger-ab052o`; I followed the task's explicit `cloud/entity-killed-player`.)

## Prior work

No upstream PR or issue fires `entity_killed_player` (searched open and closed PRs/issues for
`entity_killed_player`, "killed player", advancement PRs touching `advancements.rs`; `git log upstream/main`).
Related, merged: #7768 (guest deaths count `deaths`/`deathCount`, same `guest_die` block), #4672
(`killing_blow` on KilledTrigger uses), #7902 (catalyst takes a dying player's xp, whose trigger code I reused).
No issue exists, so ISSUE.md is written and PR.md says `Fixes #TBD`.

## Root cause

Every server-side player death (single player's server thread, LAN guests, dedicated servers) goes through
`Game::guest_die` (`game/lan_actions.rs`). It runs the deathCount scores and the `deaths` stat but never
fires `entity_killed_player`. There is no other player-death path in gameplay (grep for `"deaths"`,
`score_player_died`, `guest_die`).

## Fix

- `guest_die`: between `score_player_died` and the `deaths` stat (the order expected from `ServerPlayer.die`:
  deathCount, then `getKillCredit().awardKillScore`, then `Stats.DEATHS`), call the new
  `guest_killed_player_trigger`. It takes the guest's `CombatTracker::kill_credit` (last hurt within 100
  ticks, `LivingEntity.getKillCredit`), resolves it to the host player / a guest / a mob, builds the killing
  blow from the last combat entry, and fires `trigger_killed_as("minecraft:entity_killed_player", …)` as the
  dying guest (`as_guest`).
- The id → `EntityView` lookup and the blow construction were inside `guest_catalyst_trigger`; I moved them
  into `guest_entity_view` / `guest_killing_blow` (game) and `adv::recorded_blow` (lib) so both triggers share
  them. The catalyst trigger's behaviour is unchanged (same lookups, same `player_attack` fallback).
- `KilledTrigger`'s matching moved from the closure in `trigger_killed_as` into `adv::killed` (lib), unchanged,
  so it is unit-testable.

## Known gap — read first

- **A creeper (or any mob already gone from `self.mobs`) as the killer: the trigger does not fire.** The kill
  credit is resolved by id against the host player, the guests and `self.mobs`. A creeper leaves the mob list
  when it explodes, and the `CombatTracker` explosion fields (`explosion_source`/`explosion_cause`) are cleared
  right after the blast (`creatures.rs` `MobEvent::Explode`), so by `guest_die` the creeper's id resolves to
  nothing and the trigger returns early. The combat entry keeps only the attacker's id and display name, not its
  type. Fixing it means recording the attacker's `EntityView` (or type) in the combat entry; left out to keep
  the PR small. Same for a killer in another dimension than the mobs loaded in `self.mobs` (the `here` check in
  `guest_die`).
- The arrow case relies on the combat tracker already recording the shooter as the entry's attacker (the
  death-message path); the unit tests build the blow by hand and do not exercise that.

## Decisions to know about

- **`entity` = the kill credit, not the blow's causing entity.** The task said "damage source's causing
  entity"; my reading of `ServerPlayer.die` is that the trigger runs from `killer.awardKillScore(this, source)`
  with `killer = getKillCredit()`, as Horizon's catalyst code for the same death already models. They differ
  for e.g. "a zombie hits you, you fall to death": entity = zombie, killing_blow = fall (no source entity).
  Expected from `ServerPlayer.die` / `Entity.awardKillScore`; I could not check the jar.
- **No credit → the trigger does not fire** (rather than firing with an entity that fails). Same observable
  result for criteria with an `entity` condition; differs only for a criterion with no `entity` key.
- **Projectile direct entity** is still the pre-existing heuristic from the damage type (`arrow`, `fireball`/
  `small_fireball`, `wither_skull`, `fireworks`). `trident`, `mob_projectile` (llama spit, shulker bullet…),
  `wind_charge`, `thrown` fall back to direct = causing entity, so `killing_blow.direct_entity` /
  `is_direct: false` criteria on those fail. Pre-existing gap, not widened here.
- Killer guests are described minimally (type, position, sneaking), as the catalyst code already did.

## Verified

- `cargo test -p horizon-client-lib --lib advancements::tests::` → 27 passed, 1 ignored (needs resources).
  New tests: mob melee (entity and source = the bear; a panda fails), skeleton arrow (entity/source = skeleton,
  direct = arrow, `is_direct: false`; a melee blow from the same skeleton fails), no killer (fall blow with no
  source fails `source_entity`; no blow fails any `killing_blow`; an empty criterion passes).
- clippy / fmt / pr_ready: see PR.md's Self-check.

## Not verified

- No `Game` test fixture exists, so the firing in `guest_die` itself is not unit-tested; it needs an in-game
  check: die to a zombie, to a skeleton's arrow, and by falling, with a data pack like ISSUE.md's (or BACAP's
  `were_going_on_a_bear_hunt`). Also `python3 tools/lan_e2e.py` for a LAN guest dying.
- No parity harness, no vanilla jar here (`.local/` absent): vanilla order and kill-credit reading are
  "expected from", not observed.

- ISSUE.md's repro writes the entity predicate as `{"minecraft:entity_type": …}` because that is the key
  `adv::entity` reads; confirm against a real 26.3 data pack before filing.
- `guest_die` now swaps into the guest twice (before and after the trigger) to keep the expected order;
  `enter_guest`/`leave_guest` only swap state, no packets.

## Overlaps

- `game/lan_actions.rs` `guest_die` / `guest_catalyst_trigger`, `game/advancements.rs` `trigger_killed_as`,
  `advancements.rs` (new fns after `killed_by_arrow`, tests after `killing_blow_needs_the_fireball`).
  Open PRs #9494, #9496, #9403, #9405 touch `advancements.rs` predicate code (`entity`, item predicates,
  tags); mine adds functions in a separate place, so conflicts should be trivial.
