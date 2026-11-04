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

## Cost and risks

- With no pack asking (vanilla's own tree), `player_facts_asked` returns nothing for every trigger and nothing is
  copied.
- With BACAP, a trigger whose criteria ask clones the whole stats map, progress map or recipe set each time it
  fires. That is `tick` every tick, if BACAP's milestones use `tick` (I could not check). This was not measured.
  - A zero-copy alternative: a `World` method returning the acting player's records, plus a flag on `EntityView`
    for "this is the acting player".
  - A narrower one: index the exact ids asked and copy only those.
- Fail closed: some player views carry no facts, so criteria that ask about stats, advancements, recipes or
  level never pass for them. Before this change, those criteria passed for everyone. The views are:
  - another guest (`entity_view_of` `Target::Guest`; the killer guest in `lan_actions.rs` around line 4011);
  - a rider (`rider_entity`);
  - native mod criteria (`native_criteria.rs`).

## Size and commits

- 473 lines in `crates/` (`advancements.rs` +~390, of which ~300 are tests). The code part is ~170 lines.
- The task asked for two commits past ~400 lines. I pushed the code as one commit before measuring, and the rules
  forbid force-push, so the split is not in history. For two PRs, split by hunk: the `check_inventory_trigger`
  hunk (the `list.is_empty()` early return, `me`, `adv::conditions(&c["player"]…)`) is self-contained.
- pr_ready's "531 lines of code" counts `cloud-notes/` too.

## Overlaps with open PRs

- #9494 (entity nbt) edits the same `entity()` match and `EntityView`: expect a trivial conflict on field order.
- #9496 edits `item()`, near but not the same hunk.
- #9403 changes `in_tag` lookups in `game/advancements.rs`, different functions.

## advisor

Used once, before the approach. It suggested this copy design; the user's task asked for the same.
