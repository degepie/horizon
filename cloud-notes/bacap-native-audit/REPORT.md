# BACAP on Horizon main, with no mod: audit

> 🤖 Written by degepie's AI agent (Claude Code), not by degepie personally.

- **Base:** fresh `upstream/main` @ `63eccfc355` (2026-11-04 21:57 +0100). Line numbers below are at that commit.
- **Pack audited:** Modrinth "BlazeandCave's Advancements Pack 1.21.1" (version_number `1.21.1`, published
  2026-09-16, game version 26.3, `pack.mcmeta` min/max_format `[121,0]`), zip sha256 `ce391fe4…71d7`. It holds 1333
  advancement files (1332 with a `criteria` key; one file has none), 58 predicates, 5436 `.mcfunction` and 85 tag files. Language pack: Modrinth
  "BACAP Language Pack" `1.21` (game 26.2, sha256 `63b82a42…a864`, 31 lang files). Neither is committed. Both
  are kept outside the repo.
- **GitHub copy:** not analysed. github.com was unreachable from this session. Modrinth was blocked at first, then
  reachable, so every count below comes from the Modrinth 1.21.1 zip.
- **Method:** I read the code only, with no build and no game run. Two scripts walked every advancement and
  predicate JSON and every function. A "count" below is the number of **advancements whose criteria use the
  feature**. It is not the number proven broken in game.
- **Open-PR state:** the GitHub API for horizon-rust/horizon was not reachable. "Open" below means a `refs/pull/N/merge`
  ref exists and the head is not in `upstream/main`.
- No jar was available. Vanilla behaviour is stated as "expected from `Class.method`".
- advisor: used before writing and before finishing.
- **Branch:** `cloud/bacap-native-audit`, from `upstream/main`, with this one file. The harness-named
  `claude/bacap-native-audit-27cuqw` was not touched.

## How Horizon fails, which decides the symptom

In `crates/horizon-client/src/advancements.rs`, an unhandled field can fail in one of two ways:

- **Blocked (never granted):**
  - an unknown entity sub-predicate key returns `false` (`entity`, :844);
  - an unknown loot-condition type returns `false` (`conditions`, :1053);
  - a trigger that nothing fires never grants.
- **False grant (granted to anyone):**
  - unread player fields (:817-831);
  - unread item `components`/`predicates` keys (`item`, :598-642);
  - unread block-predicate keys (`block`, :654-678).

`minecraft:inverted` flips both cases.

## Items

