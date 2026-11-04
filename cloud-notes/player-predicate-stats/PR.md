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

Commands test a predicate on a player in several places: `execute if predicate`, a selector's `predicate=`, item
modifiers and conditions. There, `facts_asked_in` scans the predicate and fills only what it names. A player who
is not in place (`Target::Guest`, which includes single player's own player in a tick function) reads their own
records from the hub.

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

**pr_ready** (`tools/pr_ready.py --quick --base upstream/main` on vm, against upstream/main at 63eccfc35, 1 min 46 s)

| Check | Result |
| --- | --- |
| Scope | ⚠️ 5 files, +625 −18; 531 lines of code: over 500, split it if it can ship in parts |
| Cargo.lock | ✅ unchanged |
| Public tree | ✅ |
| Python tests | ✅ 644 passed (1 min 23 s) |
| rustfmt | ✅ (16 s) |
| Rust | ⚠️ 1.97.0 is older than CI's stable (1.99): clippy may flag lints CI doesn't, update with `rustup update` |
| clippy | ❌ failed; likely the old Rust above (29 s) |
| tests: horizon-client, horizon-client-lib | ✅ 2682 passed (55 s) |
| doc tests | ✅ 0 passed (5 s) |
| content root tests | ➖ content root untouched |
| tokens.css up to date | ➖ theme untouched |
| launcher rustfmt | ➖ launcher untouched |
| launcher clippy | ➖ launcher untouched |
| launcher tests | ➖ launcher untouched |
| Parity | ➖ not run (--quick): CI's parity job picks the affected harnesses |
| Label | player-facing: `feature` if players get something new, no label for a fix (`release-now` if urgent) |

clippy fails here only on a lint that is already on main: `manual_range_contains`, in
`horizon-world/src/worldgen/aquifer/noise_based.rs:476`. This runner's Rust 1.97 flags it. The changed files have
no clippy warnings (`cargo clippy -p horizon-client -p horizon-client-lib --all-targets` without `-D warnings`).
New tests: `cargo test -p horizon-client-lib --lib advancements::` (29 passed). Gameplay advancement tests:
`cargo test -p horizon-client --lib advancement` (10 passed).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
