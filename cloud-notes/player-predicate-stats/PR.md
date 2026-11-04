> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

## What and why

Fixes #TBD

Two false grants in advancement criteria:

- The player sub-predicate (`type_specific/player`) tested only `gamemode`, `looking_at` and `input`. Its `stats`,
  `advancements` (done, or per criterion), `recipes` and `level` were read by nothing, so a criterion using them
  passed as soon as its trigger fired. They are now tested.
- `inventory_changed` never tested the criterion's `player` conditions. It does now.

What a player predicate reads beyond the entity goes in a new `EntityView::player_facts`. Each part (level,
statistics, advancement progress, known recipes) is filled only for the triggers whose criteria may ask for it.
`Tree::index_player_facts` finds them by searching the criteria's text, the same way `index_looking_at` does, so
ordinary play copies nothing. A part that wasn't filled fails the predicate that asks for it: another player's view
(a rider, a command target that is a guest, a guest killer) carries none of these records.

The killer view in `damage_view` and the thrown-item pickup view now go through `player_entity_for_any`, so their
criteria get the same facts.

BlazeandCave's Advancements Pack uses these fields in about 120 advancements (every `*_milestone`,
`65_hours_of_walking`, `level_up`, `iron_man`…).

## Kind

- [x] parity: Horizon now does what vanilla 26.3 does
- [ ] bug: a crash or a breakage
- [ ] perf: faster, or less memory
- [ ] feature: something new for players (listed under New in the release notes)
- [ ] extra: Horizon-only, off by default
- [ ] tooling: tools, CI, tests, harnesses
- [ ] docs

## Vanilla reference

Expected from `PlayerPredicate.matches`:
- `level` is a `MinMaxBounds.Ints` on `experienceLevel`;
- each `StatMatcher` tests its bounds against `ServerStatsCounter.getValue` for the stat type's stat;
- each `recipes` entry checks whether the recipe book contains the recipe;
- each `advancements` entry is an `AdvancementDonePredicate` or `AdvancementCriterionsPredicate`, tested against
  `PlayerAdvancements.getOrStartProgress`, and an id that isn't loaded fails.

Expected from `InventoryChangeTrigger.TriggerInstance`: its `player` `ContextAwarePredicate` is tested by
`SimpleCriterionTrigger.trigger`.

None of this was checked against the jar from this session.

No harness: none compares advancement player predicates. The unit tests in `advancements.rs` cover each field
with near misses, plus the trigger index.

## Other projects' code

none

## Self-check

`python3 tools/pr_ready.py --quick --base upstream/main` could not run here (see REPORT). Ran instead:

- `cargo fmt -p horizon-client`: clean.
- `cargo test -p horizon-client-lib --lib advancements::`: 29 passed, 1 ignored (needs prepared resources).
  The 5 new tests are among them.
- `cargo clippy -p horizon-client -p horizon-client-lib --all-targets`: no warnings in the changed files. The only
  warnings are 6 pre-existing `manual !RangeInclusive::contains`, in `dragon.rs`, `camera.rs` and
  `aquifer/noise_based.rs`.
- `cargo test -p horizon-client --bins advancement`: see REPORT.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