| # | Item | Status | Evidence | BACAP use | What a PR needs |
|---|---|---|---|---|---|
| 1 | Player `type_specific` `stats`, `advancements`, `recipes`, `level` | **MISSING** (false grant) | `advancements.rs:817-831`: the `type_specific/player` arm checks only `gamemode`, `looking_at` and `input`. `EntityView` (:484-515) has no stats, advancement progress, recipe book or XP level. | `advancements` in 49 advancements (2550 entries: every `bacap/*_milestone`, `advancement_legend`…). `stats` in 58 (`custom` 54, `killed` 2, `mined` 2: `65_hours_of_walking`, `supersonic`, `pig_training`…). `level` in 4 (`level_up`, `xp_grinder`…). `recipes` in 0. | Carry the player's stats, advancement progress (done, or per-criterion), known recipes and XP level in `EntityView` (fill them only when a criterion asks, like `may_look`). Match them as expected from `PlayerPredicate.matches` and its `StatMatcher` / `AdvancementPredicate`s. |
| 2a | `inventory_changed` ignores its `player` conditions | **MISSING** (false grant) | `game/advancements.rs:849-948` (`check_inventory_trigger`) tests `slots` and `items`, never `c["player"]`. | 11 (`story/obtain_armor` override, `iron_man`, `diamond_clad`, `living_like_kings`, `coordinated_flair`…) | Add `adv::conditions(&c["player"], …)` as expected from `InventoryChangeTrigger.TriggerInstance.matches` (through `ContextAwarePredicate`). |
| 2b | Re-run only on an item id or count change | **MISSING** (late or no grant) | `game/advancements.rs:858` hashes `(item, count)` only, so enchanting, renaming or dyeing a stack does not re-run the trigger. | 53 advancements whose `inventory_changed` items test components or predicates | Hash the whole stack (components included). Expected from `InventoryMenu`'s slot listener calling `InventoryChangeTrigger.trigger` on any slot change. |
| 3 | Item predicates over the whole stack | **MISSING** (false grant), plus **PR-OPEN #9496** for bare keys only | `advancements.rs:598-642`: `components` reads only `potion_contents` (a string) and `damage`; `predicates` reads only `enchantments` and `damage`. Other keys are ignored, so any stack of the right id passes. `ItemView` (:474-481) has no other components. `custom_name: bool` exists but `item()` never reads it. #9496 only lets the four handled keys be written bare. | predicates: `stored_enchantments` 16 (all `enchanting/*` book hunts), `potion_contents` 6, `jukebox_playable` 3, `trim` 2, `bundle_contents` 2, `fireworks` 1, `custom_data` 1. Exact components: `custom_name` 8, `dyed_color` 4, `potion_contents` (object form) 2, `base_color`, `banner_patterns`, `container`, `instrument`, `suspicious_stew_effects` 1 each. | Give `item()` the stack's real components, from `ItemStack` plus the prototype defaults. Apply `DataComponentExactPredicate` (`components`: equal to the prototype-merged value) and the `ItemSubPredicate`s by key. Expected from `ItemPredicate.test` → `DataComponentMatchers.test`. This is the largest single code change. |
| 4 | Mob variants visible to entity predicates | **PARTLY** | `mobs/finalize.rs:2227-2242` reports wolf, cat, frog, pig, cow, mooshroom, chicken, zombie_nautilus, horse and llama variants as components, and `entity()` matches them (:842 → `components`, :738-749). **Parrot is missing** (also axolotl, fox, rabbit, salmon, tropical fish, villager type…). | `parrot/variant` 1 (`biomes/birdkeeper`, 5 variants). The other 6 variants BACAP uses are covered. | Add `Species::Parrot => "minecraft:parrot/variant"` (and the other `*/variant` components) with the registry names, expected from `Parrot.get(DataComponents.PARROT_VARIANT)`. |
| 5 | Drinking a potion or ominous bottle: `used` stat and `consume_item` | **MISSING** | `game/potions.rs:770-814` (`finish_drinking`) and `raids.rs:1448` award neither. `item_use.rs:101-104` returns before `finish_consuming`, which is where food gets both (`consume.rs:52-53`). | 9 (`potion/tank`, `nocturnal`, `hyper_healing`, `the_invisible_player`…) | In `finish_drinking`, `award_stat("used", …)` + `trigger_item("minecraft:consume_item", stack)` (before the stack shrinks), expected from `Consumable.onConsume` / `ServerPlayer.completeUsingItem`. |
| 6 | Name tag on a mob fires `player_interacted_with_entity` | **MISSING** | `game/names.rs:20-44` (`name_mob`) fires no trigger. `creatures.rs:5503` returns right after it, before the triggers at :5545-5650. | 4 (`whats_up_doc`, `nerds_never_die`, `i_just_want_to_make_toast`, `nyan_sheep`) | Fire `trigger_item_entity("minecraft:player_interacted_with_entity", <stack before shrink>, &target)` on success. Expected from `ServerGamePacketListenerImpl.handleInteract`, which triggers after an interaction that consumed. |
| 7 | Entity sub-predicate keys without a namespace | **FIXED** | `advancements.rs:771` `match &*full(key)` (commit `02205a31e0`, branch `fix/bare-ids` = merged #8662). `conditions` types use `full()` too (:1003). | Bare keys BACAP uses: `entity_type` 2032 entries, `location` 557, `nbt` 473, `equipment` 235, `distance` 106, `vehicle` 86, `flags` 72, `effects` 42, `components` 37, `passenger` 21, `stepping_on` 13, `movement` 4, `entity_tags` 3. All resolve. (BACAP writes `entity_type`, never `type`.) | none |
| 8 | Entity `nbt` on projectiles (arrow `weapon`) | **MISSING** (blocked). Mobs/player: **PR-OPEN #9494** | Today `nbt` falls to `_ => false` (:844), so every criterion with an entity `nbt` is never granted. #9494 gives `nbt` to mobs (`mob_entity`) and the player only. Projectile views (killing_blow `direct_entity`) get none. | `{weapon:{id:"minecraft:bow"}}`: 3 (`shoot_arrow`, `the_mighty_hunter`, `multiclassed`). Mob/player `nbt`: 41 (`VillagerData` 261 entries, `Variant` 66, `carriedBlockState` 45, `MainGene` 20…). | After #9494: build the arrow's or trident's saved tag (`weapon`, `item`…) in its `DamageView.direct` view, expected from `NbtPredicate.getEntityTagToCompare` on `AbstractArrow.addAdditionalSaveData`. |
| 9 | `movement` sub-predicate | **MISSING** (blocked) | No `movement` arm: `_ => false` (:844). `EntityView` has no velocity or fall distance. | 2 (`hit_and_run`: minecart speed ≥ 8; `arrow_of_ultimate_obliteration`: speed ≥ 70 while `is_fall_flying`) | Add velocity (per second) and `fall_distance` to `EntityView`. Match `x/y/z/speed/horizontal_speed/vertical_speed/fall_distance` as expected from `MovementPredicate.matches`. |
| 10 | Data-pack tags in predicates | **PR-OPEN #9403** (item, block, entity_type, biome, damage_type). **MISSING** for structure tags. Mob behaviour: MISSING, but BACAP doesn't need it | `game/advancements.rs:218-235`: `entity_type` and `worldgen/biome` read `mob_data()`, loaded with `PackOverlay::NONE` (`mobs/data.rs:85-93`). `damage_type` reads `enchant.rs:183`, the vanilla folder only. `worldgen/structure` falls to `_ => false` for every tag, vanilla ones too. Item and block tags read `self.0.res.items()` (:219), the game's own tables. The world's packs' `Items` (`world_tables`, through `Game::items`, `game.rs:19858-19863`) are not used, so `#blazeandcave:` item and block tags are blocked too. #9403 switches that lookup to `self.0.items()` and adds a `pack_tags` map for the 3 other registries. | data-pack `#blazeandcave:` item/block tags (`axes`, `spears`, `master_*`…): 45 advancements; entity tags: 41; damage tags (`is_player_hit`, `spear`): 27; biome tags: 2; structure tags (`#blazeandcave:village`, `#minecraft:shipwreck`, `ruined_portal`…): 9 (`titanic`, `ahoy`, `oh_look_it_dings`, `raidin_master`…). Union of all the tag gaps: 109 advancements. BACAP adds no `minecraft:` behaviour tags (only function tags), so mob behaviour is not affected. | Merge #9403. Then add `worldgen/structure` (load_tags with the overlay, matching `World::structures` ids). Mob behaviour (`entity_has` in `goals.rs`, `boat_passengers.rs`…) would also need the overlay. The fork branch `cloud/pack-tags-mob-behaviour` exists, but I found no upstream PR. |
| 11 | BlockPredicate `nbt` / `components` | `nbt`: **PR-OPEN #9405**. `components`: **MISSING** | `advancements.rs:654-678` reads `blocks` and `state` only, so `nbt` and `components` are ignored (false grant). #9405 adds `nbt` only. | `nbt` 2 (`riddle_fourth_line`, `light_show`). `components`: 0 in BACAP. | Merge #9405. `components`/`predicates` on the block entity's components, expected from `BlockPredicate.matchesComponents` (fork branch `cloud/block-predicate-components`; no upstream PR seen). |
| 12 | BACAP language resource pack | **MISSING** | `horizon-client-core/src/lang.rs:416` parses each `lang/*.json` with strict `serde_json` and silently drops a file that fails. **All 31** BACAP lang files fail strict JSON: they use `#` line comments (275 in `fr_fr.json`). After stripping `#` lines, 6 still fail (raw control characters in strings, `de_de`, `tr_tr`, `rpr`; other syntax, `ko_kr`, `uk_ua`, `zh_tw`). The trailing-comma case does not occur in 1.21. Pack lang overlays are supported (`Language::load` layers every source, :395-420). English works without the pack: BACAP's titles use the English text as the `translate` key and `lang::get` falls back to the key (:553-559). | Every non-English player (≈3000-3460 keys per language) | A lenient reader equal to Gson's lenient `JsonReader` (`#` and `//` comments, `/* */`, unquoted or single-quoted names, raw control characters), expected from `Language.loadFromJson` (Gson). Stripping `#` alone fixes 25/31 files, not all of them. |
| 13 | Chat spam from function-run commands | **PARTLY** | Function lines run with `silent: true` (`game/functions.rs:464-468`), installed as `command_source` by `with_source` (`commands/execute.rs:215-228`). `feedback`, `feedback_unbroadcast` and `command_failure` honour it (`commands.rs:279-332`), so `scoreboard`, `execute store` and `advancement` feedback no longer prints. **Query commands still bypass it:** `time query …` (`commands.rs:2022, 2138, 2156, 2163`), `gamerule <rule>` query (:3271), `difficulty` query (:3297), `seed` (:1099) call `system_message` directly. | `blazeandcave:ten_second_timer.mcfunction:19` runs `execute store result score … run time query minecraft:day`, which reaches `time.query.timeline`. That prints a chat line on every timer run, provided `minecraft:day` is in `ui.timelines` (`game/chat.rs:232-246` reads them from the prepared root's `data/minecraft/timeline`, where I expect vanilla's `day` to be). If it is not there, the command fails silently instead, and `bac_current_time` is never set. | Route these through `feedback` (`sendSuccess(…, false)`), expected from `TimeCommand.queryTime` / `GameRuleCommand.queryRule` using `CommandSourceStack.sendSuccess`. |

## Other gaps found in BACAP

Counts are advancements, with entries in brackets. They were checked against the code above.

| Gap | Symptom | Count | Where in Horizon |
|---|---|---|---|
| **Trigger `minecraft:entity_killed_player` is never fired** | blocked | **17 advancements / 68 criteria** (`were_going_on_a_bear_hunt`, `kung_fu_panda`, `the_actual_end`…, plus alternative criteria of the `adventure` and `biomes` roots) | `trigger_killed_as` is called only for `player_killed_entity` and `kill_mob_near_sculk_catalyst` (`game/advancements.rs:1000`, `creatures.rs:1378`, `lan_actions.rs:4081`). Nothing calls it on player death. Expected from `LivingEntity.die` → `KilledTrigger` with the killer as entity. |
| Condition `minecraft:entity_scores` | blocked, or false grant under `inverted` | 4 (`unending_hell`, `miracle_drink`, `riddle_third_line`, `lead_zeppelin`) | `conditions` → `_ => false` (`advancements.rs:1053`). Needs the scoreboard in `World`. |
| Entity key `minecraft:entity_tags` (scoreboard tags) | blocked | 3 (`potion/gas`, `end/dimension_penetration`, `nether/this_ones_mine`) | no arm, `_ => false` (:844); `EntityView` has no tags |
| Entity key `type_specific/raider` | blocked | 3 (10 entries: `has_raid`, `is_captain`) | no arm in `advancements.rs` (`horizon-world/src/loot.rs:1237` has one for loot) |
| Flag `is_fall_flying` | ignored (false grant) | 1 | `flags` arm reads 7 flags, not this one (:778-787) |
| `type_specific/player.entity_type` | ignored | 1 entry (likely a pack typo) | — |
| Item predicate key `id` | ignored | 1 entry (pack typo for `items`) | — |
| `damage.direct_entity` (not a `DamagePredicate` field) | ignored | 1 entry | pack-side |

What is handled: all 54 triggers BACAP uses except `entity_killed_player` (`impossible` is meant to be fired by functions). Every condition type except `entity_scores`. `\` line continuations in functions (`horizon-client-render/src/datapack.rs:541-566`). `schedule`, function tags `minecraft:tick`/`load`, reward `function`.

Not checked:
- `execute if predicate` with BACAP's 58 predicate files (16 uses);
- the 7 `dialog` files;
- the `datapack` and `team` commands;
- the per-trigger condition fields (90 distinct `trigger.field` pairs in BACAP, e.g. `villager_trade.item`, `enchanted_item.levels`, `brewed_potion.potion`) against each trigger's handler. I only checked `damage`, `killing_blow`, `items`, `slots`, `recipes` and the entity, item and location predicates.

**In all:** 328 of the 1332 advancements have a criterion that uses at least one gap above. That figure leaves out:
- the global items: lang pack, the `time query` spam;
- the 70 advancements that only have `impossible` criteria and depend on `advancement grant` from functions.

## Verdict

The pack loads and runs, and its function machinery is close to vanilla: silencing, continuations, schedule and tick
all work. Per-advancement correctness is not there yet. The biggest problem is **false grants**:
- 111 advancements (item 1: milestones and statistics) are awarded as soon as their trigger fires;
- 58 more get false grants through items 2a and 3;
- 137 have a criterion that is **unreachable**: `entity_killed_player`, entity `nbt`, data-pack and structure tags,
  `entity_scores`, `entity_tags`, `raider`, `movement`. Some of these advancements are still reachable through
  their other criteria (the roots, for example).

So BACAP is not playable natively yet. It is about 5 PRs away from "mostly right".

PRs that would close the most advancements (ranked by advancements affected):

1. **PlayerPredicate `stats` / `advancements` / `level` (+ `recipes`)**: stops 111 false grants (item 1), plus 11 more from item 2a (a few lines in `inventory_changed`). This one has the highest impact.
2. **Merge #9403 + structure tags**: unblocks up to 109 (item/block 45, entity 41, damage 27, biome 2, structure 9; overlapping).
3. **Merge #9494 + projectile `nbt`**: unblocks 44 (41 mob/player + 3 arrow `weapon`).
4. **Fire `entity_killed_player`**: unblocks 17 (68 criteria). A small PR.
5. **Item predicates over the real stack** (`stored_enchantments`, `potion_contents`, exact `components`, …) **+ re-run `inventory_changed` on component changes**: stops about 50 false grants and fixes 53 late grants. Merge #9496 first, since it touches the same function.

Small follow-ups:
- `finish_drinking` stat and trigger (9);
- name tag trigger (4);
- parrot variant (1);
- `entity_scores` (4);
- `movement` (2);
- `entity_tags` / `raider` (6);
- query commands through `feedback` (stops the chat line);
- a lenient lang reader (needed by every non-English player).

## What I could not verify

- No in-game run, no vanilla jar and no harness: vanilla behaviour is "expected from".
- Advancement counts come from static JSON reads, not from playing.
- PR state comes from git refs, not the API.
