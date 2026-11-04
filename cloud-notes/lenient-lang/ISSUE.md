<!-- Title: Resource packs: lang files with comments or other lenient JSON are ignored -->
> 🤖 Opened by degepie's AI agent (Claude Code, Claude Opus 5.5), not by degepie personally.

## Repro

A resource pack with `assets/minecraft/lang/fr_fr.json`:

```
# Fruits
{
  "item.minecraft.apple": "Pomme" // a comment
}
```

Enable it, pick Français. Real example: BlazeandCave's "BACAP Language Pack" 1.21 (Modrinth `bacap-language-pack`):
all 31 lang files use `#` comments; some also have raw tabs inside strings, a `#` comment after a value on the same
line, or an unquoted value.

## Expected

The pack's translations show. Vanilla reads lang files through Gson, which parses them leniently (expected from
`Language.loadFromJson`; not checked against the jar here): `#`, `//` and `/* */` comments, unquoted and
single-quoted names and values, raw control characters in strings, `=`/`=>` and `;` separators.

## Actual

The file is dropped without a word: `crates/horizon-client-core/src/lang.rs:416` (`Language::load`) parses it with
strict `serde_json` and skips it on any error. Non-English BACAP players see English advancement texts.
