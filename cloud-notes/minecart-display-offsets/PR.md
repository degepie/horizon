> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

## What and why

A chest minecart draws its chest 2/16 (at the cart's 0.75 scale) too low. `default_display_offset` returned
the hopper cart's 1 or the base 6, and the chest cart's override (8) was missing. It now matches on every
`CartKind`, so a kind added later has to choose its offset.

The one function feeds every user, so they all change together: the drawn block and the cull box's height
(`display_offset`), the `DisplayOffset` save rule (written only off the kind's default), and the
`DATA_ID_DISPLAY_OFFSET` default and value sent to clients (`cart_display_data`, from #9300). A client of a
server uses the same per-kind default for a cart whose offset wasn't sent.

Tests: `display_offset_matches_vanilla_for_each_cart_kind` now lists all seven kinds and checks that a chest
cart with no saved offset draws at 8. `display_offset_is_not_saved_at_its_default` also covers the chest cart
(8 isn't saved, 6 is) and a plain cart.

Fixes #TBD

## Kind

- [x] parity: Horizon now does what vanilla 26.3 does
- [ ] bug: a crash or a breakage
- [ ] perf: faster, or less memory
- [ ] feature: something new for players (listed under New in the release notes)
- [ ] extra: Horizon-only, off by default
- [ ] tooling: tools, CI, tests, harnesses
- [ ] docs

## Vanilla reference

`AbstractMinecart.getDefaultDisplayOffset` (6), overridden by `MinecartChest.getDefaultDisplayOffset` (8)
and `MinecartHopper.getDefaultDisplayOffset` (1); `AbstractMinecart.getDisplayOffset`,
`addAdditionalSaveData`, `readAdditionalSaveData`, `defineSynchedData` (`DATA_ID_DISPLAY_OFFSET`).

No harness: no parity harness compares cart display offsets; the per-kind values are unit-tested.

## Other projects' code

none

## Self-check

**pr_ready** (`tools/pr_ready.py --quick --base upstream/main` on vm, against upstream/main at 1f90d6f9a, 10 min 52 s)

| Check | Result |
| --- | --- |
| Scope | ✅ 4 files, +165 −5 |
| Cargo.lock | ✅ unchanged |
| Public tree | ✅ |
| Python tests | ✅ 644 passed (1 min 40 s) |
| rustfmt | ✅ (18 s) |
| clippy | ✅ (5 min 45 s) |
| tests: horizon-client | ✅ 896 passed (4 min 46 s) |
| doc tests | ✅ 0 passed (3 s) |
| content root tests | ➖ content root untouched |
| tokens.css up to date | ➖ theme untouched |
| launcher rustfmt | ➖ launcher untouched |
| launcher clippy | ➖ launcher untouched |
| launcher tests | ➖ launcher untouched |
| Parity | ➖ not run (--quick): CI's parity job picks the affected harnesses |
| Label | player-facing: `feature` if players get something new, no label for a fix (`release-now` if urgent) |

Scope counts the three `cloud-notes/` files too; the code change is `minecarts.rs` only (+32 −5).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
