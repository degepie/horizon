# minecart-display-offsets: report for the coordinator

## Prior work (checked on 2026-11-05, base upstream/main 1f90d6f9a)
- Searched upstream issues and PRs through the REST search API for `DisplayOffset`,
  `getDefaultDisplayOffset`, "minecart offset" and "chest minecart". `gh issue view` and `gh pr list` don't
  work from this session: the GraphQL API is blocked and upstream isn't attached for REST.
- Related, all closed: #741 / PR #772 (the hopper cart's offset 1), #8050 / PR #8098 (DisplayState and
  DisplayOffset saved), PR #9300 (synced to clients). None of them handles the chest cart's 8.
- No open issue or PR fixes this. Open PR #8941 ("Perf: Optimize entity occlusion before model generation")
  also edits `minecarts.rs`: it adds `cart_render_bounds`, which takes `display_offset(c)` as an input, so it
  picks up this fix without changes. The diffs don't overlap (its hunks are below `display_offset` and in the
  render loop). Expect at most a trivial rebase.

## Values: which kinds override `getDefaultDisplayOffset` (to confirm in the jar)
| Kind | Class | Value | Source |
|---|---|---|---|
| all (base) | `AbstractMinecart` | 6 | you confirmed it |
| chest | `MinecartChest` | 8 | you confirmed it |
| hopper | `MinecartHopper` | 1 | you confirmed it |
| furnace | `MinecartFurnace` | 6 (no override) | to confirm |
| TNT | `MinecartTNT` | 6 (no override) | to confirm |
| command block | `MinecartCommandBlock` | 6 (no override) | to confirm |
| spawner | `MinecartSpawner` | 6 (no override) | to confirm |
| plain | `Minecart` | 6 (no override) | to confirm |

As I remember it, the other subclasses override only `getDefaultDisplayBlockState`, not the offset. I
couldn't check that here (no jar). If `AbstractMinecartContainer` overrides it, the hopper's and chest's own
overrides still win.

## Change
- `default_display_offset` is an exhaustive `match`: Chest 8, Hopper 1, the rest 6.
- Callers, all through this one function, now agree:
  - `display_offset`: the draw translation (`minecarts.rs`, the `(offset - 8) / 16` translation) and the
    cull box's height.
  - `Minecart::to_nbt`: `DisplayOffset` is written only off the kind's default.
  - `lan_entities.rs` `cart_display_data`: the entry 12 default, and the value sent only off the default.
  - The client of a server (`remote_objects.rs:282`) stores entry 12 when it arrives and otherwise falls back
    to the same per-kind default.
- Behaviour change in saves: a chest cart with `DisplayOffset:6` now saves the 6, and one with `8` no longer
  saves it. That is the intended parity.

## Verified
- `cargo test -p horizon-client --lib minecarts::`: 18 passed, including the updated
  `display_offset_matches_vanilla_for_each_cart_kind` and `display_offset_is_not_saved_at_its_default`.
- `cargo fmt`. clippy and the touched crates' tests come from `pr_ready.py --quick` (in PR.md).

## Not verified (needs your machines)
- The values in the jar (table above).
- In game: the chest cart's chest should sit 0.09375 blocks higher than before. Compare with vanilla side by
  side, against a plain cart with `DisplayState:{Name:"minecraft:chest"}` (offset 6).
- `addAdditionalSaveData`'s "only when it differs from the default" rule is Horizon's existing reading; I
  didn't change it and can't check it here.

## Risks
- Low. Rendering moves the chest of every chest cart up by 0.09375 blocks. A chest cart's saved
  `DisplayOffset` key appears or disappears as described above.
