# Report: item predicate component keys without a namespace

## Prior work

- I could not reach the GitHub API for horizon-rust/horizon from this session (`gh` and REST both return 403;
  only anonymous git reads work). Using `git ls-remote` I fetched the PR heads #9150–#9442: none of them
  changes the item-predicate code in `advancements.rs` (`components`, `potion_contents`, `minecraft:damage`,
  `minecraft:enchantments`). `git log upstream/main` has no later fix after #8662.
- **Not checked:** open/closed state and claims of older PRs (< #9150), and whether an issue already exists.
  Coordinator: please run `gh pr list` / `gh issue list` before you publish. ISSUE.md is ready in case no
  issue exists.

## Root cause

`fn item` (`crates/horizon-client/src/advancements.rs`) indexed `v["components"]["minecraft:potion_contents"]`,
`["minecraft:damage"]`, `v["predicates"]["minecraft:enchantments"]` and `["minecraft:damage"]` with the literal
full key. A bare key (`"potion_contents"`) found nothing, so the check was skipped and every stack passed.

## Fix

A `keyed(map, id)` helper finds the entry whose `full(key) == id`, and all four lookups use it. The
`potion_contents` value is now compared with `full()` on both sides, the same way the entity `components()`
check already does.

## Verified

- New unit test `advancements::tests::item_predicate_reads_bare_component_keys`: each of the four bare keys
  with a matching value (passes) and a non-matching value (fails), plus a bare potion value.
- I put the old lookups back and ran the test again: it **fails** (the non-matching case passes), so it
  detects the bug. Then I restored the fix.
- `cargo test -p horizon-client-lib --no-default-features --lib advancements::`: 25 passed, 1 ignored.
- `cargo fmt --all -- --check`: clean.
- `cargo clippy -p horizon-client-lib --all-targets -- -D warnings`: the local clippy 1.97 flags
  `clippy::manual_range_contains` in `horizon-world` (code this branch doesn't touch). CI uses the newest
  stable. With `-A clippy::manual_range_contains` it is clean.
- pr_ready --quick: everything green except clippy (only the old-toolchain lint above); table in PR.md.

## Not verified

- Vanilla: I had no jar in this session. The behaviour is "expected from" the `ItemPredicate` /
  `DataComponentExactPredicate` / `DataComponentPredicate` codecs (`Identifier.parse`, default namespace
  minecraft). No in-game check, no parity harness.
- I did not touch the object form of `potion_contents` (`{"potion": ...}`). Horizon reads only the string form,
  both before and after this change.
- Duplicate keys (`damage` and `minecraft:damage` together): vanilla is expected to reject the pack at load,
  while Horizon uses the first match.

## Same bug class elsewhere (not in this PR)

- `crates/horizon-world/src/loot.rs:1654,1658` (`match_tool` item predicate: `minecraft:map_id`,
  `minecraft:enchantments`)
- `crates/horizon-items/src/items.rs:1557` (`predicates.minecraft:enchantments`)
- `crates/horizon-port-pack/src/carry.rs:595`
- `crates/horizon-client/src/merchant.rs:721` and `crates/horizon-client-render/src/brewing.rs:228` read
  `components.minecraft:potion_contents` from Mojang's own data, which always uses full keys, so they are
  probably fine.

## Overlaps and risks

- Only `fn item` and a new private helper in `advancements.rs` change. No open PR from #9150 onward touches it.
- Risk is low: full-form keys behave as before. Bare keys used to match every stack and are now checked.

advisor: called before choosing the approach and again before finishing.
