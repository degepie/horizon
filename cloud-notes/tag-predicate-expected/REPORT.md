# Report: `TagPredicate.expected` missing in a damage predicate

**Verdict: no code change.** Making Horizon reject this one malformed field would be the wrong shape of fix: it
would be the only required-field check in the whole advancement loader, and it fights the loader's design (it never
reads criteria conditions at load). No branch code was pushed, only this note.

advisor: used (once before deciding, once before finishing).

## Prior work

- `git log upstream/main`: nothing touches `expected` after #8662 (`02205a31e`, merged as `1294fca5e`).
- Open PRs: the GitHub API (REST and GraphQL) for horizon-rust/horizon is blocked in this session, so I could not
  list issues or read the review of #8662. Instead I fetched `refs/pull/*/head` over anonymous git and diffed every
  PR head newer than 2026-10-25 that is not in main, looking for `expected` lines in `advancements.rs` / `loot.rs`.
  The only hit, `pr/2475` ("Integration: rustfmt"), is an old integration branch that carries the old line. It is
  not a fix.
- Open issues: not checkable here (API blocked). The coordinator should search for "TagPredicate" or "expected".

## Vanilla (not checked: no jar in this session)

- Expected from `TagPredicate.codec`: `expected` is a `fieldOf`, so it is required, and so is `id`.
- Expected from `Criterion.CODEC`: it dispatches on `trigger` and decodes `conditions` with the trigger's codec
  when the advancement loads.
- Expected from `ServerAdvancementManager.apply` (a `SimpleJsonResourceReloadListener`): a file that fails to
  decode is logged and skipped, so the whole advancement is absent.
- Vanilla's own datagen always writes `expected`. Only hand-written data packs can leave it out.

## How Horizon treats required fields at load (consistency evidence)

`Tree::load_with` / `parse` (`crates/horizon-client/src/advancements.rs:106`, `:272`) drop an advancement only when:
- its JSON does not parse (`:121`, `:131`), or
- `criteria` is not an object (`:274`, the `?`).

Everything else that vanilla requires gets a default:
- a missing `trigger` becomes `""` (`:278-280`);
- display `icon` becomes `minecraft:air`, and a missing `title` or `description` becomes `null` (`:301-306`);
- criteria `conditions` are kept as `LazyJson` (`:281`) and are not parsed at all until a trigger fires. The reason
  is in `crates/horizon-world/src/lazy_json.rs:1-4`: parsed `Value`s took about 10× the JSON's size, most of it
  never read.

So Horizon deliberately does not decode conditions at load. Matching vanilla here means decoding every trigger's
conditions with a schema, which is a separate, large project. Checking `expected` alone would still leave its
sibling `id` and every other required field unchecked.

## Options considered

1. **Runtime: a missing `expected` fails the tag check.** This is not vanilla either. The advancement would still
   exist, show in the screen and be grantable with `/advancement`. It only swaps one kind of leniency for another.
2. **Load-time targeted scan:** walk the `tags` arrays at every place a damage predicate can sit
   (`entity_hurt_player.damage.type`, `player_hurt_entity.damage.type`, `entity_killed_player.killing_blow`,
   `player_killed_entity.killing_blow`, plus nested or `reference` predicates) and drop the advancement. This
   reads conditions at load, against the LazyJson design, and checks one field out of hundreds.
3. **Load-time full decode:** this is the only option that really matches vanilla, and it is out of scope.

The maintainer already called the default "more lenient, but harmless" (in the review of #8662, as quoted in the
task; I could not fetch it).

## Finding worth a separate look: the two Horizon damage-source predicates disagree

The same vanilla `DamageSourcePredicate` is evaluated in two places, and their defaults for a malformed tag entry
are not the same:

| Field missing | advancements (`advancements.rs:934`) | loot `damage_source_properties` (`crates/horizon-world/src/loot.rs:1342-1347`) |
|---|---|---|
| `expected` | treated as `true` | treated as `false` |
| `id` | `id_matches(Null)` matches any damage type (`advancements.rs:541`) | `""` matches nothing |

The loot side also builds its own tag test instead of reusing `id_matches` (a list or `HolderSet` `id` is not
handled there). Vanilla has no answer for which default is right, because it rejects both files. Aligning them
is a pure Horizon-consistency cleanup with no vanilla effect on valid data, so I left it out of this task. If the
coordinator wants it, the smallest version is to make loot's `expected` default `true`, like advancements and #8662.

## Not verified

- The vanilla behaviour above (no jar, no `rea_vanilla.py`, no game here).
- Open upstream issues, and the exact text of the #8662 review.
