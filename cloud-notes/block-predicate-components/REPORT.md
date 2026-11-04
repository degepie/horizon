# block-predicate-components: report

advisor: called once before writing the code; it was rate-limited and gave no advice.

## Prior work
- No upstream PR touches this. I fetched the heads of every upstream PR numbered ≥ 9150
  (175 refs) and diffed each against upstream/main in `crates/horizon-client/src/advancements.rs`.
  Only #9405 (degepie, `fix/dp-block-nbt`, `nbt`) matched. `git log upstream/main` has no fix.
- I could not list upstream issues or comments: the GitHub API is not enabled for
  horizon-rust/horizon in this session, only anonymous git reads. Before publishing
  ISSUE.md, check for an existing issue or `/claim`.

## Root cause
`adv::block` (`crates/horizon-client/src/advancements.rs`) read only `blocks` and `state`.

## Fix
- `World::block_entity_components(pos) -> Option<Compound>` (default `None`), placed
  after `World::block`.
- In `block()`, after the state check: when `components` or `predicates` is set, a
  missing block entity fails, otherwise `component_matchers` runs. The new helpers
  are `component_matchers`, `collection`, `nbt_item_view` and `json_equals_nbt`.
- Game `View` (`game/advancements.rs`): `saved_block_entity` → `collected_components`.
  `saved_block_entity` in `game/block_items.rs` becomes `pub(super)`.

## Verified
- 3 new unit tests (exact, partial, no block entity), including near misses: another
  name, another count, a partial value under exact, a component that is absent, a wrong
  custom_data value, a wrong container size. All 27 `advancements::tests` pass.
- pr_ready --quick: Python 644 ✅, Rust tests 2677 ✅, rustfmt ✅. Clippy ❌ only on a lint
  already in `horizon-world` (aquifer `manual_range_contains`) under the local Rust 1.97;
  CI uses 1.99. Clippy shows no warnings in the changed files.

## Not verified (cloud)
- No vanilla jar here, so every vanilla claim is "expected from". The main assumption:
  `matchesBlockEntity` reads `collectComponents()`, not just the block entity's own
  leftover `components()`. It also assumes an exact compare equals the codec-decoded
  value; the JSON↔NBT equality is my approximation of that.
- No in-game check. For a quick one, use the data pack in ISSUE.md.
- Partial predicate types other than `container` and `custom_data` return false:
  `damage`, `enchantments`, `jukebox_playable`, etc. They are only wrong if a block
  entity carries that component as a leftover.
- Decorated pots and comparators are not in `saved_block_entity` (#9405 uses
  `pot_data`/`comparator_data` for them), so their components are `None` here.
  Once #9405 lands, consider building the components from its `block_entity_nbt` instead.

## Overlap with #9405
- The same function, `adv::block`: both add a check just before the final `true`. This
  is one textual conflict on merge; keep both, `nbt` first, then components (vanilla order).
- `World` trait and `View` impl: different methods at different places, so they merge
  cleanly (checked with `git merge-tree`).
- Tests: #9405 already conflicts with upstream/main in `advancements::tests` (it
  predates the weather test). My tests sit at the end of the module and add no
  conflict there.
- After both land, the two methods could become one block-entity lookup. That is not
  needed for correctness.
