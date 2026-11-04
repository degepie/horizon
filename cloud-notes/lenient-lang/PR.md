> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

## What and why

Fixes #TBD

A resource pack's `lang/<code>.json` that is not strict JSON was dropped without a word: `Language::load` read it
with `serde_json` and skipped it on any error. Vanilla reads lang files through Gson's lenient reader, so packs
ship them with comments and other lenient syntax. BACAP's Language Pack is one: all 31 of its lang files failed
(`#` comments; after removing those, six still failed on raw tabs in strings, a comment after a value and an
unquoted value), so every non-English BACAP player saw English.

`read_lang_file` now reads each lang file strictly first, exactly as before, and only when that fails goes through
`horizon_world::datafix::json::parse_lenient`, the existing port of Gson's `JsonReader` in lenient mode (no new
parser). It also skips a leading byte order mark, as `JsonReader` does. Values follow
`GsonHelper.convertToString`: strings, numbers and booleans read as text; a null, array or object ends the file at
that entry. A file that is not JSON even to Gson still adds nothing.

One change for valid JSON: a lang file with a number, boolean or null value used to be dropped whole; now numbers
and booleans load as text and a null, array or object stops the file at that entry (deliberate, per
`convertToString`). Files whose values are all strings read exactly as before.

Checked against BACAP Language Pack 1.21 (not committed): all 31 files now load, each with as many keys as it has distinct
`"name":` lines (3000-3621); the `#[ignore]`d test `every_lang_file_of_a_pack_reads` checks that count.

## Kind

- [x] parity: Horizon now does what vanilla 26.3 does
- [ ] bug: a crash or a breakage
- [ ] perf: faster, or less memory
- [ ] feature: something new for players (listed under New in the release notes)
- [ ] extra: Horizon-only, off by default
- [ ] tooling: tools, CI, tests, harnesses
- [ ] docs

## Vanilla reference

Expected from `Language.loadFromJson` (Gson `fromJson` → lenient `JsonReader`) and `GsonHelper.convertToString`;
not checked against the 26.3 jar from this environment. The lenient grammar is the one
`horizon_world::datafix::json` already ports from `JsonReader`.

No harness: none checks lang loading (`tools/parity.py --changed-files` without dep-info records falls back to
416 of 417 harnesses, so it gives no narrower answer here); unit tests in `lang.rs` cover each syntax case and that a strict file reads as
`serde_json` reads it.

## Other projects' code

none

## Self-check

**pr_ready** (`tools/pr_ready.py --quick --base upstream/main` on vm, against upstream/main at 63eccfc35, 1 min 13 s)

| Check | Result |
| --- | --- |
| Scope | ✅ 1 files, +169 −3 |
| Cargo.lock | ✅ unchanged |
| Public tree | ✅ |
| Python tests | ✅ 644 passed (1 min 13 s) |
| rustfmt | ✅ (14 s) |
| Rust | ⚠️ 1.97.0 is older than CI's stable (1.99): clippy may flag lints CI doesn't, update with `rustup update` |
| clippy | ❌ failed; likely the old Rust above (30 s) |
| tests: horizon-client-core | ✅ 71 passed |
| doc tests | ✅ 0 passed (1 s) |
| content root tests | ➖ content root untouched |
| tokens.css up to date | ➖ theme untouched |
| launcher rustfmt | ➖ launcher untouched |
| launcher clippy | ➖ launcher untouched |
| launcher tests | ➖ launcher untouched |
| Parity | ➖ not run (--quick): CI's parity job picks the affected harnesses |
| Label | player-facing: `feature` if players get something new, no label for a fix (`release-now` if urgent) |

Clippy fails on a lint in `horizon-world/src/worldgen/aquifer/noise_based.rs:476` (untouched) that the local
1.97 toolchain flags; `cargo clippy -p horizon-client-core --all-targets --no-deps -- -D warnings` is clean.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
