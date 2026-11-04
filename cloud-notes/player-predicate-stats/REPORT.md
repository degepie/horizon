# player-predicate-stats: report for the coordinator

> 🤖 Written by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

Branch `cloud/player-predicate-stats`, based on `upstream/main` @ `63eccfc35`. BACAP audit items 1 and 2a.

## Prior work

There is no upstream PR or issue for this. I searched with the REST search API: `stats`, `advancements player`,
`inventory_changed`, `PlayerPredicate`, `StatMatcher`, `milestone`, `experience level advancement`.
`gh pr list` (GraphQL) and `repos/horizon-rust/horizon/pulls` were not reachable from this session, so the list
comes from search only. `git log upstream/main` has nothing either. No issue exists, so `ISSUE.md` is a draft.
Nothing was claimed.

## Root cause and fix

- `advancements.rs`, `entity()`: the `type_specific/player` arm tested only gamemode, looking_at and input.
  - New `PlayerFacts` (level, stats, advancement progress with its `Arc<Tree>`, known recipes), each part an
    `Option`, in `EntityView::player_facts`.
  - `player_facts()` tests the four fields, expected from `PlayerPredicate.matches`:
    - `StatMatcher`: type + stat, missing reads 0, bounds on `value`;
    - recipes: id → known;
    - advancements: id → done bool, or criterion → granted bool. An unknown id or an unknown criterion fails.
  - A field asked of a part that was not filled fails (fail closed).
- `Tree::index_player_facts`, built in `load_with` beside `index_looking_at`, gives `player_facts_asked(trigger)`.
  - It is a text search: a criterion holding `type_specific/player` together with `"level"`, `"stats"`,
    `"advancements"` or `"recipes"` marks the trigger for that part. A `reference` marks all four.
  - `recipe_unlocked`'s own `"recipes"` field does not count, because there is no player predicate around it.
- `game/advancements.rs`:
  - `player_entity_for` (now `player_entity_for_any(&[triggers])`) fills only the parts the triggers ask for.
  - `player_entity()` (commands, `execute if predicate`) fills all four.
  - Sources: `experience.level`, `stats.values` (if `stats.loaded`), `adv.tree` + `adv.progress` (if
    `adv.loaded`), `level.recipe_book.recipes`. `as_guest` swaps all of these, so a guest's turn reads the
    guest's own records.
- `check_inventory_trigger` now also tests `conditions(&c["player"], …)` on the player view, expected from
  `InventoryChangeTrigger.TriggerInstance` through `SimpleCriterionTrigger.trigger`. It returns early when no
  criterion is a candidate, so no view is built.
- `creatures.rs` `damage_view` (player killer) and `trigger_thrown_picked_up_by_player` now use
  `player_entity_for_any`, so their criteria get facts too.

## Verified

- Unit tests in `advancements.rs`, 5 new, each field with near misses:
  - level: bounds off by one, exact value;
  - stats: boundary, bare ids, a counter never counted reads 0, the same id under another type, all entries
    must match;
  - recipes: known/unknown × true/false;
  - advancements: done/not done, criterion true/false, unknown criterion, unknown advancement;
  - each field fails when its facts were not filled;
  - the trigger index: stats → tick, advancements+level → location, a reference → all four, `recipe_unlocked`
    and a gamemode-only player predicate → none.
- `cargo test -p horizon-client-lib --lib advancements::`: 29 ok. `cargo test -p horizon-client --lib
  advancement`: 10 ok. pr_ready `--quick`: tests 2682 ok, rustfmt ok.
- After the review fix, pr_ready `--quick`: 7 code files, tests 2683 ok, rustfmt ok, clippy fails only on the
  `noise_based.rs:476` lint described below. Only `load_with` (and tests) calls the tree indexes, so no other
  path builds a `Tree` without `index_player_facts`.
- clippy: no warnings in the changed files. pr_ready's clippy fails only on a `manual_range_contains` lint that
  is already on main (`noise_based.rs:476`, also `dragon.rs`, `camera.rs`). This runner has Rust 1.97; CI has 1.99.

## Not verified

