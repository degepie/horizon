# Lenient lang files (BACAP audit item 12)

## Prior work
- `gh api search/issues` on horizon-rust/horizon (PRs and issues: "lenient lang", "lang comments", "lang json",
  "lenient json", "BACAP"): nothing on this. Direct `repos/horizon-rust/horizon/...` calls were refused (403, repo
  not attached to the session), so open/merged PRs were searched, not listed. `git log upstream/main` on `lang.rs`:
  only translation unions. #8875 ("resource packs doesn't work well", open, unclaimed as far as the search body
  shows) is a vaguer report — maybe related, maybe not; no claim to it.
- #957 ("Carry: read data files with comments") is port-pack only, a separate comment stripper.

## Root cause / fix
`Language::load` (`lang.rs:416` before) used strict `serde_json` and silently dropped failures. New
`read_lang_file`: strict serde first (byte-identical behaviour for strict files), else UTF-8-lossy → strip BOM →
`horizon_world::datafix::json::parse_lenient` (existing Gson `JsonReader` port; horizon-client-core already depends
on horizon-world). Values per `GsonHelper.convertToString` (string/number/bool → text; null/array/object stops the
file at that entry, since loadFromJson consumes entries in order and throws there — expected, not checked).

Accepted syntax (that of Gson lenient `JsonReader`, as ported): `#`, `//`, `/* */` comments anywhere whitespace
is allowed (incl. after a value on the same line); unquoted and single-quoted names and values; raw control chars
in strings; unknown escapes; `=`/`=>` name separators; `;` member separators; case-insensitive true/false/null.
Still rejected (Gson rejects them too): trailing comma before `}`, missing comma between members, unterminated
objects/strings.

## Verified
- Unit tests (lang.rs): plain JSON == serde result (incl. duplicate keys last-wins); each comment form; unquoted
  and single-quoted; raw tab/newline; `=`/`=>`/`;` + BOM; trailing comma rejected; numbers/bools as text;
  non-primitive truncation; not-JSON files add nothing; a commented pack file layers over English via `load`.
- BACAP Language Pack 1.21 (Modrinth, sha256 63b82a42…a864, same as the audit) — all 31 files load via the
  `#[ignore]`d test `every_lang_file_of_a_pack_reads` (`HORIZON_LANG_PACK_DIR=<pack>/assets/minecraft/lang`).
  Every file's key count equals the number of unique `"key":` lines (Python regex), so nothing is truncated.
  The 6 "other syntax" files: de_de, tr_tr, rpr = raw control chars in strings; ko_kr, uk_ua = `#` comment after
  a value on the same line; zh_tw = one unquoted value followed by `""` (Gson's unquoted literal does not stop at
  `"`, so the value keeps a trailing `""`; expected to show the same in vanilla).
- cargo fmt, `cargo clippy -p horizon-client-core --all-targets --no-deps -D warnings` clean, `cargo test -p
  horizon-client-core` 71 pass. pr_ready --quick: all green except clippy, which fails on an untouched
  horizon-world lint (`manual_range_contains`, aquifer/noise_based.rs:476) flagged by the local Rust 1.97.

## Not verified / risks
- Not checked against the 26.3 jar (no jar here): that `Language.loadFromJson` really goes lenient (Gson
  `fromJson(Reader)` reads lenient unless the Gson instance is set strict), and the non-primitive/truncation
  behaviour. Coordinator: `rea_vanilla.py method net.minecraft.locale.Language loadFromJson`.
- No in-game check (no GPU/resources). Parity harnesses not run.
- Not covered: Gson's `)]}'` non-execute prefix (the port lacks it; no pack seen using it); `\uD800` lone
  surrogates become U+FFFD (Java keeps them). `deprecated.json` and `pack.mcmeta` language lists stay strict.
- Overlaps: only `lang.rs` `Language::load`; no open PR found touching it.