- No vanilla jar, `rea_vanilla.py` or harness here: all vanilla behaviour is "expected from". These points are
  worth a check on the jar:
  - an unknown advancement id fails whatever is wanted;
  - an unknown criterion name fails;
  - an unknown stat id reads as 0 (vanilla probably rejects it when the criterion loads).
- `inventory_changed` + `player` is verified by reading only: no Game-level test.
- No in-game run with BACAP.

## Command predicates (coordinator review)

- `player_entity()` no longer fills anything. The command paths now call `entity_view_asked(t, &json)` or
  `entity_view_asking(t, asked)`, which fill only what the predicate they test may ask:
  - `functions.rs` `check_predicate` (`execute if predicate`);
  - `commands/compute.rs` (conditions);
  - `commands/items.rs` ×2 (item modifiers);
  - `scoreboard.rs` selector `predicate=`, the union over the selector's predicates.
- What a predicate may ask comes from `adv::facts_asked_in(&Value)`. It walks the JSON with no allocation and
  reads the fields of each `type_specific/player` object. A `minecraft:reference` gives all four: a condition
  `condition`/`type`, or an item modifier `function`.
- Unit test `a_command_predicate_asks_only_the_player_facts_it_names` covers the "not asked → not filled" decision
  on this path, including a modifier whose `levels` sits outside any player predicate.
  - There is no `Game` test fixture, so the wiring at the 5 call sites was checked by reading.
- `entity_view_of` is still used by the `looking_at` target path. There the player view gets no facts, because a
  player never looks at itself.

## LAN guests and single player

Single player's own player is a hub guest on the server thread (`integrated.rs`: "the hub runs without a host").

- **Triggers** (all of them: `tick`, `location`, `inventory_changed`, `player_killed_entity`, …):
  - they run inside `as_guest`, and `enter_guest` swaps in the guest's `adv.progress`, recipe book, `stats` and
    `experience`;
  - so `player_entity_for` fills that guest's own records;
  - guests do get facts there, not fail closed.
- **Commands where the player in place is the source** (a guest's own command, a function run as them in their
  turn): `Target::Player`, the guest's swapped records. They get facts.
- **Commands on another player** (`Target::Guest`): for example a tick or scheduled function, which runs with the
  server as source, doing `execute as @a if predicate …`. That includes single player's own player.
  - At first these got no facts, which failed closed.
  - They now get their own records: `guest_facts` reads `Guest.xp_level`, `Guest.advancements`,
    `Guest.recipe_book` and `host.play[conn].stats` (stats only if `loaded`).
  - Selectors leave out the guest in place, and `guest_facts` also refuses `lan_acting == conn`, so it never reads
    stale hub copies.
  - Advancements use `self.adv.tree`. If no guest turn has loaded the tree yet, an asked advancement is unknown
    and fails.
- **Still no facts (fail closed):**
  - the `EntityView`s built by hand for other guests inside trigger handlers: the guest killed in
    `lan_actions.rs` ~4011 (for `kill_mob_near_sculk_catalyst` / killed views) and a guest killer at ~4042;
  - `rider_entity` (a boat's passenger);
  - native mod criteria (`native_criteria.rs`).
  - Before this change, criteria asking stats/advancements/recipes/level passed for them. Now they fail.

## Cost and risks

- Vanilla's own tree asks for none of these, so no trigger copies anything. A command predicate that names none
  of the fields copies nothing.
- When asked, the whole map is cloned:
  - a trigger whose criteria ask clones on every fire, which for BACAP may be `tick` every tick if its
    milestones use it (unchecked);
  - a command predicate that asks clones on every check.
  - Not measured.
- To make the asked case cheaper: put `Progress`/`Stats` behind `Arc` snapshots, or index the exact ids asked.
  Both are larger changes to the game's records.

## Overlaps with open PRs

- #9494 (entity nbt) edits the same `entity()` match and `EntityView`: expect a trivial conflict on field order.
- #9496 edits `item()`, near but not the same hunk.
- #9403 changes `in_tag` lookups in `game/advancements.rs`, different functions.

## advisor

Used once, before the approach. It suggested this copy design; the user's task asked for the same. Used again before finishing. It asked for three things, all done:
- rerun pr_ready after the review fix and paste the new block into PR.md;
- check that no other path re-indexes a `Tree`;
- finish this section.

It was not rate-limited.
