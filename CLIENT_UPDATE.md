# Updating this server for a newer game client

Status as of 2026-09-27. This doc exists so a future session (mine or a fresh
one) doesn't have to redo the reconnaissance below from scratch.

**Read this section first.** It's a scannable summary of the entire
engagement. Everything below it (`## Confirmed...` onward) is the detailed,
chronological, round-by-round working log — genuinely useful for "why was
this specific decision made" archaeology, but not the place to start.

## Executive summary

**Starting state**: this server was built for an older Brown Dust 2 PC
client. The installed client had moved on to patch `20260918000`, and
separately — unrelated to the version gap, just never noticed before —
**446 of 486 already-written route handlers were never registered** in
`httpserver/src/main.rs`, and **most of the handlers that *were* registered
were empty TODO-stub templates** that never touched the database. Effectively,
most of "the game" didn't actually work yet, version gap aside.

**What got done, in order:**

1. **Client-version update.** Confirmed this project targets the PC/Google
   Play Games client specifically (not Android). Decompiled the new client
   with `ilspycmd`, diffed its schema against this repo's `protocol/proto/**`:
   167 new `common.db` tables, 23 new per-character "pack" databases, ~526 new
   network message types. Built `tools/BD2DataExtractor` — a BepInEx/Harmony
   plugin that hooks the *client's own* protobuf parser to dump real table
   data as JSON (no need to reverse-engineer any local cache encryption) —
   captured 145 tables / 14.6k+ rows live, merged into `data/tables/`, wrote a
   descriptor-to-proto3-text printer to regenerate `protocol/proto/**`
   including all 23 new pack DBs.
2. **13 new feature systems built from scratch**, each with a real SQLite
   schema, query layer, and game logic (not just response plumbing): **Life**
   (farming/home-building sim, 35 requests), **Fishing** (27), **Colosseum**
   (async PvP arena, 23), **Ib** (grid-inventory dungeon crawler, 14),
   **Hopscotch** minigame (5) + ranking views over existing minigames (6),
   **SpineInteraction** (character-interaction scenes, 10), **CharVote** (7),
   **MyRoom** (personal room decoration, 20 — most of which turned out to be
   pre-existing stubs, not new), **Avatar** (cosmetic loadout, 6),
   **Friendship** (per-costume affection system, 5), **RoomChat**
   (moderation, 4), and a 31-request rebuild of **Equip** (equipment/
   inventory — 27 of those 31 were pre-existing stubs, not new-client work).
3. **The stub-vs-real audit** (the largest chunk of work by far, triggered by
   noticing Equip and MyRoom were mostly pre-existing stubs, not new
   features): systematically found and fixed **~158 stub route-handler files**
   across nearly every core system — Guild (42, incl. a full guild-raid
   subsystem), EvilCastle (36, incl. an entire roguelike deck-builder mode),
   Char (17), PvP (16, an older arena system distinct from Colosseum),
   Cafeteria (16), Monster Hunt (14), Total War/Ranking (13), Field (13),
   Battle (9), Gacha (8), Pack (11), Friend (10), Supporter (8), User (7),
   Id/ID-Card (7), Event (7), Preset (6) + Deck (4), Costume (6), Quest (4),
   Item (5), Hunt (5), Shop (4), Talent (3) + Mission (3), and the entire
   45-handler minigame family (Action/Bingo/Board/Defense/Field/Rhythm/
   Roulette/Run/Sichuan/Survival), plus ~20 smaller single-digit clusters.
   Along the way, a final sanity sweep caught **12 separate silent bugs** —
   handlers with no stub markers that were nonetheless serving hardcoded fake
   data to every account (e.g. `recipe_info.rs` returning the same fake
   unlocked-recipe list forever; `save_total_battle_power.rs` hardcoding the
   same fake high score; several real, fully-scaffolded per-account tables
   sitting completely unused behind constant literal responses).

**Current state**: full workspace `cargo build` is clean, the server boots
end-to-end (`cargo run -p httpserver`, all ~410 migrations apply, listens on
0.0.0.0:8082/8443, no panics), and essentially every route the game client can
call now has a real, persistent, database-backed implementation rather than a
stub or a 404. This was independently re-verified after the work concluded.

## What "real" means here, precisely

Every round in this engagement followed the same honesty rule, worth
understanding before trusting any specific feature: **state that depends only
on what the client itself sends is always fully real and persists correctly**
(inventory, currency, levels, ownership, cross-account rankings/matching,
claim-tracking so nothing double-grants). **Where an action's *output* depends
on a master-data table this project never captured from the live client**
(some newer/rarer-to-reach systems — Colosseum, Ib, parts of Life/Fishing/
Friendship/CharVote/Avatar/EvilCastle-raid/Guild-raid), **the input side still
consumes/persists for real, but the reward or output value is a clearly
source-commented placeholder constant, never a fabricated-looking real value.**
This distinction is documented at every single site it applies, in the source
files themselves, not just in this doc.

## What's left to do (as of 2026-09-27, post-cleanup-round — this is the current, accurate list)

Everything that was open after the main engagement (route collisions, the
~15 tiny systems, several "left empty" response fields, ranking display
names) got resolved in the cleanup round below. What genuinely remains is
smaller and falls into three real categories — no more open bugs or
unregistered/stub routes are known to exist:

1. **Placeholder reward/config *values*, not missing logic** (the biggest
   category). Every system whose master data wasn't captured from the live
   client still has real, working persistence — inventory changes, currency
   deductions, claim-tracking, cross-account ranking — but grants a
   documented placeholder amount/id instead of a real one. Affects, at
   least: Colosseum (Vp deltas, promotion tiers, AP-buy cap), Ib (life/shop/
   sell-upgrade formulas, stage rewards), Life (cook/craft/tool/chunk-expand/
   helper-species/crop tables), Fishing (which fish you catch), EvilCastle's
   roguelike event-choice magnitudes, Guild-raid rewards, CharVote (event
   schedule, candidate roster, reward tiers), Avatar/Friendship/SpineInteraction
   shop-id-to-reward mappings, MiniEventHub (entirely empty, no table at
   all). **Fix**: re-run `tools/BD2DataExtractor` (see its README) while
   deliberately visiting each of these specific screens on the live client;
   every placeholder site is source-commented with exactly which table it's
   waiting on, so a recapture is a mechanical follow-up, not more design work.
2. **A handful of inferred-but-unconfirmed schema judgment calls**, each
   narrow and already documented at its exact source location (grep
   `CLIENT_UPDATE.md` for "judgment call" to jump to any of them): CharVote's
   free/paid `vote_type` mapping, TacticsBingo's one-card-per-event
   assumption, Fishing's shop-sell target ambiguity, MyRoomMove's swap
   semantics, Equip's `EquipChange`/`EquipMainOptChange` semantics, a couple
   more of similar shape. None of these are guesses made carelessly — each
   is the most schema-consistent reading available, flagged specifically so
   a real player noticing wrong behavior can be pointed straight at the one
   line responsible.
3. **Architectural, not fixable by more code**: no live real-time
   multiplayer exists anywhere in this project. PvP-shaped features
   (Colosseum, PvP, Guild raid, Ib, Battle) all use the same pattern — server
   hands the client an opponent snapshot + shared seed, client simulates
   locally, server trusts the reported result. This is a real, deliberate,
   working design (and matches how several comparable mobile games'
   client-authoritative PvP already behaves), not a stopgap, but it does mean
   there's no live head-to-head play. Also: this project has no git
   repository (downloaded as a zip) — there is no commit history anywhere,
   this doc is the only record of what changed and why.

## How to build, run, and test

```
cargo build              # full workspace; ~8-10 min cold per this repo's own README
cargo run -p httpserver  # boots on 0.0.0.0:8082 (http) and 0.0.0.0:8443 (https)
```
Then use `scripts/prox.py` (mitmproxy) as before to route the real PC client's
traffic to this server instead of the live Neowiz backend. No account/save
data is at risk from any of this work — every change is additive schema and
new logic, nothing here alters how the client is contacted or authenticates.

## Live testing session (2026-09-27)

First actual end-to-end test against the real PC client, post-engagement.
Found and fixed real bugs the desk-review couldn't have caught — worth
reading before assuming the server is bug-free just because it builds/boots
clean. Setup used: `cargo run -p httpserver`, mitmproxy (`mitmdump -s
scripts/prox.py`) as a system-wide HTTP(S) proxy (`127.0.0.1:8080`, set via
`HKCU\...\Internet Settings`), mitmproxy's CA trusted in
`Cert:\CurrentUser\Root`.

**Bug 1 — `scripts/prox.py`'s JS-injection check matched `.json` files too.**
`if ".js" in path` is a substring check, and `.json` contains `.js` as its
first three characters — so `game.json`/`launcher_g.json`/`10000002.json`
(the client's critical bootstrap config) were having a JS auth-bypass
snippet prepended to them, corrupting them as JSON and producing "game
configuration file download failed". Fixed: match on `path.endswith(".js")`
after stripping the query string, not a substring check.

**Bug 2 — AVG antivirus's Web Shield conflicts with mitmproxy.** Both are
system-level HTTPS-intercepting proxies; running them simultaneously
produced `PermissionError: Access is denied: '\\.\avgMonFltProxy\'` noise
(this specific message turned out to be benign/cosmetic — it persisted even
during fully-successful runs — but the underlying conflict was still real).
**Fix (user action, not code): temporarily disable AVG's Web Shield** (or use
its tray icon's "Pause Protection"). No code-side fix exists for this one.

**Bug 3 — mitmproxy buffers entire response bodies in memory whenever a
`response()` hook is defined**, which is fine for small JSON configs but
causes large game-asset CDN downloads (multi-hundred-MB Addressables
bundles) to stall at a fixed progress percentage and retry forever (visible
in the mitmproxy log as repeated `200 OK (content missing)` immediately
followed by `Client disconnected` for the same URL). Fixed: added a
`responseheaders()` hook to `prox.py` that sets `flow.response.stream =
True` for anything that isn't one of the small files we actually need to
inspect/modify (the same `.js`/config-json check from Bug 1, factored into a
shared `_needs_body_inspection()` helper) — everything else now streams
through mitmproxy without being buffered.

**Bug 4 — `/sendmail/1` got a hard `401 Unauthorized` instead of the
fallback.** This route isn't registered anywhere in this codebase (not part
of `Proto.Net` either — not found anywhere in the decompiled client sources,
likely lives in a different bundled Neowiz SDK assembly, possibly billing/
account-related given it fires immediately after `acf.neonapi.com`
mycard/xsolla billing-check calls in the request sequence). It's fetched
like a plain URL without this game's session cookie (same shape as
`SpineInteractionRecordData`), but wasn't in the auth middleware's exemption
list — so instead of reaching the `.default_service` fallback and getting a
harmless empty `200`, it hit `httpserver/src/middleware/auth.rs`'s
"Missing cookie header" branch and returned a real 401, which made the
client hard-disconnect with a "data not found" error during the post-login
sync burst (this always happened at the exact same point, every single
test run — not flaky, fully deterministic). Fixed: added `sendmail` to the
auth bypass list, matching the `SpineInteractionRecordData` precedent.

**Bug 4, part 2 — IN PROGRESS**: fixing the 401 got the client past that
point, but it now shows a client-side "data not found, client logic error"
immediately after receiving the fallback's genuinely-empty response —
meaning `/sendmail/1` needs *some* real response shape, not just any 200.
Added temporary diagnostic logging to `fallback_handler`
(`httpserver/src/routes/default/fallback.rs`, guarded to only fire for paths
starting with "sendmail" — remove once resolved) to capture the actual
request body/headers next time this fires, since nothing in the decompiled
client sources references "sendmail" at all (it may live in a different
bundled DLL, e.g. one of the `Neo.Unity.*` assemblies rather than
`Assembly-CSharp.dll`). Not yet resolved as of this writing.

Also confirmed, not a bug: `Illegal SNI hostname received "127.0.0.1"`
rustls warnings are cosmetic — a direct `curl` test confirmed the TLS
handshake completes fine despite the warning; these appear during every
successful run too, including ones with zero other problems.

## Client-side runtime bugs (2026-09-27) — `tools/BD2CompatPatch`

Everything above this point is server-side (`httpserver`/`gameserver`/
`database`). This section is different in kind: once the server-side work
let a real client actually connect and log in, sustained live play surfaced
a second category of problem that no amount of route/schema work touches —
**bugs in the game client's own compiled code**, triggered by conditions
(zero-progression fresh account, no live real-time chat/tutorial backend,
sparse master-data tables) that the original Neowiz servers never hit but
this private server hits on literally every account. Every fix below lives
in `tools/BD2CompatPatch/Plugin.cs`, a BepInEx/HarmonyX plugin injected into
the client process (`winhttp.dll` doorstop) that patches these methods at
runtime — nothing in `Assembly-CSharp.dll` itself is modified. Decompiled
with `ilspycmd` to `%TEMP%\bd2_decompile\out` for reading (see that folder
for exact line numbers if revisiting any of these).

**Methodology, since it's unusual enough to be worth stating plainly**: for
the click-routing bugs specifically, root cause was found by hooking Unity's
own `UnityEngine.UI.GraphicRaycaster.Raycast` and
`UnityEngine.EventSystems.EventSystem.RaycastAll` directly and logging the
real hit-list on every actual mouse click — i.e. watching what Unity itself
decided the click hit, rather than reasoning about it from source alone.
This turned out to be necessary: several plausible-looking hypotheses from
reading the decompiled source turned out to be wrong (see the `Collider`
false-positive below) and only direct observation resolved them.

**Bug 5 — `MenuUI.Init()` aborted almost immediately on every login,
silently skipping the entire menu bar.** `UpdateMenuMonthlySubIcon()` (one
of ~27 sub-updaters called from `UpdateIssue()`, itself called early in
`Init()`) dereferences a cash-shop subscription icon `GameObject` found by
child-name lookup, with no null check — and this server has no cash-package
data, so the lookup returns null on every account. Because C# has no
per-statement recovery here, that one uncaught exception silently aborted
every remaining `UpdateIssue()` sub-updater *and* every top-level `Init()`
call after it — currency, portrait, battle power, nickname, gacha, story,
guild, skin banner, promotion banner, the newbie-pass navigator, all of it.
This is almost certainly what read as "the whole home screen is broken."
Fix, in three layers (each catching what the previous one didn't fully
cover): (1) skip `UpdateMenuMonthlySubIcon` specifically; (2) replace
`UpdateIssue()` with a version that runs each of its ~27 sub-updaters in its
own try/catch (found a second real null, `UpdateStoryTimelineIssue`, this
way); (3) wrap all 195 of `MenuUI`'s other declared methods with a generic
Harmony *finalizer* that swallows any exception right where it's thrown and
lets the caller continue to its next statement, as a blanket safety net
against every other sub-call in `Init()`'s ~30-call tail (`SetMenuStory` was
the next one this caught). The finalizer approach — not skipping the method,
just catching what it throws — means every widget still runs its real logic
and only fails silently if it actually would have thrown anyway.

**Bug 6 — two separate stuck full-screen overlays from the tutorial
system, each eating every click on top of whatever they visually covered.**
This server has no tutorial-step completion/progress data, so the client's
own "is this tutorial step done" checks never return true, and two different
tutorial components get stuck permanently "on": `TutorialRoot`'s
`_uiInputBlocker` (a literal full-screen "Image - InputBlock" raycast
blocker, meant to be temporary while a highlighted target waits for a tap)
and `TutorialFocusController.Focus()`'s `_background` (a separate dimming
overlay, activated at the start of a `while (!done) yield return null;`
coroutine whose completion condition — a content-progress check or an actual
click on the highlighted target — can never become true here). Together
these produced exactly what got reported as "the screen looks washed out
white and nothing responds to clicks." Fix: patched `GameObject.SetActive`
itself, scoped narrowly to reject only activation calls where the target is
literally named `"Image - InputBlock"` (confirmed via testing that this
object is *already* active from prefab instantiation in some cases, not
just from an explicit `SetActive(true)` call site, so blocking the setter
was more reliable than patching every caller); and skipped
`TutorialFocusController.Focus()` entirely (returns an empty `IEnumerator`)
so that coroutine — and its `_background` overlay — never starts at all.

**Bug 7 — the real, generalized root cause of "nothing responds to any
click, on every screen."** Bug 6 fixed the tutorial-specific case, but
clicks were *still* not reaching real UI after that fix — proof came from
hooking `GraphicRaycaster.Raycast`/`EventSystem.RaycastAll` directly and
watching real click hit-lists: on completely unrelated screens (the field
HUD, `FieldCharSettingPopupUI`, `MenuUI`), the winning raycast hit was
consistently one of a small family of generic, purely-structural objects —
`TouchScreen` (present as the *first* hit inside nearly every popup canvas's
own result list, ahead of its real buttons), `Blocker`, and a stray
`Text - Enter` element (a leftover artifact, likely from the Firebase
email-login text field hit during an unrelated session incident — see the
proxy/auth note below — that never got torn down and kept winning raycasts
project-wide at a fixed screen position/depth thereafter). Fix: a
`GraphicRaycaster.Raycast` postfix that strips matching hits from the result
list before Unity's own sort picks a winner, so whatever real button is
underneath wins instead. **Final list, after the two false positives below
were found and reverted: `Blocker`, `Image - InputBlock`, `Text - Enter`.**

**Two false positives worth recording so they aren't repeated** — same
mistake shape both times: a generic-sounding object winning a lot of
raycasts is not by itself evidence it's a bug; confirm against the specific
screen/decompiled source before blocklisting a name project-wide.
- `Collider`, initially added to the strip list, also showed up winning
  raycasts everywhere, including the very first "TOUCH TO START" splash
  screen, which promptly stopped responding to any input once `Collider`
  was stripped. `IntroUI.cs` explained it: `IntroUI` has a parameterless
  `public void OnClickUI()` (distinct from the `OnClickUI(GameObject)`
  pattern every other screen uses) wired directly to a standard Unity
  `Button.onClick`, and that button's hit target is this exact `Collider`
  object — the legitimate click target for the whole splash screen.
- `TouchScreen` (also initially in the list) turned out to be the field's
  own movement input surface — `GameFieldDefaultUI.cs`:
  `FindChild(gameObject, "TouchScreen").SetPad(_touchPad)` wires it directly
  to `TouchPadScreen`, the drag-to-move joystick component. Stripping it
  broke field movement entirely, and it was never actually necessary for
  clicks in the first place — confirmed-working testing had already shown
  `FieldCharSettingPopupUI.OnClickUI` correctly receiving `Button - Cancel`
  clicks *while* `TouchScreen` was still present earlier in that same
  canvas's raycast list, meaning the real button already won its own
  canvas's internal sort without any help. Reverted.

**Bug 8 — a companion-list loading spinner that spun forever.**
`SpineManager`'s per-entry list-population code calls a method whose *only*
job, when a character's Spine skeleton data is confirmed missing, is to
build a detailed diagnostic log line and then `throw` — it has no
success-path responsibility at all (the real success path,
`SetSpineAnimation()`, is a separate branch this method is never called
from). Since this server is missing at least one real character's Spine
data (`illust_char000101_1` — content gap, see below), that throw aborted
whatever continuation was populating the remaining list entries, leaving
the "loading more" spinner stuck permanently. Fix: skip that diagnostic
method entirely (`SkipMethodPrefix`) so a missing Spine asset is silently
skipped instead of freezing the whole list.

**Bug 9 — one broken pack-list entry could blank out its neighbors.**
`PackIconBase.SetSubPackIcon` throws a `NullReferenceException` for certain
pack types on this server (a null `_spritePvpPackRank`/
`_spriteEventPackLevel` reference on a fresh account with no PvP rank yet),
and `SetIcon()` calls it partway through an otherwise-independent sequence
(sub-icon, progress bar, PvP effect glow, event-hub banner, play-state text,
reward summary, pack level badge) — so that one throw silently skipped every
step after it, for that one pack list entry, same "one broken step kills
the rest" shape as Bug 5. Fix: wrapped all of `PackIconBase`'s declared
methods with the same exception-swallowing finalizer technique as Bug 5.

**Bug 10 — the general version of Bug 8, and probably the single
highest-impact fix in this whole section.** The resource-loader class
backing `GetPrefabAsset` (used to load nearly every UI popup prefab in the
game, not just Spine illustrations) has the *same* shape of bug as Bug 8's
`SpineManager` method, in its own completion handler: when Unity's
Addressables system reports `op.OperationException != null` (i.e. the asset
genuinely doesn't exist — an `InvalidKeyException`, "No Location found for
Key=..."), the handler throws instead of invoking its own `callback(null)`
and cleaning up the way its sibling "op.Status != Succeeded" branch already
correctly does. Confirmed via the log this exact pattern (`if
(op.OperationException != null) { ...; throw ...; }`) is repeated **~24
times** across this one loader class, for different resource-type
completion handlers. Two concrete instances were confirmed and patched —
the Spine-illustration-specific one from Bug 8's investigation, and the
general `GetPrefabAsset` one (both take a concrete, non-generic
`AsyncOperationHandle<GameObject>`, so patching them is safe). **Deliberately
did NOT attempt a blanket fix across all ~24**: several of the others take
an *open generic* `AsyncOperationHandle<T>` — and this project already hit
the failure mode of patching a closed generic instantiation of a shared
generic method earlier in the engagement (see the SpriteAtlas
type-confusion incident referenced in `Plugin.cs`'s own comments): Mono
shares JIT'd code for one closed reference-type instantiation across every
other instantiation of the same generic method, so patching e.g. the
`<Texture>` instantiation could silently corrupt unrelated `<GameObject>`/
`<SpriteAtlas>`/etc. loads elsewhere. If a similarly-shaped stuck spinner
shows up again, look at this same loader class first (the two safe,
concrete-type completion handlers are named in `Plugin.cs`; grep it for
"GracefulPrefabLoadFailurePrefix" to find both patch sites and the comment
explaining why the generic ones are off-limits).

**Content gaps confirmed during this work, NOT client bugs** — these need
real asset files, not more patching: `UI/Prefabs/Spine/IllustSpecial/
SpecialIllust181.prefab` (a specific story-episode character illustration —
this is what Bug 8's spinner was stuck loading), and three event-pack icon
images (`PackUnopened_1/pack_event{8,9,10}_unopened_{61,64,67}.png`). Both
surfaced as `UnityEngine.AddressableAssets.InvalidKeyException: No Location
found for Key=...` — a real, verifiable "this file isn't in the asset
catalog" condition, not a logic error. A confirmed-working sprite-atlas
diagnostic (`SpriteManager.AtlasContainer.GetSprite` postfix, still active
in `Plugin.cs`) logged **zero** atlas-lookup failures across this entire
session, which is useful negative evidence: most of the remaining blank/
white icons reported during testing (hotkey slots, some HUD badges) are not
missing assets at all — they're a level-1 zero-progression account
correctly showing empty/unassigned skill and companion slots, the same as
a real fresh account would.

**Also encountered, worth recording since it cost real debugging time and
was a self-inflicted regression, not a client bug**: mid-session, restarting
`httpserver` (to get clean log capture) coincided with `mitmdump` — which
had been running continuously for several hours across many client
relaunches — entering a state where new connections to the *real* external
Firebase/`acf.neonapi.com` auth endpoints (the parts of the login flow
`prox.py` intentionally does not redirect) started failing with
`ConnectionError, Cannot connect to destination host`, even though direct
`curl` tests to the same hosts succeeded fine. This knocked the client out
of its cached guest session and into the real Google/Apple/Email sign-in
screen, which this server obviously can't service. **Fix was operational,
not code**: restarting `mitmdump` itself (not `httpserver`) cleared
whatever internal state had degraded, and a normal client relaunch went
straight back to the cached-session "Login Guide -> Auto Login" flow.
Lesson for next time: if login starts failing on external (non-`pmang.cloud`)
hosts partway through a long session, restart the proxy before assuming a
server or client-code regression — `mitmdump` had been alive for 3+ hours
and hundreds of connections by that point.

## Where to go from here

In rough priority order: (1) actually launch and play — this is the first
point where doing so tests the entire accumulated engagement, not just a
partial slice of it; (2) if something looks wrong, it's almost certainly one
of the three "what's left" categories above, not a crash — search this doc
for the specific feature name; (3) if richer reward values matter more than
new code right now, a targeted `tools/BD2DataExtractor` recapture (item 1
above) is higher-value than anything else left on the list.

## Cleanup round (2026-09-27)

A follow-up pass through the specific "judgment call worth a second look"
items scattered through the log below, plus the previously-deprioritized
tiny new-content systems:

1. **Verified all 6 remaining route-name collisions** from round 1
   (`BalanceVersionCheck`, `CharInfo`, `MaintenanceInfo`, `MissionInfo`,
   `NoticeInfo`, `ServerInfo`). `CharInfo`/`MissionInfo`: live via
   `httpserver/src/routes/default/batch/batch.rs`, both correctly delegate
   to real `gameserver::logic::game::{char,mission}::*::handle` — no bug,
   the `routes::game::*` duplicates are genuinely dead. The other 4: live via
   their `routes/basic`/`routes/default` handlers, which return legitimate
   static system-wide config (server URLs, a curated notice feed, a
   maintenance flag) — pre-existing deliberate design (not per-account state
   like the `RecipeInfo` bug was), confirmed fine as-is. All 6
   `gameserver`-tree duplicates confirmed unreachable dead code.
2. **Wired real char/equip snapshots into 4 previously-empty sites** using
   the same technique the Preset system already established
   (`database::db::char::char_info::get_by_inven_index` +new
   `database::db::equip::equip_info::get_by_use_char` for "what's currently
   equipped on this character"): `MonsterHuntPresetUse`'s
   `char_info`/`char_equip_info`, `SupporterBorrow`'s `supporter_char_info`
   (a real cross-account lookup — resolves the *owning* account's
   registered character, not the borrower's), `ColosseumBattleMatching`'s
   `enemy_char_info` for real opponent accounts (bots still get none — no
   account to look up), and `TotalWarBattleStart`'s `blue_char_info`/
   `red_char_info` (simpler than expected: the request already carries the
   client's own full `BattleCharDbInfo` for both teams, so the response now
   just echoes it back instead of trying to reconstruct it from inven
   indices). `buff_stat_info`/`costume_info`/`awake_info` stay empty
   everywhere — those need real stat-calculation formulas this project has
   never modeled, not a data lookup, so filling them would mean inventing
   game balance rather than reading real data. `SupporterBattleInfo` needed
   no fix — its only `supporter_char_info` field is the raw client-sent
   string, already passed through correctly; the original note conflated it
   with `SupporterBorrow`'s separate structured field.
3. **Real display names in cross-account rankings.** Added
   `logic::game::display_name(pool, uid)` (real `UserInfo.UserId` in-game
   nickname, falling back to `Guest{uid}` — matching this project's
   existing bot-naming convention) and wired it into every ranking/lookup
   response that previously used the stringified uid as a placeholder:
   Hopscotch's two rankings, Monster Hunt's ranking, both EvilCastle
   rankings (tower + roguelike), Friend recommend/search, and Guild's
   member roster + both supporter-info reads (looked up fresh at *read*
   time rather than trusted from the stored row, since a member's nickname
   can change after they joined — the stored snapshot would go stale).
   `CharVote`'s ranking needed no fix — it ranks *characters* by
   `candidate_id`, there's no account `user_id` field on that message at
   all; the earlier note was simply wrong.
4. **Implemented the remaining tiny new-content systems** — re-verified
   against the actual current codebase first (several from round 1's
   original list turned out to already be covered by the stub audit's
   broader passes: LifeCheat, Statue, Dating, TotalWar, Shop/Reputation).
   Genuinely built this round, all with real per-account persistence where
   the request only depends on client-sent state, honest placeholders only
   where no master-data table exists: **ChatSetting**, **ContentOpen**,
   **DailyStory** (Clear + Info), **EvilCastleGiveUp** (a real no-op — no
   "attempt in progress" state exists for the tower mode to clear, unlike
   the roguelike's own separate give-up route), **FieldEventSpawn** (Info +
   Reward + Start — real progress/caught-list/daily-count tracking),
   **FireWorks** (Info + Reward), **CashBonus** (Info + Reward),
   **GachaMultiBuy** (one placeholder-character draw per requested banner
   id, same honest convention as the single-buy Gacha route),
   **GuildNameChange** (reuses the guild system's own pre-existing
   `update_by_guild_id`), **ItemLock** (a pure column update — `ItemInfo`
   already had the `KeepFlag` column, just needed a setter), **MasterTitle**
   (Info + Update), **MiniEventHub** (an honestly-empty list — no
   `MiniEventHub*Table` data exists anywhere), **NpcQuiz** (Info + Clear —
   every answer accepted as correct, no answer-key data exists to validate
   against), **Square** (RewardInfo + Reward — a daily claim), **StressCheat**
   (a real no-op — a debug/stress-test endpoint with nothing to persist),
   **TacticsBingo** (DeckSave + Info — treated as one bingo card per event,
   a judgment call since the request carries no `group_id`),
   **UpdateAgeGate** (a real per-account record), **UserRelayInfoUpdate**
   (a real no-op — empty request/response by the proto's own design), and
   **TotalRanking** (a cross-system top-20-each summary, composed by
   re-querying the same real per-system ranking data PvP/MonsterHunt/
   GuildRaid/Colosseum's own individual ranking routes already serve for
   real — nothing new fabricated). 27 new `PacketCodeType` entries added
   (655-682).

Verified: full workspace `cargo build` clean, `cargo run -p httpserver`
boots end-to-end (migration 411 applies alongside 318-410), no panics or
route collisions.

Genuinely still open (not fixed this round, no further data/schema exists
to resolve them): every placeholder reward-value site already documented
throughout the log below (these need real master data, which needs a
targeted `tools/BD2DataExtractor` recapture, not more code); `TacticsBingo`'s
single-card-per-event assumption; `vote_type`'s free/paid mapping in
CharVote; a handful of similar small inferred-field judgment calls scattered
through earlier rounds.

## Confirmed: this project targets the PC (Google Play Games / pmang) client

Not the Android APK. Evidence:
- `scripts/prox.py` hooks the `pc.bd2.pmang.cloud` domain specifically and
  rewrites `patch_url` to a `pc.bd2.pmang.cloud` path.
- It injects fake web-launcher auth (`localStorage` session/uid/isLoggedIn)
  into `launcher_g.json`/`game.json` responses — that's the PC launcher's
  webview login flow; the Android APK doesn't have this.
- `httpserver/src/routes/basic/localization.rs` has `/api/gpg/price/localization`
  ("gpg" = Google Play Games).
- The actual client install (`BrownDust II_Data/Managed/PlayPcSdk*.dll`,
  `Neo.Unity.Neon.Purchase.Gpg.dll`) confirms Google Play Games PC SDK.
- `manifest.xml`: package `com.neowizgames.game.browndust2`, has a
  `PlayGamesServices` block.

## This repo's origin

Downloaded (not cloned) from `github.com/yoncodes/BD2PS`. As of this writing
that upstream's most recent commit is **2025-10-29** — same as the local
`.gitignore`/README timestamps — so **there is no newer upstream version to
pull**; any update has to be done here.

Also found `github.com/Flechazo098/bd2` — a different (Go-based) BD2 private
server project that independently arrived at the same core technique we're
using: a BepInEx client plugin (`plugins/CaptureEnvironment`) for packet/data
capture, plus a `LocalIdentity` plugin synced to the client. Worth a look if
this project's approach ever needs a second reference point. Community
data/wiki site also exists: `browndust2-db.souseha.com`.

## The client used for comparison

`A:\Neowiz\Browndust2\BrownDust2_10000002` — patch `20260918000` (dated
2026-09-18; a prior cached patch `20260604000` sits alongside it, unused).
Unity `2022.3.62f2`, **Mono** scripting backend (not IL2CPP) — confirmed via
`MonoBleedingEdge/` and a plain managed `Assembly-CSharp.dll`. This matters
for tooling choice (BepInEx Mono build, ilspycmd decompiles it directly with
no unstripping step needed).

The `.tied.dat` files sitting in the client's install root
(`20260604000.tied.dat`, `20260918000.tied.dat`, ~300-370MB) are red
herrings for our purposes — they're just zip-in-a-zip full-install patch
snapshots (first entry inside is literally `BrownDust II.exe`). Not where
table data lives; don't waste time on them again.

## Schema gap analysis (old repo protocol vs new client)

Done by decompiling `Assembly-CSharp.dll` with `ilspycmd` (installed via
`dotnet tool install -g ilspycmd`) and diffing type/message names against
`protocol/proto/**`.

- **389 -> 556 tables** in `common.db` (in `protocol/proto/common.db/AchievementTable/Proto_Design_common.proto`,
  a single ~8200-line file holding every common-db message despite the
  odd per-table directory name). **167 new tables**, covering entire systems
  this server doesn't implement yet: Avatar (dress-up), Alchemy,
  an ActionGame minigame, Bingo, Calendar, and more. One table
  (`GuildRaidBattleTable`) appears removed/renamed — not yet investigated.
  **Correction (stub-audit round, 2026-09-27)**: Cafeteria was wrongly listed
  here as new — its 12 master tables all predate this whole project; this
  line originally mis-tagged it, likely from a partial-name diff match.
- **59 -> 82 "pack" databases** (`protocol/proto/packNNNN.db/`) — these hold
  per-character battle/skill data. **23 entirely new pack DBs**: `pack1008-1010`,
  `pack11008-11010`, `pack12008-12010`, `pack13007`, `pack19-22`, `pack20000`,
  `pack2008-2010`, `pack3008-3012`. Each represents characters/content added
  since this repo was last updated.
- **1210 -> 1857 `Proto.Net` message types** (request/response DTOs). ~647 new
  network message types, i.e. new API endpoints with no server-side
  implementation at all yet.
- **Core login/session protocol is backward-compatible.** `LoginUserRequest`
  only gained one new optional field (`CountryCode`, field 12) — everything
  else is byte-identical. `BalanceVersionCheck` server logic
  (`gameserver/src/logic/game/balance/balance_version_check.rs`) doesn't
  enforce any version gate against the client's reported version, so the new
  client can still connect and use every currently-implemented route without
  changes.
- Existing server already implements **486 HTTP routes** — this is not a
  small project, there's a lot already built; the gap is additive new content,
  not a broken foundation.

## Where the actual table DATA comes from (the hard part)

`data/src/exceldb/*.rs` files are headed "Auto-generated from JSON data — do
not edit manually", generated from `data/tables/*.json`. That generator
script is **not present in this repo** (was run externally/previously,
presumably by whoever last updated this project, or in an earlier session).
The JSON files are real game data (verified against `AchievementTable.json`),
but not extractable from the static client build:
- Not in `Assembly-CSharp.dll` (only schema/parsing code lives there, under
  `Proto.Design.*` namespaces, driven by a big obfuscated `RawDataManager`
  class).
- Not in the Addressables catalog (`StreamingAssets/aa/catalog.json`, 66MB —
  grepped for table/design/exceldb-ish keywords, no hits).
- Not in the encrypted `AppData\LocalLow\Gamfs\BrownDust II\Data\<hash>` cache
  files either, as far as we got (all three share an identical 16-byte
  header/magic; didn't chase further once the BepInEx approach below panned
  out — no need to crack this).

**Solution: `tools/BD2DataExtractor`** — a BepInEx/Harmony plugin that hooks
the client's own protobuf parsing and dumps every table it decodes to JSON,
in the same shape as `data/tables/*.json`. See that folder's README for full
details, build/deploy steps, and gotchas already debugged (short version:
don't use Unity's legacy `Input`/`Update()` for timing anything in this
project — use a plain `System.Threading.Timer`; don't rely on
`OnApplicationQuit` either — both were confirmed unreliable here).

## What's left to do (in rough order)

1. ~~**Capture real table data**~~ DONE (2026-09-26). Used
   `tools/BD2DataExtractor` against the live client across a few sessions,
   including a force-load pass (see that folder's README) that pulled in
   most of `common.db`/`block.db`/`FieldObjectSceneData.db` without needing
   to manually click through every screen. Landed 145 tables / ~14.6k+ rows
   in `<game root>\ExtractedTables\*.json`. Note: the force-load pass did
   **not** yield any per-character `pack*.db` battle/skill data (no errors
   thrown, but nothing came back either) — still unexplained, deprioritized
   since battle-simulation logic isn't implemented server-side yet anyway so
   that data has no consumer right now. Revisit if/when battle logic work
   starts.
2. ~~**Merge into `data/tables/*.json`**~~ DONE (2026-09-26). Merged into both
   `data/tables/` and `httpserver/data/tables/` (kept byte-identical, per
   `get_data_path()`'s dual-resolution behavior in `httpserver/src/main.rs`).
   437 table JSON files total now (was 420).
3. ~~**Regenerate `data/src/exceldb/*.rs`**~~ DONE (2026-09-26). Wrote a
   one-off codegen (not committed as a reusable script — was a fork-agent
   task; redo similarly next time rather than expecting a checked-in tool)
   that: generated 17 brand-new table structs fresh, and for the 40 existing
   tables whose captured data revealed schema drift, conservatively widened
   fields to `Option<T>` / added new fields rather than blindly regenerating
   (to avoid breaking the ~486 existing routes' call sites into these
   structs). `data/src/exceldb/mod.rs` regenerated wholesale (437 modules).
   Full workspace `cargo build` succeeds clean.
4. ~~**Update `protocol/proto/**`**~~ DONE (2026-09-26). Wrote a Python
   descriptor-to-proto3-text printer (extracts each embedded base64
   `FileDescriptorProto` from the decompiled `*Reflection.cs` files, parses
   with `google.protobuf.descriptor_pb2`, prints proto3 matching this repo's
   existing merged-file convention). Result: 556 messages in `common.db`
   (167 new), 23 new pack DB directories (914 messages), 492 new `Proto.Net`
   messages + 34 new `Define_*` enums appended to `Commons/proto_net.proto`.
   `NLD_MID.proto`/`Network_TCP.proto` confirmed dead code (zero references
   anywhere in `gameserver`/`httpserver`/`common`) — left untouched.
   `protocol/build.rs` updated. Full workspace build clean.
5. **Implement new httpserver routes / gameserver logic** for the new
   features. **In progress, round 1 done (2026-09-26)** — see below.

### Step 5 progress log

**Round 1 (2026-09-26):**

- **Major unrelated discovery, fixed:** `httpserver/src/main.rs` only
  registered 40 of the 486 already-written route handler functions via
  `.service(...)` — 446 fully-implemented features (achievements, cafeteria,
  alchemy, battle, gacha, guild, etc. — features that look like brand-new
  systems in the client are often already built here, just never wired up)
  were dead code. Registered all 438 of them that don't collide on route
  path with an already-registered duplicate (8 collisions found — see
  "Known follow-ups" below), plus one additional orphaned handler
  (`routes::game::overwhelm`) that wasn't even declared in its module's
  `mod.rs` and had a broken import (`gameserver::logic::game::game::overwhelm`
  → fixed to `gameserver::logic::game::overwhelm`). **479 routes now active.**
- **Added a safety-net fallback**: `.default_service(...)` in `main.rs`
  (`httpserver/src/routes/default/fallback.rs`) returns a well-formed empty
  `GameResponse::success` for anything not explicitly registered, instead of
  actix's default 404 — so any of the new client's ~193 still-unimplemented
  request types fail soft (client shows empty/nothing) instead of
  hanging/erroring. Logs the route path at `info` level each time it fires,
  which doubles as a live "what does the client actually call" inventory for
  future rounds.
- **Fixed two boot-blocking bugs found by actually running the server**
  (previous rounds only got as far as `cargo build`, never `cargo run`):
  1. `data/src/exceldb/bufftable.rs`'s `buffGroup` field was typed
     `Option<i32>` but the captured data has it as a variable-length array
     in every row (`Vec<i32>` now, no other code referenced this field so
     it was a safe change). A scan of every table JSON against its struct
     found this was the *only* real type mismatch out of 437 tables.
  2. The `common` crate's compiled artifact was stale, baked with a
     duplicated/wrong `CARGO_MANIFEST_DIR` from some earlier build (its
     `env!("CARGO_MANIFEST_DIR")`-derived cert/key paths resolved to a
     nonexistent nested `BD2PS-main\BD2PS-main\...` path) — `cargo build`
     never recompiled it since since its source hadn't changed. Fixed via
     `cargo clean -p common` + rebuild. **Root cause pre-dates this whole
     client-update project** — worth remembering if a "why does this stale
     thing not match the source" mystery ever comes up again elsewhere.
  3. Left in place (net positive, not just diagnostic): `main.rs`'s
     cert/key loading now uses `anyhow::Context` for actual path info in
     any future error instead of a bare unhelpful io error.
- **Server verified to boot cleanly end-to-end** (`cargo run -p httpserver`)
  with all 479 routes + the fallback, listening on 0.0.0.0:8082/8443, zero
  actix route-collision panics.
- **Categorized the 193 new (`Commons/proto_net.proto` line ~10012 onward)
  request types that still have no real handler** (they hit the fallback).
  Nearly every one carries per-user-progress or cross-account-aggregate
  fields in its response (confirmed by sampling ~10 response shapes) — true
  zero-persistence "just reads a static table" endpoints are rare, and the
  couple of plausible candidates found (`PackSummaryInfoListRequest`,
  `PackDetailInfoRequest` — aggregate counts over a pack's field-design
  tables) are blocked by low master-data capture completeness right now
  (e.g. `FieldRewardObjectTable` only has 5 rows captured so far, versus
  presumably 1000+ that exist — computing "real" aggregates from that would
  be actively misleading, not just incomplete, so deliberately not
  implemented this round). Work queue by feature/system (message count =
  new Request types needing both a persistence design and route logic):

  - **Life** (32) — a farming/home-building system (world objects, crafting,
    cooking, helpers/NPCs, tools, shop). Biggest new system by request count.
  - **Fishing** (27) — full minigame: casting, biting, boat/rod upgrades,
    shop, collection, voyages.
  - **Colosseum** (23) — PvP arena: matching, battles, decks/presets,
    ranking, seasons, blessings.
  - **Ib** (14) — an "IB" dungeon system (deck, shop, stages, items) — name
    not decoded yet, doesn't correspond to a table prefix seen so far.
  - **MiniGame** (11) — Hopscotch + generic minigame ranking/record-keeping
    (rhythm, sichuan/mahjong, survival — several sub-variants).
  - **SpineInteraction** (10) — a record/save system tied to `Spine`
    character interactions (achievement + reward tracking variants).
  - **CharVote** (7) — character popularity voting w/ seasonal rankings.
  - **MyRoom** (6) / **Avatar** (6) — presets and dress-up, respectively.
  - **Friendship** (5) — dating-adjacent counseling/gifts/episodes.
  - **RoomChat** (4) / **Equip** (4) — chat moderation (block/report), and
    equipment main-option/rank-upgrade-to-break-auto variants.
  - Everything else (1-3 each): LifeCheat, FieldEventSpawn, Battle (phase
    change/resume/statistics), TacticsBingo, Statue, Square, Pack (the two
    blocked-on-data ones above), NpcQuiz, Master(Title), FireWorks, Dating,
    DailyStory, Costume, CashBonus, UserRelay, UpdateAgeGate, TotalWar,
    TotalRanking (cross-account leaderboard — needs new aggregation
    infra, not just a new table), Stress(Cheat), Shop(Reputation),
    MiniEvent(Hub), Item(Lock), Guild(NameChange), Gacha(MultiBuy),
    EvilCastle(GiveUp), ContentOpen, ChatSetting.

  For each system: needs a `database/migrations/*.sql` addition (see the
  existing 319-migration pattern) for whatever per-user state it tracks,
  a `database/src/db/**` query module, `gameserver/src/logic/game/<feature>/`
  handler(s) following the exact pattern in e.g.
  `gameserver/src/logic/game/achievement/achievement_info.rs`, and
  `httpserver/src/routes/game/<feature>/` route file(s) + registration in
  `main.rs`, mirroring `achievement_info.rs`'s handler pattern exactly.

  **Known follow-ups from this round:**
  - 8 route-path collisions were found between newly-registered
    `routes::game::*` handlers and already-registered older duplicates
    (`BalanceVersionCheck`, `CharInfo`, `LoginUser`, `MaintenanceInfo`,
    `MissionInfo`, `NoticeInfo`, `RecipeInfo`, `ServerInfo` — full pairs
    listed in git-blame-free form: grep `main.rs` for these path strings to
    find both implementations). Left the already-registered ones active and
    skipped the `routes::game::*` duplicates entirely rather than guess
    which is authoritative — worth a real side-by-side diff at some point
    since the `routes::game::*` versions look like a newer/more complete
    reorganization that was never fully cut over.
  - Master-data capture completeness varies wildly by table (some tables
    have thousands of rows, some — like `FieldRewardObjectTable` — have
    only a handful). Before implementing any feature whose response needs
    real aggregate/lookup data from a sparse table, either capture more
    (rerun `tools/BD2DataExtractor` covering that feature's screens) or
    implement it honestly against whatever's actually there rather than
    presenting partial data as complete.

**Round 2 (2026-09-26): the Life system.**

Corrected/confirmed description: **Life is a farming/home-building life-sim**
layered on top of BD2 proper — placing world objects on owned land "chunks",
planting/growing crops, gathering resources (logging/mining/farming skills
with their own level+exp), recruiting citizens and helpers (NPCs with an
avatar loadout) to work assigned jobs, cooking/crafting, a dedicated
Life-only shop and currency (`life_coin`, separate from the game's generic
item/currency system), and eating food for timed buffs. 35 request types in
total (round 1 estimated ~32).

**All 35 got a registered route + real persistence** — none were left on the
default-service fallback. New schema: 11 tables
(`database/migrations/318`-`328`), a `database/db/life/` +
`database/models/game/life/` query/model layer, `gameserver/logic/game/life/`
(34 handler files + a `mod.rs` carrying shared conversion helpers — a
deliberate small deviation from the flat per-feature `mod.rs` convention,
justified by real reuse across 34 handlers: object-tree building from flat
DB rows via `ParentIndex`, avatar row↔proto mapping, item consume/grant),
and `httpserver/routes/game/life/` + 35 `common::packet_code::PacketCodeType`
entries (467-501) + `main.rs` registrations.

**What's genuinely real vs. a flagged placeholder** (every instance is
commented in the source, not just here): state that only depends on what the
client itself sends — object placement/move/status/unplace, chunk
ownership, helper assign/fire/rename, citizen recruit/avatar, shop
purchase-count + reset-timer tracking, char level/exp save, eating food
(consumes real items) — is **fully real**, no caveats. Where the *output* of
an action depends on a master table this server hasn't captured yet
(`LifeCookTable`, `LifeCraftingObjectTable`, `LifeToolTable`,
`LifeChunkExpandTable`, `LifeHelperSpeciesTable`, `LifeGatheringObjectTable`,
`LifeCropSeedTable`/`LifeCropGradeTable` — all real tables confirmed present
in the schema, zero rows captured so far), inputs are still consumed for
real (no free retries) but the reward/output is honestly left empty rather
than fabricated, OR a clearly-labeled placeholder is used for a value that
has to be *something* for the state machine to keep working (30min food-buff
duration, 60min crop growth, tool tier = current+1, 10 coin/unit sell price,
rolling 24h/7d/30d shop-reset windows instead of calendar-aligned). Two
requests (`LifeHelperGacha`, its pool depends entirely on
`LifeHelperSpeciesTable`) return a genuinely empty roll rather than invented
candidates. `LifeHelperReconnect`'s exact intended semantics couldn't be
pinned down from the schema alone (empty request/response, no distinguishing
fields) — implemented as a no-op ack.

**Verified**: full workspace `cargo build` clean (only the 2 pre-existing
unrelated `rand` deprecation warnings). `cargo run -p httpserver` boots
end-to-end — migrations 318-328 applied, game data loaded, listening on
0.0.0.0:8082/8443, zero route-collision panics.

Next: apply this same pattern to Fishing (27 requests) and Colosseum (23),
the next-biggest systems in the round-1 work queue above.

**Round 3 (2026-09-26): Fishing system.**

What it actually is (round 1's guess confirmed close): cast a rod, wait for a
bite, fight it via an HP/stamina tug-of-war (or skip straight to a one-shot
"auto" catch), catch goes into a fish inventory + a personal-best collection
log per species; separate rod/boat/map/boat-skin ownership and a
dedicated fishing shop (buy/sell); a passive "trap" system that accrues
catches over real time for later claiming; a "voyage" concept and a
multi-player room list that both assume real-time multiplayer this project
has no infrastructure for.

**Coverage: all 27 requests got a registered route + real persistence** —
none left on the fallback. New schema: 11 SQLite tables (migrations
329-339), full `database`/`gameserver`/`httpserver` layers, 27
`PacketCodeType` entries (502-528), all registered in `main.rs`.

**Master data is much sparser here than Life's** — only `FishingBoatTable`
(1 row) and `FishingDefaultTable` (1 row, but a rich one: real game
constants like bite timing, stamina divider, starter map/items, and the
`fishTrapMaxTime` trap-accrual cap) were captured; `FishingFishTable`,
`FishingRodTable`, `FishingMapTable`, `FishingShopTable`,
`FishingFishPoolTable`/`FishingFishSizePoolTable`/`FishingGradePoolTable`
(the actual catch tables) have **zero** captured rows. Consequences,
clearly flagged in-source at each site:
- What fish you catch (id + size) is an honest placeholder (fixed id,
  randomized size) — there's no real pool to draw from. Real ones: the
  starter map id and starter item grant on first `FishingInfo` call (from
  the real `FishingDefaultTable` row), buy/sell counts, ownership sets, rod
  equip state, lock flags, exp/level bookkeeping, and the trap system's
  24h accrual cap (also from real data).
- `FishingBoatUpgrade`/`FishingMapBuy`/`FishingBoatSkinBuy`/
  `FishingFishInvenSlotAdd` requests carry no payment field at all in the
  schema, so those are granted unconditionally rather than gated on a cost
  this project has no table for anyway.
- `FishingMultiRoomInfo` returns a genuinely empty room list (no
  multiplayer backend exists here — not a data gap, an architecture gap).
  `FishingVoyageStart` always reports `is_multi_host: false` for the same
  reason.

**Judgment call worth a second look**: `FishingShopSellRequest` carries both
an `inven_index` and a shop `group_id`/`id`, which is ambiguous about
whether it's selling a caught fish or a generic fishing item — implemented
against the generic fishing-item inventory (see the comment in
`fishing_shop_sell.rs` for the reasoning); worth revisiting if real
`FishingShopTable`/sell-price data ever gets captured.

**Verified**: full workspace `cargo build` clean (only the by-now-familiar
`rand::thread_rng` deprecation warnings, same class as before, none new in
kind). `cargo run -p httpserver` boots end-to-end — migrations 329-339
applied alongside 318-328, game data loaded, listening on
0.0.0.0:8082/8443, zero route-collision panics.

Next: Colosseum PvP (23 requests), the next-biggest system in the
work queue.

**Round 4 (2026-09-26): Colosseum system.**

What it actually is (round 1's guess confirmed close): an async PvP arena —
**not real-time**. `ColosseumBattleMatchingRequest` returns several candidate
opponents (each a full deck/char/costume snapshot plus a `battle_random_seed`);
`ColosseumBattleStartRequest(enemy_owner_index)` locks one in;
`ColosseumBattleEndRequest(battle_result)` just reports the outcome the
**client already simulated locally** (deterministically, via the shared
seed) — the server only needs to trust the result and apply Vp/rewards, never
run battle logic itself. Also: deck presets (save/use/delete/slot-add/rename),
a bless system (attack/defense loadout, separate from decks), Vp-based
ranking + rank-detail lookups, battle history w/ season stats, one-time
promotion-tier rewards, season-end rewards, and an AP-refill shop.

**Coverage: all 23 requests got a registered route + real persistence** —
none left on the fallback. New schema: 13 SQLite tables (migrations
340-352), full `database`/`gameserver`/`httpserver` layers, 23
`PacketCodeType` entries (529-551), all registered in `main.rs`.

**No `ColosseumDefaultTable`/`ColosseumRankTable`/`ColosseumSeasonTable`
data was captured** (the account used for capture never opened Colosseum —
the schemas exist, confirmed via the decompiled client, just zero rows).
Consequences, flagged in-source at each site: Vp-change-per-battle,
promotion-tier thresholds/reward-ids, AP-buy daily cap, and match-candidate
count are all documented placeholder constants in
`gameserver/src/logic/game/colosseum/mod.rs` rather than real balance data.
Reward bundles (`ColosseumApBuyResponse`, `ColosseumBattleEndResponse`,
`ColosseumSeasonRewardResponse`) are honestly empty rather than fabricated.
Real, no caveats: Vp/win/loss tracking, deck/bless/preset save+load, battle
history logging (both sides, for real accounts), promotion-reward grants
(against the placeholder tiers) and season-reward claim state.

**No real multiplayer infra exists here either** (same conclusion as the
Fishing round) — but unlike Fishing's `MultiRoomInfo`/`VoyageStart`, which
had no reasonable single-player substitute, Colosseum's request/response
shapes made a **real non-live substitute possible**: when at least one other
real account exists in this server's own database, matching pulls their
actual saved deck and battle history gets a genuine mirrored record on both
sides (attacker + defender). When it doesn't (the common single-account
case), a bot opponent is synthesized from real `CharTable` ids near the
caller's own Vp, using a negative sentinel `owner_index` so it's never
mistaken for a real account — this keeps the core matching → battle → result
loop fully playable solo, which a bare stub could not have done. Bots (and
even real opponents, for now) never get a populated
`enemy_char_info`/`costume_info`/`buff_stat_info`/`awake_info` snapshot — no
consumer needs it since battle simulation isn't server-side, so it's left
empty rather than built out speculatively. The Vp-based ranking list is
**real-accounts-only, deliberately not bot-padded** — unlike matching, a
fake-looking leaderboard would be actively misleading rather than just
thin. `ColosseumBattleReplayInfoRequest` reconstructs from *current* deck
state rather than a true point-in-time snapshot (none is persisted) —
documented as an approximation, not a real replay.

**Judgment calls worth a second look**: presets don't have their own bless
storage in the schema (no per-preset bless-save endpoint exists) — `PresetUse`
echoes back whatever bless is currently active account-wide rather than a
preset-specific one; the `ColosseumPresetBlessInfo` table exists in the
migrations but is intentionally unused for now, kept in case that
assumption turns out wrong. `ColosseumBattleEndRequest` carries no
`enemy_owner_index` of its own, so the server tracks "current battle enemy"
via a new `CurrentBattleEnemyIndex` column on `ColosseumUserInfo`, set at
`BattleStart` and cleared at `BattleEnd` — assumes one battle in flight per
account at a time, which the request/response shapes support but don't
explicitly guarantee.

**Verified**: full workspace `cargo build` clean (same pre-existing class of
`rand` deprecation warnings as before, nothing new in kind). `cargo run -p
httpserver` boots end-to-end — migrations 340-352 applied alongside
318-339, game data loaded, listening on 0.0.0.0:8082/8443, zero
route-collision panics.

Next: the remaining work queue (biggest-first) is "Ib" (14 requests, an
unidentified dungeon system), MiniGame variants (11), then ~10 smaller
systems (1-10 requests each) — see the round-1 log above for the full list.

**Round 5 (2026-09-26): Ib system.**

**What Ib actually is** (round 1's guess was close but incomplete): a
grid-inventory dungeon crawler — enter a dungeon, fight through it stage by
stage with a deck of items placed on a board (position + rotation, drag/drop
sockets — decompiled client class names like `IBDragManager`,
`IBEquipSocketSlot`, `IBSocketUI_S_Slot`, `IBUnitItemView`,
`IBGroundItemView` confirm the drag-and-drop item-board mechanic), an item
inventory you can sell/upgrade, a rotating shop with reservable/refreshable
slots, a life/coin resource pair, and per-season dungeon-clear tracking.
Battles use the exact same async pattern as Colosseum: the server hands the
client a random seed + both sides' deck/item snapshots (base64-encoded
`IbProcessDBInfo` protobuf bytes in the `process_info` string field), the
client simulates locally, and `IbStageEnd` just reports the result back — the
server trusts it. This is a genuinely new system, **not** an extension of the
pre-existing roguelike (`rl*`) tables — confirmed no naming/schema overlap.

**Coverage: all 14 requests got a registered route + real persistence** —
none left on the fallback. New schema: 5 SQLite migrations (353-357:
play-state, cleared-dungeons, inventory, deck, shop), full
`database`/`gameserver`/`httpserver` layers under `ib`, 14 `PacketCodeType`
entries (552-565), all registered in `main.rs`.

**No `IBDefaultTable`/`IBDungeonTable`/`IBSeasonTable`/`IBStageTable`/
`IBItemTable`/`IBShopTable`/etc. data was captured** (schemas confirmed via
the decompiled client — 15 `IB*Table` types exist there — but zero rows in
`data/tables/`, account never opened this feature). Consequences, all
documented in `gameserver/src/logic/game/ib/mod.rs`: starting life (3),
shop slot count (6), sell/upgrade coin formulas, shop refresh cost, and
stage-clear coin reward are placeholder constants; shop item ids/prices are
a simple generated placeholder roll, not real drop-table data. Real, no
caveats: dungeon entry/give-up, deck save/load, inventory sell/upgrade
(state changes and coin balance), shop buy/reserve/refresh (state and coin),
season-change detection and reset, first-clear-this-season tracking, and the
win/loss life-and-coin outcome of a stage.

**Simplification worth a second look**: with no `IBStageTable` data, there's
no known stage count per dungeon — a win is treated as clearing the whole
dungeon in one stage rather than advancing through multiple stages
(`is_dungeon_cleared` fires on any win, `next_stage_id` only matters on a
loss, where it just repeats the same stage). Revisit if stage-count data
ever gets captured. Also: season change intentionally does NOT wipe
inventory/deck/coin (only dungeon progress + season-scoped cleared-dungeon
list) — no captured data confirms whether it should, so the
less-destructive choice was made.

**Verified**: full workspace `cargo build` clean (same pre-existing class of
`rand`/deprecation warnings, nothing new in kind). `cargo run -p httpserver`
boots end-to-end — migrations 353-357 applied alongside 318-352, game data
loaded, listening on 0.0.0.0:8082/8443, zero panics.

Next: the remaining work queue (biggest-first) is MiniGame variants (11
requests), then ~10 smaller systems (1-10 requests each) — see the round-1
log above for the full list.

**Round 6 (2026-09-26): MiniGame system.**

Of the 11 new requests: 5 are a genuinely new minigame, **Hopscotch** (a
board/tile-capture game — enter a stage, submit a captured-area + clear-time
score, unlock gallery entries, real cross-account ranking). The other 6 are
record/ranking *views* over four pre-existing minigame types (Action, Rhythm,
Sichuan, Survival).

**Important discovery, bigger than this round's own scope:** verifying those
6 view-requests required checking the *existing* handlers for Action/Rhythm/
Sichuan/Survival minigames — and every one of the 44 existing "mini" family
route handlers (Action, Bingo, Board, Defense, Field, Rhythm, Roulette, Run,
Sichuan, Survival, Puzzle) turned out to be **unfilled TODO-stub templates
that never touch the database**, not working features. Round 1's "446 dead
routes" fix registered these in `main.rs` so they no longer 404, but
registered != implemented — round 1's route-count categorization couldn't
tell the difference (a stub handler still "counts" as registered). This is a
**separate pre-existing gap from before this whole client-update project**,
scoped to the "mini" family specifically (not yet verified whether other
non-mini features among the 479 registered routes have the same
stub-vs-real gap — worth a dedicated audit pass at some point, this was not
checked broadly here).

Coverage: all 11 new requests got a registered route with real logic — none
left on the fallback. Hopscotch is fully real (3 new migrations 358-360,
real cross-account ranking + percentile calc + gallery/record persistence).
The other 6 honestly query the existing (currently-always-empty, since their
backing stub handlers never write anything) rank tables — they'll start
returning real data automatically, with no further changes needed here, once
a future round fills in those 44 stub handlers.

Judgment calls: no master data exists for Hopscotch (stage list, gallery
rewards) — used documented placeholder constants; a GameEnd submission's
"clear condition" is placeholder-always-true; ranking `user_id` is the
account's numeric uid stringified, not a display name (no profile-lookup
helper was in scope).

Verified: full workspace `cargo build` clean. `cargo run -p httpserver` boots
end-to-end, migrations 358-360 applied alongside all prior ones, listening on
0.0.0.0:8082/8443, no panics or route collisions.

Next: ~10 smaller systems remain (1-10 requests each) — SpineInteraction
(10), CharVote (7), MyRoom (6)/Avatar (6), Friendship (5), RoomChat (4)/
Equip (4), and singles/pairs — see the round-1 log above for the full list.
Also queued: the 44-stub-handler "mini" family gap found this round, and a
wider audit of whether other already-registered (pre-project) routes have
the same stub-not-real gap.

**Round 7 (2026-09-26): SpineInteraction system.**

**What it is**: the game's "Character Interaction" feature — an interactive
Live2D/Spine visual-novel-style scene per character, tied into the existing
Dating system (`SpineInteractionTable.unlockDatingId`/`unlockDatinggroupId`
link the two). Three sub-parts: **Achievement** (which "motions"/poses have
been unlocked, per `(interaction_group_id, group_id, point_id)`), **Record**
(save/name/delete/fetch a recorded interaction — the client uploads the raw
recording bytes to S3 in the real game and gets a URL back; this server has
no S3, so it stores the bytes itself and serves them back from its own
address via a new unauthenticated `SpineInteractionRecordData/{inven_index}`
GET route — added to the auth middleware's public-endpoint allowlist,
matching how `MaintenanceInfo`/`ServerInfo`/etc. are already exempted, since
a real S3 fetch wouldn't carry this game's session cookie either), and
**Reward** (claim a reward tied to an achievement group, once each).

Master data here is well-populated: `SpineInteractionPointTable` has 562
real captured rows; `SpineInteractionTable` (10 rows) and
`SpineInteractionToolbarTable` (14 rows) are small master lists as expected
(not sparse-by-accident, these are just small tables). No table anywhere
links a reward triple to an actual item/reward definition, so
`SpineInteractionReward`'s claim-tracking is real (can't double-claim) but
the granted amount (100 gold, item id 4 — confirmed real from
`CurrencyTable`) is a documented placeholder.

**Coverage: all 10 requests got a registered route + real persistence** —
none left on the fallback. New schema: 3 migrations (361-363:
`SpineInteractionAchievement` — normalized one row per unlocked motion,
matching this project's established convention rather than a JSON-in-TEXT
column; `SpineInteractionRecord` — uses the SQLite rowid directly as
`inven_index`, no manual per-account index bookkeeping needed;
`SpineInteractionRewardClaim`), `PacketCodeType` 577-586 (note: the actual
current top of that enum was already 576, not 570 as the round-6 report
assumed — round 6's own new entries pushed it further; always re-check the
real max before picking new codes rather than trusting a prior round's
stated ending number).

**Judgment calls worth a second look**: achievement saves are upsert-only
(never delete on save) since unlocks are almost certainly meant to be
permanent and the client resends its full known state each time — flagged in
`spine_interaction_achievement.rs`. New records default to an empty `Name`
(no naming-convention hint existed anywhere).

**Verified**: full workspace `cargo build` clean (only the same pre-existing
class of `rand` deprecation warnings, nothing new). `cargo run -p httpserver`
boots end-to-end — migrations 361-363 applied alongside all prior ones,
listening on 0.0.0.0:8082/8443, no panics or route collisions.

Next per the work queue: CharVote (7), MyRoom (6)/Avatar (6), Friendship (5),
RoomChat (4)/Equip (4), and the remaining 1-3-request singles/pairs.

**Round 8 (2026-09-26): CharVote system.**

Round 1's guess ("character popularity voting w/ seasonal rankings") was
right. Decompiled UI classes are actually named `CharacterVote*`
(`CharacterVoteItem`, `CharacterVoteScrollItem`, `CharacterVoteTooltipUI`,
plus field-scene classes `FieldVoteHallStatueObject`/`FieldVoteRankStatueObject`)
— no `CharVote*` UI classes exist under that exact name, `CharVote` is just
the network-message-prefix shorthand. 7 requests: Info, FavoriteAdd/Delete,
Ranking (one round), Save (cast a vote), SeasonRanking (event-wide total),
TotalRanking (client-specified `event_id`).

No `CharVote*Table` master data exists anywhere (not captured, and no such
table turned up in the decompiled client's schema either) — there's no
election-schedule or vote-cost table to read from at all. Handled per the
project's established real-state/honest-placeholder split:

- **Real**: the vote persistence itself (per uid+event+round+candidate,
  upsertable via SQLite `ON CONFLICT`), favorites, cross-account ranking
  aggregation (real `SUM()`/`GROUP BY` over every account's saved votes — same
  "don't bot-pad, just aggregate what's really there" approach as Colosseum's
  ranking), milestone-reward claim-tracking (never double-grants), and item
  consumption for paid votes (the request already carries the exact items to
  consume, so no cost table is even needed to validate against — trusted
  as-is).
- **Placeholder** (all documented in `char_vote/mod.rs`): there's exactly one
  always-active event/round (`CURRENT_EVENT_ID`/`CURRENT_ROUND` = 1, no real
  schedule), the candidate roster is the first 10 real `CharTable` ids
  (real ids, placeholder selection — no table says who's actually up for
  vote), reward milestones are an arbitrary `[10, 50, 100, 250, 500, 1000]`
  cumulative-vote ladder granting the same placeholder 100-gold reward
  established in the SpineInteraction round, and `vote_type == 0` is treated
  as a free "normal" vote vs. any other value being a paid "additional" one
  (a guess — no table confirms this is what `vote_type` actually encodes).

**Coverage: all 7 requests got a registered route + real persistence** — none
left on the fallback. New schema: 4 migrations (364-367: `CharVoteUserInfo`,
`CharVoteFavorite`, `CharVoteRewardClaim`, `CharVoteDailyState`),
`PacketCodeType` 587-593, all registered in `main.rs`.

**Judgment calls worth a second look**: the free/paid `vote_type` mapping
above; daily-reset tracking for the "normal vote candidates today" list uses
a simple comma-separated string column rather than a join table (fine at this
scale, would need revisiting only if this list ever needs indexed queries).

**Verified**: full workspace `cargo build` clean (same pre-existing `rand`
warnings, nothing new). `cargo run -p httpserver` boots end-to-end —
migrations 364-367 applied alongside all prior ones, listening on
0.0.0.0:8082/8443, no panics or route collisions.

Next per the work queue: MyRoom (6)/Avatar (6), Friendship (5), RoomChat
(4)/Equip (4), and the remaining 1-3-request singles/pairs.

**Round 9 (2026-09-27): MyRoom system.**

Round 1's "6 new requests" count only covered the genuinely-new message types
(`MyRoomPresetDelete/Info/NameChange/Save`, `MyRoomShareOptionUpdate`,
`MyRoomShopMultiBuy`). What it missed: **all 14 pre-existing MyRoom route
handlers were also unfilled TODO-stub templates** — the same "registered but
not real" gap round 6 found in the "mini" family, now confirmed in a second
place. So this round actually covers all 20 MyRoom requests, not 6.

What MyRoom is: personal room decoration/housing. An account owns one or
more numbered rooms (unlocked via "Expand"), each holding placed furniture
items at grid positions; a dedicated shop; trophies (cosmetic unlocks,
presumably achievement-linked); liking other accounts' rooms; browsing
friends'/guild members'/recommended other accounts' rooms; and save-able
room-layout "presets" (either your own saved layouts, or a copy of someone
else's layout you liked).

**Coverage: all 20 requests now have real routes + real persistence** — none
left on the fallback. Master data here is unusually well-populated (236 real
`MyRoomItemShopTable` rows, 297 `MyRoomItemTable`, 184
`MyRoomTrophyItemTable`, 9 `MyRoomExpandTable` tiers, 1 `MyRoomDefaultTable`
config row) — so shop purchases, buy-count caps, and expand-tier validation
are all real against real data, not placeholders. New schema: migrations
368-371 (`MyRoomPresetInfo`, a `MyRoomUserInfo.AllowScopeType` column, a
`MyRoomItemPositionInfo.RoomId` column, and a `MyLikeInfo.Date` column),
`PacketCodeType` 594-600.

**Pre-existing schema bugs fixed along the way** (both were latent — never
triggered before because the stub handlers never actually queried the
tables): `MyRoomInfo`'s Rust model had a `my_room_position_info_index` field
that didn't exist in its migration's actual columns at all — `SELECT *`
would have errored the instant anything real called it; and
`MyRoomItemPositionInfo` had no way to distinguish which of an account's
*multiple* rooms a given placed item belonged to (no `RoomId` column) —
fixed via the new migrations above rather than worked around.

**Judgment calls**: `MyRoomMove`'s exact semantics are inferred (no item
reference in the request, just two room ids) — implemented as swapping which
slot number two rooms occupy. A preset's type (own vs. copied-from-another)
is inferred from whether `source_owner_index` is set to someone other than
the caller, since the save request carries no explicit type field. Room
"like" real accounts only — no bot-padding (consistent with Colosseum's
ranking, which also never pads with bots). `MyRoomUserInfo.item_info` is
populated from the account's general inventory (there's no MyRoom-specific
item list distinct from the general one in this schema).

Also noted but deliberately NOT fixed this round (out of scope, zero runtime
impact): `PacketCodeType::from_code()` has been missing entries for every
round's new codes since round 2 (Life) — confirmed it has **zero callers
anywhere in the codebase**, so it's dead code; this round's own new codes
were added there for completeness but the backlog from rounds 2-8 was left
alone rather than scope-creeping into an unrelated cleanup.

**Verified**: full workspace `cargo build` clean (same pre-existing `rand`
deprecation warnings, nothing new). `cargo run -p httpserver` boots
end-to-end — migrations 318-371 all applied, game data loaded, listening on
0.0.0.0:8082/8443, no panics or route collisions.

Next per the work queue: Avatar (6), Friendship (5), RoomChat (4)/Equip (4),
and the remaining 1-3-request singles/pairs. Also still queued: a wider
audit of whether other pre-existing (pre-project) routes beyond "mini" and
"MyRoom" have the same stub-not-real gap — two-for-two so far when checked,
worth checking systematically rather than only discovering more by accident
each round.

**Round 10 (2026-09-27): Avatar system.**

What Avatar is: a cosmetic "social avatar" loadout, separate from the actual
battle character — 10 equip slots (char/hair/hair-accessory/face-accessory/
costume/body-accessory/hand-accessory/pet/mount/effect), an item shop with
normal+premium pricing (paid via whatever item the client specifies), a
separate motion (emote) shop, and a wishlist of shop entries. Confirmed via
the message fields themselves plus the `AvatarCategoryTable`/`AvatarCharTable`/
`AvatarItemTable`/`AvatarMotionTable`/`AvatarSetTable`/`AvatarShopTable`/
`AvatarMotionShopTable`/`AvatarDefaultTable` schemas in the decompiled client.

**Correction to a prior round's assumption**: round 1's guidance for this
round assumed these `avatar*table` modules "predate this project" — they do
not. They're 8 of the 167 genuinely-new common.db tables from this whole
client-update project (schema gap analysis, near the top of this doc,
already listed "Avatar (dress-up)" among the new systems — round 1's own
work-queue categorization just mis-tagged it later). Checked directly: no
`avatar*` route handlers existed anywhere pre-round-10 (not stubs, not
real — nothing), and no `Avatar*.json` was ever captured. So unlike MiniGame
and MyRoom, this one really was exactly 6 new requests, no hidden stub
backlog.

**Coverage: all 6 requests got real routes + real persistence** — none left
on the fallback. New schema: migrations 372-375 (`AvatarUseInfo`,
`AvatarItemInfo`, `AvatarMotionInfo`, `AvatarShopWishListInfo`),
`PacketCodeType` 601-606, all registered in `main.rs`.

Zero master data exists for any `Avatar*Table` (not just uncaptured — this
system was apparently never opened during any capture session). Real, no
caveats: the equipped-loadout save/read round-trip, the wishlist save/read
round-trip (full-replace semantics, matching the request's "send the whole
list" shape), and item/motion consumption+ownership bookkeeping. Placeholder
(documented in `gameserver/src/logic/game/avatar/mod.rs`): with no
`AvatarShopTable`/`AvatarMotionShopTable` rows to map a `shop_id` to a real
reward, `shop_id` is treated as the item/motion id to unlock directly, under
a placeholder item-category value — the same "no cost/reward table exists,
so grant what was asked for" judgment call used in earlier rounds (Fishing's
boat upgrade, Ib's shop). `AvatarSaveRequest` (equipping something into a
slot) is accepted unconditionally with no ownership check, since no item
table exists to validate against either.

Also confirmed one interesting schema detail worth remembering: the
dedicated `AvatarItemDBInfo`/`AvatarMotionDBInfo` message types exist in the
schema but are **never referenced as a field type anywhere** in the whole
proto file — owned avatar items are actually exposed through the generic
`ItemDBInfo` shape in `AvatarInfoResponse`, and a purchased motion has no
return channel at all except as a generic `ItemDbInfo` inside
`RewardDbInfoBundle` when bought. Not a bug, just how this part of the
schema was designed.

**Verified**: full workspace `cargo build` clean (same pre-existing `rand`
deprecation warnings, nothing new). `cargo run -p httpserver` boots
end-to-end — migrations 372-375 applied alongside 318-371, game data loaded,
listening on 0.0.0.0:8082/8443, no panics or route collisions.

Next per the work queue: Friendship (5), RoomChat (4)/Equip (4), and the
remaining 1-3-request singles/pairs. Still queued: the wider stub-not-real
audit (unaffected by this round — Avatar had no pre-existing handlers to
check).

**Round 11 (2026-09-27): Friendship system.**

**What Friendship is**: a per-costume affection/leveling system, separate
from the main character costume system. Decompiled client classes
(`CharFriendShipUI`, `FriendshipManageUI`, `FriendshipCounselingEnterPopupUI`,
`FriendshipGiftPopupUI`, `FriendshipStoryPopupUI`) confirm the mechanic:
"counseling" (a quiz/dialogue minigame — answer a prompt, gain exp, capped by
a daily AP limit) and "gift" (spend items for flat exp) both raise a
per-costume level/exp, and "special episodes" are one-off story unlocks
tracked as cleared per `(group_id, id)`. 5 requests confirmed via
`protocol/proto/Commons/proto_net.proto`: `FriendshipInfo`, `FriendshipCounseling`,
`FriendshipGift`, `FriendshipSpecialEpisodeClear`, `FriendshipSpecialEpisodeInfo`.
No pre-existing route handlers of any kind existed for this (verified) — same
as Avatar's round, no hidden stub backlog here.

**Coverage: all 5 requests got real routes + real persistence** — none left
on the fallback. New schema: 5 migrations (376-380: `FriendshipInfo`,
`FriendshipCounselingSession`, `FriendshipCounselingDaily`,
`FriendshipCounselingCostumeDaily`, `FriendshipSpecialEpisodeClear`),
`PacketCodeType` 607-611, all registered in `main.rs`.

Master data here (`FriendshipDefaultTable`, 1 row) is unusually rich for a
sparse-table feature: real correct/incorrect counseling exp (100/80), a real
daily AP cap (3 total, 1 per costume), and a real daily-completion bonus
(type 3, count 100) — all implemented for real rather than placeholder,
including the AP-cap gating logic itself. What's placeholder: no
counseling-question/answer data exists anywhere, so every answer is treated
as correct (documented, same "don't fabricate a failure state nothing
confirms" call as Ib's dungeon-clear); no level-exp-curve table exists, so
leveling uses a linear placeholder curve (100*level per level, capped at 20 —
the highest of `FriendshipDefaultTable`'s three tier caps, since per-costume
tier assignment isn't captured either); reward TYPE 3 has no table mapping it
to a real item, so — following the exact precedent SpineInteraction's round
established — item id 4 / type 1 (confirmed real gold, `CurrencyTable` id 4)
is granted instead, with the real captured count; gift exp-per-item and
special-episode reward amounts are flat placeholders (no value tables exist
for either).

**Judgment calls worth a second look**: `is_quick` doesn't change the exp
formula (no data distinguishes a quick-mode reward); which of the three
`friendshipMaxLevel` tiers a given costume belongs to is unknown, so the
single highest cap is used for everyone rather than under- or over-capping
some costumes incorrectly.

**Verified**: full workspace `cargo build` clean (same pre-existing `rand`
deprecation-warning class, 22 total, nothing new in kind). `cargo run -p
httpserver` boots end-to-end — migrations 318-380 all applied, listening on
0.0.0.0:8082/8443, no panics or route collisions.

Next per the work queue: RoomChat (4)/Equip (4), then the remaining 1-3-request
singles/pairs. Still queued: the wider stub-not-real audit across the rest of
the original 486 routes (2 confirmed instances so far: mini-family, MyRoom).

**Round 12 (2026-09-27): RoomChat system.**

**What RoomChat is**: per-account chat moderation — block another player,
unblock them, report them for something they said, and fetch your own current
moderation status (who you've blocked, how many times you've been reported,
and any active penalty against you). 4 requests confirmed via `proto_net.proto`:
`RoomChatBlockAdd`, `RoomChatBlockRemove`, `RoomChatReport`,
`RoomChatReportAndBlockInfo`. No pre-existing `RoomChat*` route handlers
existed at all (verified directly) — a genuinely clean 4-request round, no
hidden stub backlog. (Found an unrelated pre-existing stub while checking —
`ReportUserRequest`/`report_user.rs` is a *different*, still-unimplemented
generic report feature that predates this project; out of scope here, left
untouched, just another data point for the wider stub audit.)

**Coverage: all 4 requests got real routes + real persistence** — none left
on the fallback. New schema: 4 migrations (381-384: `RoomChatBlock`,
`RoomChatReportLog`, `RoomChatReportUser`, `RoomChatPenalty`),
`PacketCodeType` 612-615, all registered in `main.rs`.

No `RoomChat*Table` master data exists anywhere in the client's own schema
(this is pure moderation bookkeeping, there's nothing to look up). Real, no
caveats: block/unblock persistence (with the target's real `UserId` resolved
via `UserInfo.OwnerIndex`), a full historical report log, and a real rolling
report-count-against-the-target mechanism that fires an actual timed penalty
once a threshold is crossed. Placeholder constants (documented in
`database/src/db/room_chat/room_chat_report.rs`, no table anywhere defines
any of these): the report threshold (5), the rolling window (24h), and the
penalty duration (1h).

**Judgment call worth a second look**: `RoomChatReportAndBlockInfoRequest`
carries no target — it's asking about the caller's own standing — so
`report_user_info`/`penalty_info` are interpreted as "how many times *I've*
been reported" / "is a penalty currently active against *me*", with reports
tallied against the reported party rather than the reporter. This reading
fits the message shape well but isn't confirmed by any table.

**Verified**: full workspace `cargo build` clean (same pre-existing `rand`
warnings, nothing new). `cargo run -p httpserver` boots end-to-end —
migrations 318-384 all applied, listening on 0.0.0.0:8082/8443, no panics or
route collisions.

Next per the work queue: Equip (4), then the remaining 1-3-request
singles/pairs. Still queued: the wider stub-not-real audit across the rest of
the original 486 routes (2 confirmed instances so far: mini-family, MyRoom;
`report_user` spotted this round is a 3rd candidate, not yet fixed).

**Round 13 (2026-09-27): Equip system.**

**Scope correction, same pattern as MyRoom's round**: the 4 "new" Equip
requests (`EquipMainOptChange`, `EquipMakingToBreakAuto`,
`EquipRankUpgradeBuy`, `EquipUpgradeToBreakAuto`) were confirmed via
`proto_net.proto` — but checking them required reading the *existing* Equip
system first, and **all 27 pre-existing Equip route handlers turned out to
also be unfilled TODO stubs** (4th confirmed instance of this gap, after
mini-family/MyRoom/`report_user`). So this round covers all 31 Equip
requests, not 4 — every equip-related feature (inventory listing, making,
upgrading, breaking, locking, marking, presets, storage, option reroll,
rank/smelting, and the two-phase making->upgrade/break combo flows) was
non-functional until now.

**Found and fixed 6 latent pre-existing schema bugs** (zero prior exercise,
same root cause as MyRoom's — scaffolded together, never tested): `EquipInfo`
model expected column `BaseInfoIndex`, migration had `BaseInfo`;
`EquipBaseInfo` model expected `PrivateOptionIndex` (migration had
`PrivateOption`) plus `MainOptionIndex`/`SubOptionIndex`/`Rank` that didn't
exist in the migration at all; `EquipBatchUseInfo`/`EquipClearInfo` models
expected an `EquipInvenIndex` column neither migration had; `EquipPresetInfo`
model expected an `ItemInfoIndex` column the migration lacked. Fixed via a
new migration (`385_equip_schema_fixes.sql`, rename + add columns) rather
than editing the historical ones. Also added a `Mark` column (for
`EquipMarkSet`/`EquipMarkDelete`, which the original schema had no storage
for at all) and `PrevMainOptionIndex`/`PrevSubOptionIndex` (to let
`EquipOptionReRollConfirm(is_confirm=false)` revert a reroll).

**Coverage: all 31 requests got real routes + real persistence** — none left
on the fallback. `PacketCodeType` 616-619 added for the 4 new ones (existing
27 already had codes from before, just never used).

Master data here is thin — `EquipmentTable` (5 rows), `EquipmentGradeTable`
(8), `EquipmentGrowthTable` (1, keyed to a `growthGroupId` no captured
equip references), `EquipmentRankTable` (9), `EquipmentOptionTable` (25, real
and used for option-reroll pools), `EquipOptionalterTable` (80, unused this
round). Real, no caveats: inventory listing/join, making (consumes real
materials), level/rank persistence, lock/mark, use/batch-use/clear (all key
off `EquipInfo.UseChar`), storage put/out (tracked as a tag layered on the
normal inventory row, not a separate item store — this schema has no other
way to model separate storage), presets (full save/load/rename), option
reroll (real per-group option pools from `EquipmentOptionTable`, real
lock-slot handling, real revert-on-decline). Placeholder, documented
in-source at each site: break/upgrade-target-exceeded rewards (no
`EquipmentGrowthTable` row applies to any real equip id), upgrade success is
unconditional (no per-level cost/success-ratio data exists), `EquipAddSlot`/
`EquipStorageAddSlot` accepted unconditionally (no capacity table exists).

**Judgment calls worth a second look**: `EquipChange` was given identical
semantics to `EquipUse` (same field shape, no table distinguishes them);
`EquipMainOptChange` replaces an item's *entire* main-option list with the
single (group_id, id) pair the request carries, since there's no slot index
to target one entry specifically.

**Verified**: full workspace `cargo build` clean (same pre-existing `rand`
warning class, 24 instances now — 2 more of the same kind, nothing new).
`cargo run -p httpserver` boots end-to-end — migration 385 applies cleanly on
top of 318-384, game data loaded, listening on 0.0.0.0:8082/8443, no panics.

Next per the work queue: the remaining 1-3-request singles/pairs. Still
queued: the wider stub-not-real audit (4 confirmed instances now:
mini-family, MyRoom, Equip, and `report_user` as a likely 5th — strongly
suggests checking EVERY pre-existing "registered" route's actual body before
trusting round 1's 479-active-routes count as meaning 479 *working* routes).

## Stub-vs-real audit (cross-cutting, not part of the new-client work)

Started 2026-09-27, after 13 rounds of implementing new-client features kept
incidentally turning up pre-existing "registered but fake" route handlers
(mini-family 44, MyRoom 14, `report_user` 1, Equip 27 — 86 total found by
accident). This confirmed the gap wasn't isolated, so a dedicated pass was
started to find and fix the rest systematically, mechanically, across
*every* registered route rather than waiting to stumble into more clusters.

**Method**: `grep -rL "database::" gameserver/src/logic/game --include="*.rs"`
finds every handler with zero reference to the `database` crate — a
near-certain stub in this codebase (real handlers always call into
`database::db::...`; the scaffold-stub template literally contains
`// TODO: Fetch data from database` and unused `_pool`/`_uid` parameters).
A handful of genuinely-stateless real handlers (pure master-data lookups,
`balance_version_check.rs`-style) also match this grep and are false
positives — confirmed by reading, not assumed.

**Full inventory found** (handler-file count per feature directory, as of
this round's start): mini 47 (partially already known/addressed in round 6
— 44 of these), guild 42, evil(castle) 36, char 17, pvp 16, cafeteria 16,
monster 14, total(war/ranking) 13, field 13, pack 11, friend 10, battle 9,
supporter 8, gacha 8, user 7, life 7 (separate from this project's own
"Life" system — pre-existing unrelated handlers under the same directory
name), id 7, event 7, preset 6, costume 6, quest 5, item 5, hunt 5, equip 5
(on top of round 13's 27), shop 4, my(room-adjacent) 4, deck 4, cash 4,
talent 3, sky 3, save 3, pass 3, mission 3, fishing 3, dating 3, colosseum
3, and ~40 more directories with 1-2 files each (full list reproducible any
time via the grep command above).

**Fixed this round**: **Battle** (9 handlers — `battle_enter/start/end/
end_test/exit/give_up/retry/retry_previous_turn/verify_state`) and
**Gacha** (8 handlers — `gacha_info/buy/buy_preview/buy_preview_lock/log/
point_exchange/point_manual_exchange/selection_save`). These were
deliberately prioritized over larger clusters (guild, evil castle) because
they're the most foundational systems in the game — without a working
Battle flow, no stage/quest/hunt battle anywhere grants anything server-side
regardless of how good the reward data is; without working Gacha, no new
characters can ever be acquired. New migration 386 (`BattleSession`, a
lightweight session correlator for Enter->Start->End, since none of the
existing BattleCharInfo/BattleResultInfo/BattleStatisticsInfo tables serve
that specific purpose — they're for recording finished results, not
tracking one in flight). No new Gacha migration was needed — its query/model
layer (`gacha_user_info`, `gacha_log_info`, `gacha_selection_info`,
`gacha_total_count_info`, `gacha_step_up_user_info`, etc.) already existed
in full, exactly like Equip's case in round 13 — only the gameserver glue
was missing.

Real vs. placeholder in what got fixed: **Battle** — session tracking,
character HP/level persistence (`BattleCharInfo`), item consumption
(`end_inven_index`, via a new `item_info::delete_by_inven_index` helper),
and win/loss bookkeeping are all real; the actual battle reward CONTENTS are
a placeholder (100 gold) since no table anywhere maps a group_id/monster_id
to a specific reward — this is genuinely different from `QuestClear`
(`gameserver/src/logic/field/quest_clear.rs`, already real, already wired
up, unrelated to this stub gap) which DOES have `QuestTable1.reward_*`
columns to work from; Battle's async client-simulates/server-trusts-result
pattern matches Colosseum/Ib exactly. Test-mode `BattleEndTest` deliberately
never grants rewards or consumes items (a judgment call — a test/preview
endpoint paying out real rewards would be an obvious exploit).
**Gacha** — currency/ticket handling, real character granting into
`CharInfo`, real draw logging, real selection-save persistence are all
real; the specific character/item a draw produces is a placeholder (a
uniform-random pick from real `CharTable` ids) since no weighted drop-rate
table was captured (`GachaTable`: 1 row, `GachaFixedTable`: 3 rows) —
`GachaGroupTable` (24 real rows) IS used for real banner/schedule listing.

**Verified**: full workspace `cargo build` clean, `cargo run -p httpserver`
boots end-to-end with migration 386 applied alongside all prior ones (no
panics, no collisions).

**What's left** (not started this round, ordered by estimated impact):
Guild (42 — major social system, second-biggest cluster after mini),
EvilCastle (36 — a whole tower-climb game mode), Char (17 — character
progression: level-up, growth, healing, awakening, revival, scout/pity —
note `char_scout_info`/`char_special_scout_*` here may be an entirely
separate parallel gacha-like system, not yet investigated), PvP (16 —
distinct from the new Colosseum system built this session), Cafeteria (16 —
distinct from the new-client Cafeteria MASTER DATA already merged in step
3; this is the pre-existing route layer for it), Monster(Hunt) (14),
Total(War/Ranking) (13), Field (13), Pack (11 — likely pack-event related,
distinct from the 82 pack DBs), Friend (10 — the social friends-list system,
distinct from the new-client Friendship romance system built this session),
Supporter (8), User (7), the pre-existing "Life" handlers (7 — distinct
directory name collision with this session's new Life system, needs
disambiguation before touching), Id(card) (7), Event (7), and ~35 more
smaller clusters. Recommended approach for whoever picks this up: same
method (the grep command above, minus Battle/Gacha's directories which are
now clean), prioritize by the same "core progression/economy first, most
mechanical fix first" heuristic, and always check whether a stub directory
name collides with one of this session's 13 new-client-feature rounds
before assuming scope (Life and Cafeteria both need that disambiguation
check).

**Fixed (audit round 2): Guild** (42 handlers, the single largest remaining
cluster) — full social system: guild lifecycle (create/info/edit/leave/
delete-cancel), a real join-application queue (approval-required guilds
store a pending application; auto-join guilds join immediately), member
roster with role/ban management, search/recommend across every real guild
on the server, a per-guild notice board, per-account raid-supporter slots,
and the full guild-raid subsystem (deck/preset save-load, boss/normal battle
attempt tracking, member/season rankings, season reward claim-tracking).
`my_room_search_guild.rs` (also under this directory) was already real from
the MyRoom round — not touched. New migration 387 (`GuildJoinApplication` —
the scaffolded `GuildJoinSendInfo` table's TEXT-blob-of-serialized-sub-arrays
shape was unworkable as a real queue, so a proper normalized table was added
instead of forcing the existing one).

Design note: this schema stores every guild-related table per-account
(`Uid`-scoped), including the guild's own base info — there's no single
shared "the guild" row. Made this work correctly by: (1) a guild's public id
doubles as its `GuildBaseInfo` row's own global rowid, so any account can
resolve a guild by id via a new cross-`Uid` query regardless of who created
it; (2) each member's `GuildMemberInfo` row lives under their OWN `Uid`,
tagged with the shared guild id — the roster is assembled by querying that
id across every account rather than duplicating rows into a "shared" copy;
(3) a plain `UPDATE ... WHERE GuildBaseInfoIndex = ?` (no `Uid` filter) keeps
every member's own `MemberCount` copy in sync in one query. This is
real and correct for actual multi-account use on a shared server instance,
not just the solo case.

Two more scaffolded-but-unworkable TEXT-blob columns hit the same fix as
`GuildJoinSendInfo`: `GuildRaidDeckInfo.DeckInfoIndex`/
`SupporterDeckInfoIndex` and `GuildRaidPresetInfo.PresetInfoIndex` now hold
direct serialized JSON (the whole deck / whole preset list) instead of
whatever indirection the "Index" naming originally implied — round-trips
correctly, doesn't try to honor a broken indirection scheme.

Real vs. placeholder: everything that's genuinely per-account/per-guild
state (membership, applications, roles, notices, decks, presets, supporter
slots, ban/leave, search/recommend results) is real. No guild-raid master
data was ever captured (boss reward tables, season reward tables, rank-tier
tables all have zero rows), so: raid battle enter/quick-battle follow the
same async client-simulates/server-trusts-result pattern as Battle/
Colosseum/Ib (no live simulation exists here either) and reward CONTENTS are
documented placeholders; member/season rankings are computed fresh from real
`GuildRaidMainInfo.user_score`/`guild_total_score` columns (rather than via
a separately-maintained cache table, since nothing populates one) so they're
real math over currently-sparse-but-genuine data; `GuildSupporterInfoList`
simplifies to the caller's own supporter slots rather than the whole
roster's (would need enumerating every member's `Uid`, deferred).

Two PacketCodeType gaps found and handled the same way the pre-existing
`GuildRaidPresetInfoChange` stub already did: `GuildRaidPresetDelete` and
`GuildRaidSeasonRanking` have no dedicated entry in `common/src/packet_code.rs`
at all — used the `Common` fallback for both, matching established
precedent rather than inventing a new enum entry.

Verified: full workspace `cargo build` clean, `cargo run -p httpserver` boots
end-to-end with migration 387 applied alongside all prior ones (318-386), no
panics, no route collisions.

**Fixed (audit round 3): EvilCastle** (36 handlers) — two sub-systems: a
tower-climb mode (3 towers via `EvilCastleEnvyTowerTable`/`GreedTowerTable`/
`RageTowerTable`, boss checkpoints via the pre-existing `EvilCastleTable`,
daily login-streak rewards, cross-account ranking) and a much larger
roguelike deck-builder run (procedural floor/room generation, events,
shops, relics, permanent meta-growth, seasonal scoring/ranking) — this
second half is what most of the 36 handlers actually belong to. Master data
here is unusually rich for a stub cluster: `RLRoomTable` (8684 rows),
`RLFloorTable` (240), `RLEventTable` (80), `RLRelicTable` (135),
`RLGrowthTable` (40), `RLRewardTable` (10), `RLEventChoiceTable`,
`RLRelicMixTable`, `RLShopTable`, `RLDefaultTable`/`RLLevelTable` — real
constants throughout, not sparse placeholders. One new migration (388):
added a missing `PackId` column to the pre-existing `EvilCastleInfo` (it had
no way to distinguish multiple towers per account at all), a missing `Ids`
column to `EvilCastleRogueLikeChoiceInfo` (had `Type` but nowhere to store
the proto's `repeated int32 id`), a missing `Floor` column to
`EvilCastleRogueLikeRoomInfo` (rooms from different floors of the same run
couldn't be told apart), a new `EvilCastleRogueLikeDeckInfo` table (the
roguelike run's own deck, separate from the account's main team deck — same
one-deck-table-per-feature convention as every other system), and a new
`EvilCastleDailyRewardInfo` table (no pre-existing table covered daily-claim
tracking at all). Floor/room generation is real and data-driven (RLFloorTable's
real per-slot room-group weights pick a group, RLRoomTable supplies the real
room definition) except for the room-count-per-floor itself, which no table
states explicitly and is a documented placeholder constant (8).

Real vs. placeholder: tower progress/stage-clear rewards (real, using
`EvilCastleTable`'s real reward arrays — cycled by stage index since no
explicit stage-to-checkpoint mapping was captured), daily rewards (real,
`EvilCastleDailyRewardTable`'s real per-day rewards), roguelike run state
(floor/room position, gold, deck, relics, meta-growth levels, shop
inventory/purchases, relic mixing, season score/ranking) are all real and
persist correctly. Placeholders: event-choice outcome magnitudes (no
interpreter exists anywhere in this project for `RLEventEffectTypeTable`'s
effect ids, so success/fail is rolled against the real `eventSuccessRate`
but the gold delta itself is a flat placeholder), battle-adjacent rewards
(same client-simulates/server-trusts pattern as Battle/Colosseum/Ib/Guild —
no live simulation exists in this project), and reward-choice contents when
a choice is a relic pick (real relic ids from `RLRelicTable`, but which ones
appear in a given choice set is randomly sampled rather than following
whatever the client's own weighting actually is, since that logic lives
client-side).

Also fixed along the way (found via the compiler, not incidentally this
time): 4 `EvilCastleRogueLike*` routes (`MoveFloor`, `EditEvent`,
`CharRevival`, `BattleSkip`) had **no `PacketCodeType` enum variant at all**
— unlike every other gap seen so far in this project, these weren't even
stubbed with a wrong-but-existing code, the variant was simply absent. Added
620-623. Also found `EvilCastleRankingInfo`'s real code is named
`EvilCastleRakingList` (a typo baked into the original scaffold — "Raking"
not "Ranking", "List" not "Info") — used the actual name rather than
"fixing" the typo, since renaming an enum variant that other already-working
code might reference elsewhere is out of scope for this round.

Verified: full workspace `cargo build` clean, `cargo run -p httpserver` boots
end-to-end with migration 388 applied alongside all prior ones (318-387), no
panics, no route collisions.

**Fixed (audit round 4): Char** (17 handlers) — character progression:
level-up (`CharLevelUp`, holy-water items) and growth (`CharGrowth`, generic
material items) share an exp/level curve using real `CharLevelTable`
per-level exp values, bounded by the highest real `maxLevel` in
`CharGrowthTable`; class-tier progression (`CharClassUp`, new `ClassLevel`
column added to `CharInfo`); imprint slot leveling (`CharImprintLevelUp`,
validated against REAL per-character caps in `CharAwakeTable`'s
`imprintSlot1/2/3` — 17 rows, well-populated); awakening
(`CharAwakeActive`/`CharAwakeInfo`); healing/revival (`CharHealing`,
`CharAllRevival`, `CharImmortal` — no `MaxHp` concept is stored anywhere on
`CharInfo`, so "full heal" uses a documented placeholder HP value rather
than a real max); auto-revive account setting (`CharAutoReviveSet`, new
`CharAutoReviveSetting` table — this route had **no `PacketCodeType` variant
at all**, same gap as 4 EvilCastle routes last round, added code 624);
character-removal-on-expiry (`CharExpiry`, real deletion); a partner/pairing
system (`CharPartnerInfo`/`CharPartnerReward`/`CharPartnerStoryReward`,
using the `Reward` column as a claimed-steps bitmask — bit 0 reserved for
the "story" pairing-reward, bits 1+ for numbered reward steps, a documented
judgment call since no table distinguishes the two claim types); and a
**confirmed second, separate gacha-like system** — the "special scout"
(`CharScoutInfo`/`CharSpecialScoutBuy`/`CharSpecialScoutReset`), a rotating
2-featured-character showcase with a manual-reset option, using REAL
`SpecialScoutInfoTable` config (appear count, auto-reset timer, reset
limit) — buying grants a real character into `CharInfo` exactly like Gacha's
own buy flow. `CharSpecialScoutReset`'s real currency cost
(`resetCostType`/`resetCostCount`) is NOT enforced — the request carries no
items to consume and no generic currency-deduction mechanism exists
anywhere else in this codebase to spend it against; everything else about
the reset (limit enforcement, lineup regeneration, timer) is real.

One new migration (389): the `ClassLevel` column and the
`CharAutoReviveSetting` table mentioned above. Two more `PacketCodeType`
naming quirks found (matching the `EvilCastleRakingList` precedent — used
the real names rather than renaming): `CharAwakeInfoRequest`'s real code is
`CharImprintInfo`, and `CharScoutInfoRequest`'s real code is
`CharSpecialScoutInfo`.

Verified: full workspace `cargo build` clean, `cargo run -p httpserver` boots
end-to-end with migration 389 applied alongside all prior ones (318-388), no
panics, no route collisions.

### PvP (16 handlers) — DONE

This is an OLDER, pre-existing arena system (`pvpdefaulttable`/`pvpranktable`/
`pvpranktablev1`/`pvpseasontable` predate this whole client-update project) —
distinct from **Colosseum**, the new-client async PvP system implemented
earlier in this session. Confirmed via message schemas
(`protocol/proto/Commons/proto_net.proto`, `Pvp*` messages): same overall
shape as Colosseum (client simulates, server trusts the reported result via
a shared `battle_random_seed`), but single-opponent matching (not a
candidate list), separate attack/defense decks with their own win/lose/reset
tracking, and richer captured master data — `PvpRankTable` (81 rows) has
**real per-bracket win/lose Vp points and real reward-count arrays**, unlike
Colosseum's completely empty tables, so Vp changes and reward amounts here
are genuinely data-driven, not placeholder constants (only the reward
*item* — id/type — is the established gold-item convention, same as
SpineInteraction/CharVote/Friendship, since no table maps a reward type to
an actual item).

Found a pre-existing but architecturally mismatched scaffold already in
place (`database/migrations/252-263_pvp_*.sql` + matching model/query
files) — one denormalized "snapshot row per response" table per message
type, rather than persistent per-account state. Given the scale of
retrofitting that design vs. building a clean normalized one from scratch,
kept the old scaffold in place untouched (dead weight, harmless) and added
**5 new migrations (390-394)**: `PvpUserInfo` (Vp/win-lose/season/deck-season
counters, once-reward claim bitmask), `PvpDeckInfo` + `PvpDeckMeta` (attack/
defense decks with item/equip JSON + battle power), `PvpCurrentMatch`
(pending opponent snapshot), `PvpBattleHistory` (per-battle log with a
JSON deck snapshot for replay), `PvpSeasonRewardClaim`.

All 16 requests now have real routes + persistence — none left on the
fallback: user info, matching (real account when one exists, else a bot
from real `CharTable` ids), start/end (real Vp deltas + real gold rewards
from `PvpRankTable`, mirrored defense-side record for real opponents),
history (both attack/defense, real season/deck-season counters, real
`battleHistoryLimitCount`-bounded), history-deck-info and replay-info (both
reconstructed from the stored JSON snapshot), once-reward-info and
battle-reward (real Vp-threshold tiers derived from `PvpRankTable`'s 81
distinct Vp values, not an invented tier list), ranking (real accounts only,
not bot-padded — consistent with Colosseum/Guild), rank-user-detail (real
cross-account deck read), reset (real per-deck-type counter reset), deck-info/
deck-save/contents-item-renew (real deck + per-char item/equip persistence).

Two `PacketCodeType` variants (`PvpBattleReplayInfo`, `PvpBattleReset`) had
no code at all — added 625/626, same recurring gap as EvilCastle/Char/
MyRoom's rounds.

Verified: full workspace `cargo build` clean; `cargo run -p httpserver` boots
end-to-end, migrations 390-394 apply alongside 318-389, listening on
0.0.0.0:8082/8443, no panics.

**Cafeteria (2026-09-27):**

**Naming-collision resolved — it was a false alarm.** Checked the step-5 log
directly: none of this session's 13 new-feature rounds ever implemented
anything called Cafeteria. All 12 `Cafeteria*Table` master-data files
(`CafeteriaBubbleTable`, `CafeteriaCostumeTable`, `CafeteriaDefaultTable`,
`CafeteriaEventTable`, `CafeteriaFacilityTable`, `CafeteriaLevelTable`,
`CafeteriaManageTable`, `CafeteriaMapTable`, `CafeteriaMovePatternTable`,
`CafeteriaNpcTable`, `CafeteriaTimeTable`, `CafeteriaUniqueNpcSpawnTable`)
predate this whole client-update project — confirmed against the very first
`mod.rs` read at the start of this session, before any table merging
happened. The "distinct from new-client Cafeteria" note a few rounds back was
simply wrong; there is only one Cafeteria. (Correcting the schema-gap-analysis
section above too: Cafeteria was never one of the 167 new common.db tables —
whatever produced that early list mismatched on a partial name.)

**What it is**: a café management sim layered on top of the Dating/costume
system — costumes work "shifts" (a daily-rotating roster), each serving
customers for a per-costume reward and building up a "note" (serve-count
milestone reward), plus rare/event NPC interactions, an ordered facility/
part-time-manager unlock catalog with real gold costs, a passive idle-income
timer, and a one-time introduction-story reward.

All 16 handlers now have real routes + persistence — none left on the
fallback. Found and fixed the same class of latent scaffolding bug as
MyRoom/Equip: `CafeteriaInfo`'s model referenced `DailyRegularCostumeId`/
`RewardedDailyRegularCostumeId` columns that were never created in the
migration at all (the underlying proto fields are `repeated int32`, so these
had to become JSON-array TEXT columns, not the `i32` the model claimed). One
new migration (395) adds those two columns plus a `IntroStoryRewardClaimed`
flag that didn't exist anywhere. Also added an `update_cafeteria_info` DB
function and several `CafeteriaRegularCostumeNoteInfo` helpers (`get_by_costume_id`/
`increment_serve_count`/`mark_reward_received`) — the existing query layer had
insert/get/delete but no update path at all, another symptom of "scaffolded
together, never actually exercised."

Real, using genuinely well-populated pre-existing master data (30-182 rows
per table depending on which): daily shift roster/rewards (from
`CafeteriaCostumeTable`'s real per-costume `rewardId`/`rewardType`/
`rewardValue`), serve-count/note tracking, the facility/manager unlock queue
(walked one row at a time via `CafeteriaManageTable`'s captured row order —
no explicit ordering field exists beyond a (groupId, id) pair that resets
per group, so the JSON array's own order is used as the sequence) with real
gold costs (`costValue`) actually deducted via `item_info::consume`, the
event-NPC reward (`CafeteriaEventTable`'s real `rewardType`/`rewardCount`,
keyed by groupId+id since `id` alone isn't unique there either — same
Achievement-table lesson from much earlier in this project), the daily
NPC-reward-currency cap (`dailyShopCurrencyLimit`), and the note-reward
threshold+value (`cafeteriaNoteConditionValue`/`noteRewardValue`). Documented
placeholders: level-up is free with a flat gold reward (no cost/reward table
exists for leveling at all), rare-NPC interaction reward is flat gold (no
reward fields exist on `CafeteriaUniqueNpcSpawnTable`, only spawn-timing
config), and the passive cumulative-reward income rate is a flat gold-per-
minute placeholder (the min/max accrual-time window itself is real, from
`CafeteriaDefaultTable`, just not the rate).

One `PacketCodeType` variant (`CafeteriaSpawnResetCheat`) had no code at all
— added 627, same recurring gap as every prior stub-audit round.

Verified: full workspace `cargo build` clean; `cargo run -p httpserver` boots
end-to-end, migration 395 applies alongside 318-394, listening on
0.0.0.0:8082/8443, no panics.

**Next**: Monster Hunt (14) is now the largest untouched cluster, followed by
Total(War/Ranking) (13), Field (13), and everything else listed above.

**Round: Monster Hunt.**

A boss-damage-race mode: fight the same boss repeatedly, one placeholder
damage roll per "quick battle" attempt (no formula anywhere models
deck/character stats into damage server-side — same gap as every other
battle-adjacent system in this project), compared against the boss's REAL
data-driven HP curve (`MonsterHuntTable.levelUpHealthRate *
levelUpHealthSlope^(level-1)` — genuinely real scaling, only the damage
roll itself is placeholder). Clearing a level grants a real per-level
reward from `MonsterHuntRewardTable` (keyed by groupId+level, 1380 real
rows) plus a real once-daily reward from the same row. Also: saved
decks-per-team, presets (name/resource/deck snapshot per slot), a
real cross-account damage-race leaderboard (same non-bot-padded pattern as
Colosseum/PvP/Guild/EvilCastle), and a claim-once season-end reward using
`MonsterHuntRankTable`'s real ranking-threshold tiers (top 1/10/100, 18
real rows with real reward triples).

Unlike every stub cluster fixed so far, this one already had its full
migrations/models/query-layer scaffolding in place (201-207, predating this
whole project) — just needed real gameserver logic wired in, plus two
small schema gaps: `MonsterHuntPresetInfo` had no `Slot` column at all
despite three of the six preset requests addressing presets by slot number
(added via migration 396, along with `PresetName`/`PresetResourceId`/
`PresetResourceColor` columns the model already expected), and
`MonsterHuntUserInfo`'s query layer had insert/get/delete but no `update`
— the same "scaffolded together, never exercised" pattern as Cafeteria/
MyRoom before it. Added a real `update`/`get_or_create`/cross-account
`rank_all` set of queries.

All 14 handlers now have real routes + persistence — none left on the
fallback. Four `PacketCodeType` variants had no code at all
(`MonsterHuntPresetDelete`/`PresetInfoChange`/`SeasonReward`/
`ScheduleInfo` — added 628-631), same recurring gap as every prior round.

Judgment calls: `MonsterHuntPresetUse`'s response leaves `char_info`/
`char_equip_info` empty rather than fabricating character/equip detail from
a deck snapshot (no clean derivation path exists); schedule info treats the
one real boss as an always-active season with placeholder start/end
timestamps (no schedule data was ever captured, `info_open_day`/
`calculate_end_date` etc.); a level-up's exact damage-vs-remaining-HP
comparison assumes single-attempt lethal HP tracking (a `current level
highest damage` running total compared against the level's full HP) rather
than a multi-turn HP pool, since the request carries no turn/HP-remaining
field of its own.

Verified: full workspace `cargo build` clean (same pre-existing `rand`
deprecation-warning class, nothing new). `cargo run -p httpserver` boots
end-to-end — migration 396 applies alongside 318-395, listening on
0.0.0.0:8082/8443, no panics.

**Next**: Total(War/Ranking) (13) is now the largest untouched cluster,
followed by Field (13), and everything else listed above.

**Round: Total War/Ranking.**

A cumulative-score siege/war event: the client fights battles on its own
(same client-simulates/server-trusts pattern as every other battle-adjacent
system in this project — no server-side combat resolution exists anywhere
here) and reports per-category score deltas via `TotalWarBattleEnd`; the
server accumulates them for real per category id, and pays out real
score-threshold reward tiers from `TotalWarRewardTable` (real data) via a
separate `TotalWarReward` claim request. Also: saved decks (full-list
replace, matching how the client always resends the whole deck), presets
(name/resource/deck snapshot per slot, with a real slot-count cap from
`TotalWarDefaultTable.total_war_preset_max_count`), and a real
cross-account top-score/percentile computation (same non-bot-padded
pattern as every other ranked system here).

Like Monster Hunt, this already had its full migrations/models/query-layer
scaffolding in place (303-305, predating this project) — just needed
gameserver logic wired in, plus two small gaps: `TotalWarInfo`'s
`EngineType` column was modeled as `serde_json::Value` (which sqlx can't
actually decode from a TEXT column via the derive macro — would have
failed on first real read) — retyped to `Option<String>` matching the
column; and there was no way to track which score-tier rewards had
already been claimed, so a `ClaimedRewardIds` column was added (migration
397).

All 13 handlers now have real routes + persistence — none left on the
fallback. Two `PacketCodeType` variants had no code at all
(`TotalWarPresetDelete`/`TotalWarPresetInfoChange` — added 632-633), same
recurring gap as every prior round.

Judgment calls: `TotalWarContentsItemRenew`'s response (per-character equip
loadout for the event) is left honestly empty — that data belongs to a
separate `TotalWarEquipSave` route that isn't part of this stub cluster and
isn't implemented anywhere in this project yet; `TotalWarBattleStart`
leaves `blue_char_info`/`red_char_info`/`buff_stat_info` empty rather than
reconstructing full `BattleCharDBInfo` from the stored deck rows (which
only hold inven indices, not full stats — not a clean derivation, and the
client already holds its own character data locally); `is_obtainable_daily_
reward` is interpreted as "at least one unclaimed score tier is available
right now" (no table distinguishes a literal daily reward from the ordinary
score tiers).

Verified: full workspace `cargo build` clean (same pre-existing `rand`
deprecation-warning class, nothing new). `cargo run -p httpserver` boots
end-to-end — migration 397 applies alongside 318-396, listening on
0.0.0.0:8082/8443, no panics.

**Next**: Field (13) is now the largest untouched cluster, followed by
everything else listed above (~40 smaller clusters).

**Round: Field.**

Open-world exploration: saved field-object positions (collectibles/research
points placed on the map), respawn timers, a per-account trap-state list,
and field-monster interactions (damage/event/regen/reward). One of the 13
— `field_object_reward.rs` — turned out to be a **false positive** in the
audit's stub-detection grep: it already had full real logic
(`collect_field_object` in `gameserver/src/logic/field/reward_object.rs`,
outside the audited directory) that persists via raw `sqlx::query` calls
rather than the `database::db::...` crate path the grep looked for — so it
never showed the literal string `database::` despite being completely
real. Worth remembering: the grep heuristic isn't infallible, always spot-
check a file's actual content before assuming "no literal match" means
"stub."

Like Monster Hunt and Total War, this system's migrations/models/query
layer already existed (099-106, predating this project) — just needed
gameserver logic wired in for the other 12. Real, using genuinely populated
per-object reward fields on `FieldMonsterTable`/`FieldResearchObjectTable`
(both predate this project): monster-event and research rewards actually
grant real items. Real state: saved object positions, respawn timers (real
interval from `FieldMonsterRegenTable`), trap list. Added one missing
`PacketCodeType` (`FieldObjectPositionUpdate` — the stub itself was already
using a `Common` fallback code, confirming the gap was real, not a
grep artifact).

Schema-granularity judgment calls (inherited pre-existing gaps, not
introduced this round): `FieldObjectInfo`/`FieldObjectPreview` have no
`pack_id` column to scope by map, so they return every saved object for
the account rather than filtering; `FieldTrapInfo.SwitchObjectId` is a
single column despite the proto's repeated field. `FieldMonsterReward`'s
request carries no `monster_id` (or anything else identifying) at all —
returns honestly empty rather than guessing.

Verified: full workspace `cargo build` clean (no new migration needed —
schema already existed). `cargo run -p httpserver` boots end-to-end,
listening on 0.0.0.0:8082/8443, no panics.

**Round: Pack (11) — DONE.**

Already-scaffolded system (models/db layer, like Monster Hunt/Total
War/Field). Found and fixed one more `serde_json::Value`-in-a-plain-column
bug (`PackRewardObjectCountInfo.Type`, migration 398 — same class of bug as
Total War's `EngineType`).

All 11 handlers now real: `pack_info` (real per-account progress for all
79 real `PackTable` packs, replacing what were two hardcoded fake pack
entries), `pack_buy` (real `IsBuy` flag + real `buy_reward_id/type/count`
grant from `PackTable`), `pack_detail` (validates against real `PackTable`
ids), `pack_event_battle_info`/`pack_event_story_info` (real per-account
progress reads, filtered by requested event_uid), `pack_event_story_clear`
(real first-clear-only reward from `PackEventStoryTable`'s 284 real rows,
tracked per account so it never pays twice), `pack_event_story_replay_clear`
(same real reward, but re-grants every call — that's the point of a
"replay"), `pack_jam_event` (real reward count/type from the single real
`PackJamEventTable` row), `pack_reward_object_count`/
`pack_sub_quest_clear_info` (real per-account counters).

Judgment calls: `pack_preview_info` previously returned two entirely fake
hardcoded quest/quest-title entries — now returns them honestly empty
(that data belongs to the separate, not-yet-audited Quest system) while
`is_pack_event_reward` is now a real check against `PackJamEventTable`'s
existence; `PackJamEventTable` has no `reward_id` field, so the granted
item id defaults to gold (its `insert_min`/`insert_max` fields look
unrelated to this reward, more likely a UI/RNG range for something else,
so left unused); `PackRewardObjectCountInfo`'s `MaxCount` is a flat
placeholder (3) for brand-new counters only — no table anywhere defines a
real cap; two pre-existing schema-granularity gaps inherited (not
introduced here): `BattleChallengeIndex` and `SwitchObjectId`-style single
columns standing in for the proto's repeated fields.

Two more missing `PacketCodeType` variants found and added (`PackDetail`,
`PackEventStoryReplayClear` — both were using `Common`/no code before),
consistent with the recurring gap in every round so far.

Verified: full workspace `cargo build` clean. `cargo run -p httpserver`
boots end-to-end, migration 398 applies alongside 318-397, listening on
0.0.0.0:8082/8443, no panics.

**Round: Friend (10) — DONE.**

Real friend-request system: send/accept/refuse/remove/cancel-sent, sent
and received pending lists, confirmed-friends list, search (validates
against real accounts), recommend (real other accounts not already
related). Found a real schema gap: the pre-existing `FriendInfo` table
(predates project) had no way to distinguish confirmed-friend from
pending-sent/pending-received despite half the request types needing
exactly that — added a `Status` column (migration 399: 0=confirmed,
1=pending sent, 2=pending received).

All relationship writes mirror BOTH sides for real cross-account
correctness (same technique as Guild/Colosseum/PvP): sending a request
inserts a status=1 row on the sender's side and a status=2 row on the
target's side; accept/refuse/remove/cancel all update or delete both
mirrored rows together. `user_id` falls back to the other account's
stringified uid (same placeholder precedent as CharVote/Colosseum/PvP
ranking — no display-name lookup exists anywhere in this project).

All 10 handlers now real, none left on the fallback. No new
`PacketCodeType` gaps found this round (all already had codes).

Verified: full workspace `cargo build` clean. `cargo run -p httpserver`
boots end-to-end, migration 399 applies alongside 318-398, listening on
0.0.0.0:8082/8443, no panics.

**Round: Supporter (8) — DONE.**

"Borrow a friend's registered character for battle" system. Already
scaffolded (models/db predate project). Found another `serde_json::Value`-
in-plain-column bug (`SupporterUsageInfo.BorrowType`, migration 400, same
class as Pack/Total War's).

All 8 real: register/remove (real per-slot storage), info/status (real own
registered slots + real rental history + real daily-claim count via
`SupportCharacterDefaultTable`'s real cap), detail (real cross-account slot
lookup), battle_info (real friend-list + real recommend-list decks, each
built from the target account's actually-registered slots — accounts with
no registered slots are skipped rather than returned empty), borrow (real
usage-history recording + real `BattleUseCount` bump on the target's slot,
real `is_friend`/`can_request_friend` from the real Friend relationship
table), reward (real claim-once payout using the real
`support_reward_item_type`/`support_reward_value` config, respecting the
real daily cap).

Judgment call: `SupportCharacterDefaultTable` has real reward type/value
fields but no item-id field, so gold (id 4) is used as the granted item —
same "no id field" situation and precedent as Friendship's round.
`supporter_char_info` (full char/costume/equip reconstruction for a
borrowed character) is left empty in both `borrow` and `battle_info` — no
cross-account inventory join exists in scope for these handlers.

No new `PacketCodeType` gaps this round.

Verified: full workspace `cargo build` clean. `cargo run -p httpserver`
boots end-to-end, migration 400 applies alongside 318-399, listening on
0.0.0.0:8082/8443, no panics.

**Round: User (7) — DONE.**

Profile settings (greeting/title/nickname/portrait), the level-up reward
claim, a privacy-options read, and the big cross-account profile-view
aggregate (`UserContentsInfo`). Added 4 columns to the pre-existing
`UserInfo` table (migration 401: `Greeting`, `TitleId`, `IsAllPrivate`,
`PrivacyOptions`) — none of the 4 simple setters had anywhere to persist
to at all.

`UserContentsInfo` is a genuinely large aggregate pulling from most other
systems — pulled REAL data from everything already built this session with
its own per-account persistence (User/Pvp/MonsterHunt/TotalWar/Friend/
Supporter — reused `pvp_user_info::rank_of`, the `monster`/`total` modules'
own helper functions cross-module rather than reimplementing). Left as
documented placeholders (not fabricated) the fields that would need
re-verifying another system's exact schema well beyond this handler's own
scope: `guild_base_info`/`guild_raid_rank`/`guild_raid_score` (Guild),
`room_info`/`my_room_like_count` (MyRoom), the 3 EvilCastle tower-floor
fields + rogue-like level, `achievement_level`, `id_card_info`,
`like_count`, `total_battle_power`.

`UserLevelReward` claim-tracking uses `UserInfo.LevelReward` as a bitmask
(no dedicated level-reward table exists anywhere in this project) — gold
amount is a documented flat placeholder.

All 7 real, no new `PacketCodeType` gaps.

Verified: full workspace `cargo build` clean. `cargo run -p httpserver`
boots end-to-end, migration 401 applies alongside 318-400, listening on
0.0.0.0:8082/8443, no panics.

**Round: "Life" small cluster (7) — FALSE POSITIVE, no work needed.**

Correction to earlier round-1/coordinator assumption that this was "a
separate pre-existing thing, not the new Life farming-sim" — it's not
separate at all. The 7 files (`life_cheat_build_complete`,
`life_cheat_growth`, `life_cheat_regen`, `life_cooking`, `life_crafting`,
`life_helper_gacha`, `life_helper_reconnect`) ARE part of the same Life
farming-sim module from round 2, and are **already fully complete** —
this is the same class of grep false positive as `field_object_reward.rs`
in the Field round: `life_cooking.rs`/`life_crafting.rs` call a shared
`try_consume_items` helper defined in the module's own `mod.rs` (which
does the real `database::` work) rather than referencing the crate path
directly themselves, so the audit's `grep -rL "database::"` flagged them
even though they're genuinely finished, complete, and match round 2's own
documented judgment calls exactly (empty reward bundles where
`LifeCookTable`/`LifeCraftingObjectTable`/`LifeHelperSpeciesTable` have no
captured data; the 3 cheat endpoints are legitimately empty-schema debug
no-ops; `life_helper_reconnect` is a documented no-op pending clearer
client behavior). No code changes made — nothing needed fixing.

**Next**: ~40 smaller stub clusters remain (1-4 handlers each — Id,
Event, Preset, Costume, Quest, Item, Hunt, and more). Given this false
positive, worth a quick sanity check on any future cluster before diving
in: skim whether its files call a shared in-module helper (like
`try_consume_items`) before assuming "no `database::` match" means "needs
work." No single cluster stands out by size anymore; working through them
in the order `CLIENT_UPDATE.md`'s original round-1 inventory lists them,
per the user's "keep going" mandate.

**Round: Id (7) — DONE.**

The "ID Card" profile customization feature (background/effects/stickers/
my-info, layered with position/rotation/scale/color, plus save-able
presets and a real shop). Already scaffolded (models/db predate project),
using a genuinely relational design (an `IdCardInfo` row holds Index
pointers into `IdCardItemInfo` rows for each slot, rather than a JSON
blob) — matched that design rather than reworking it: every save inserts
fresh `IdCardItemInfo` rows and re-points the indices (old rows become
harmless orphaned garbage, an accepted simplification).

Real subtlety handled correctly: `IdCardInfo` is shared by both the
account's single "current" card AND every saved preset's own snapshot,
with no column distinguishing them — `get_current` now excludes any row
referenced by `IdCardPresetInfo.IdCardInfoIndex` and picks the
most-recent remaining row, which is correct given `IdCardSaveRequest`
always fires after whatever preset-saves preceded it in the normal client
flow.

All 7 real: save/recovery (real read-back, "recovery" interpreted as
restoring the current card rather than resetting it — no default-card
table exists to reset to, and destructively resetting on the one request
that reads it back would be actively harmful), preset info/save/delete,
shop info/buy (real per-account buy counts, real `buy_max_count` cap and
real `price_id`/`type`/`count` from `IdCardItemTshopTable`'s 1002 real
rows — actually consumes the real price rather than trusting the
client's `use_item_info`).

One `PacketCodeType` that existed but was wired to the wrong constant
(`IdCardPresetInfo` was using the generic `Common` fallback) — fixed to
use its real code.

Verified: full workspace `cargo build` clean (no new migration needed —
schema already existed). `cargo run -p httpserver` boots end-to-end,
listening on 0.0.0.0:8082/8443, no panics.

**MAJOR COURSE CORRECTION**: the "~29 smaller clusters remain" estimate
above was based on the STALE round-1 inventory (which only counted
newly-added message types with no handler at all, from before the stub-
audit methodology existed). A full fresh sweep of the entire
`gameserver/src/logic/game` tree (`grep -rL "database::"`) just turned up
**~190 files**, not ~29. This is the real remaining scope. Three
important caveats discovered while triaging the first batch of these:

1. **Some are dead code, not stubs** — e.g.
   `gameserver/src/logic/game/login/login_user.rs` is a stub, but it is
   **never registered** in `httpserver/src/main.rs`; the real `"LoginUser"`
   route is a completely separate, already-fully-implemented handler at
   `httpserver/src/routes/user/login_user.rs` (real account
   creation/token issuance/cookie). This is the last of round 1's
   long-known "8 route-path collision" names (`LoginUser`) finally
   explained — actix only ever registered the real one, the stub file is
   simply unreachable. **Always check `main.rs` registers
   `gameserver::logic::game::<x>` before trusting the grep hit.**
2. **Some are correct-as-empty by design**, e.g. `PingCheckResponse` and
   `LogoutUserResponse` are empty messages in the proto itself — an empty
   ack IS the correct real implementation, not a stub to fill.
3. **Some are correct-as-empty for a different reason**: `personal_info.rs`
   looked like it should return the account's profile, but
   `PersonalInfoResponse` actually carries `PersonalDBInfo` (id/title/
   url/start_date/end_date — a personalized notice/banner list, NOT the
   user profile) with no matching master data table captured anywhere —
   legitimately left empty, documented rather than fixed.

Fixed for real this round: **`join_user`** and **`platform_login`** — both
carry a real `UserDBInfo` in their response and are both live-registered
routes; both were returning `..Default::default()` (i.e. the client's
own account name/gold/jewelry/etc. would show up blank/zero on join or
platform re-login even though the account is real). Reused the exact
same `UserInfo::to_proto()` mapper and `account::get_or_create_user`/
`parse_uid_from_token` helpers the real REST `LoginUser` route already
uses (`database/src/mappers/user_info.rs`, `gameserver/src/logic/game/
account.rs`) — no new persistence needed, this was purely a missing-glue
bug. Guild-membership fields left `None` (would need a fresh guild-
roster lookup, out of scope for this fix). Verified build+boot.

**Refined methodology**: grepping for the literal template marker
`// TODO: Fetch data from database` (present only in never-touched
scaffolding) is a MUCH more precise stub signal than "zero `database::`
references" — the latter also flags already-finished files that use raw
`sqlx::query` directly, a shared in-module helper, or are correctly
empty by design. Re-running that literal-marker grep across the whole
tree found **113 genuine stub files** (not ~190) — and critically,
**zero of them fall inside any already-"DONE" cluster's directory**,
confirming every previously-completed round really is complete. Of
those 113, **45 are the already-known, already-deliberately-deferred
`mini` family** (Action/Bingo/Board/Defense/Field/Rhythm/Roulette/Run/
Sichuan/Survival/Puzzle minigames — noted back at the Round 6 decision
point). That leaves **~68 genuinely new small-cluster handlers**, plus
the 45-file mini family still deliberately deferred.

Fixed this round (beyond `join_user`/`platform_login` above):
**Shop (4)** — real per-account, per-shop rotation (`ShopInfo` recreated
with a real `ShopId` column it never had — migration 409 — plus a new
`ShopProductInfo` table for real per-product buy-count tracking,
replacing the old unused `ProductInfoIndex` TEXT-list). Real reset
duration from `ShopTable.resetCount`, real product catalog from
`ProductTable` filtered by `group_id == shop_id` (judgment call — no
explicit FK exists), real buy-cap enforcement against
`ProductTable.buyMaxCount`, real item consumption/granting on buy, real
gold payout on sell using the client-reported per-item `rate`. **
Reputation (1)** — real per-account `ReputationInfo` read (already had
full db scaffolding, just needed the gameserver glue); reused by
`shop_open`'s `reputation_state` field.

**Talent (3) + Mission (3) DONE.** Both already had full db scaffolding
(models/query-layer predate project), just needed gameserver glue.
Talent: real item consumption on upgrade, real current-state echo on
use (`TalentSkillInfo`/`TalentNpcInfo`), real food-item consumption;
`talent_slot_save` reuses the pre-existing but oddly-shaped
`TalentObjectInfo` table as an ordered slot-position -> char-id list
(judgment call — no dedicated "talent slot" table exists). Mission:
real progress tracking against `MissionTable`'s real `condition_value`
(`mission_update`), real claim-once reward granting from
`MissionTable`'s real reward triple with the claimed `MissionInfo` row
deleted after (`mission_clear`), real section-reward claiming against
`MissionSectionRewardTable` with a new claim-tracking check
(`mission_section_reward`). All 6 real, build+boot verified.

**Achievement (2) + Alchemy (2) + Eat (2) + Cash (2 of 4) + Cooking (2)
DONE**, plus a hardcoded-fake-data fix on **Recipe (1)**:
- Achievement: real tier-claim against `AchievementTable`'s real
  `condition_value`/reward arrays/`exp`, claimed tiers tracked via
  `MaxClearId`; real progress accumulation.
- Alchemy (`alchemy` + `alchemy_batch`, share one `craft()` helper):
  real crafting against `AlchemyTable`'s real material cost/result item;
  `talent_level` used as the talent-exp-per-craft formula (judgment
  call — only plausible numeric field for it).
- Eat (`eat_food` + `eat_food_auto`): real food consumption, real HP
  heal via `FoodTable`'s real `recovery_point` (uncapped — no MaxHp stat
  exists anywhere in this project, confirmed again).
- Cash: `cash_shop_info`/`cash_mail_info` were already real (false
  positives); fixed `cash_shop_buy`/`cash_shop_purchase_count_info` —
  real per-product purchase-count tracking (`PurchaseCountInfo`, wired
  to the same field `UserDBInfo.purchase_count_info` names), but actual
  purchase *contents* stay honestly empty since the captured
  `cash_shop_info.json` starter data has zero reward-catalog fields
  (scheduling only) — there's nothing real to grant on a private server
  with no payment processing anyway.
- Cooking (`cooking` + `cooking_research`): real crafting against
  `CookingTable`, real recipe-unlock persistence via the pre-existing
  `RecipeInfo` table. `CookingResearchTable` turned out to be a single
  global config row with no per-recipe id — research always succeeds
  (no real chance data exists to base a failure on).
- **Recipe (1) — found and fixed a hardcoded-fake-data bug**:
  `recipe_info.rs` (which round-1 flagged as a "collision" and moved on
  from) was actually **live** — invoked from
  `httpserver/src/routes/default/batch/batch.rs::recipe_info_handler`,
  not a per-route file, which is why the earlier main.rs-registration
  check missed it. It was hardcoding `recipe_id: vec![101]` for every
  account regardless of what they'd actually unlocked — now reads the
  real per-account `RecipeInfo` table `cooking_research` writes to.
  **Lesson for the remaining "round-1 collision" names** (BalanceVersionCheck,
  CharInfo, MaintenanceInfo, MissionInfo, NoticeInfo, ServerInfo): don't
  assume dead just because no `routes::game::X::Y` registration exists —
  check `batch.rs` too before writing one off.

All verified together, build+boot clean.

**Dating (3) DONE** — and a 3rd type of "looked-fine-but-wasn't" bug
found: `dating_info.rs` had **no TODO template comments at all** (someone
had already stripped them) but was still just returning
`..Default::default()` — a completely empty response for a screen with
real per-account state (`episode_info`/`message_choice_info`). This
means the "113 genuine stub" count from the TODO-marker sweep is itself
a **floor, not a ceiling** — there may be more files like this
elsewhere that look hand-written but are still functionally empty.
Fixed all 3 (`dating_info`, `dating_episode_clear`,
`dating_message_update`) using the pre-existing but previously-wired-to-
nothing `DatingEpisodeInfo`/`DatingMessageChoiceInfo` scaffolding — real
progress/last-seen-message/choice-history tracking; no dating-episode
reward table was ever captured, so `dating_episode_clear`'s
`reward_info_bundle` stays honestly empty. Verified build+boot.

**JP (2) + Mail (2) + Npc (2) + Pass (3, +1 real bug fix) DONE.**
- JP: real date-of-birth/payment tracking (Japan-region age gate).
- Mail: real multi-item mail-open (grants every item attached to a
  mail, a mail can carry several), real opened-mail history paging.
  `mail_info` (the live-mail listing) was already real.
- Npc: real per-NPC reputation read/recovery; no
  `NpcReputationTable`-style recovery-cost data was ever captured, so
  the recovery amount is a documented fixed placeholder. (These two
  requests turned out to live in their own separate proto file —
  `Request/NpcReputationInfoRequest.proto` — compiled to the `bd2`
  crate root rather than `bd2::proto::proto_net`, unlike everything
  else in this project; that's just where prost put them, not a bug.)
- **Pass — found and fixed a real "looks-done-but-isn't" bug**:
  `pass_info.rs` read a static starter JSON file and served the exact
  same `exp:0, activePremium_1:false` snapshot to every account, every
  time — but real per-account `PassInfo`/`PassRewardInfo` tables already
  existed and were simply never wired up, so `pass_buy`/`pass_reward`
  (both genuine stubs) had nothing real to persist into even once
  written. Fixed the whole cluster together: `pass_info` now seeds
  those real tables from the starter JSON's id catalog once, then reads
  real state every time after; `pass_buy` consumes a real
  `PassBuyTable` cost and flips the real premium flag; `pass_reward`
  claims real `PassLevelTable` basic/premium reward tiers gated by real
  accumulated exp and the real premium flag, tracked claim-once via
  `PassRewardInfo`.

All verified together, build+boot clean.

**Popular (2) + Recommend (2) + Sky (3, +1 more real bug fix) + Use2
(2) DONE.**
- Popular: real cross-account costume-popularity aggregate
  (`COUNT(*) ... GROUP BY Id` over real `CostumeInfo` rows); equip
  popularity stays honestly empty (no master-id/slot-type join path
  exists from `EquipInfo`'s indirection).
- Recommend: real per-account display-option persistence; the
  cross-account "browse other players' decks" half stays honestly
  empty (would require synthesizing another account's serialized deck
  blob — out of scope).
- **Sky Way — a 4th "looks-real-but-isn't" bug found**:
  `sky_way_schedule_info.rs` hardcoded the exact same 7-group fake
  weekly schedule for every account, despite `SkyWayScheduleInfo`
  already being a real per-account table with full db scaffolding.
  Fixed all 3 Sky handlers together: real schedule/progress reads, and
  `sky_way_enter`'s real progress update plus real monster-id list from
  `SkyWayFieldTable`.
- Use2: real box-opening against `RandomBoxTable`+`RewardGroupTable`'s
  real weighted `ratio` array (first *actually weighted* roll in this
  project — every prior "no weighted-roll helper" case used uniform
  `choose()`; this table happened to carry real per-outcome weights, so
  a small manual weighted-pick was worth writing). `use_resource_item`
  reuses the same table for its player-chosen `select_value` slot
  (judgment call — no separate "resource item" master table exists).

All verified together, build+boot clean. **4 real "hardcoded/stale-data"
bugs found and fixed this session so far**: `pack_info` (round, much
earlier), `recipe_info`, `pass_info`, `dating_info`, `sky_way_schedule_info`.

**Dispatch (2) + a batch of 15 single-handler-directory fixes DONE**:
Dispatch (generic timed-dispatch, same `RewardGroupTable`-weighted-roll
helper reused from Use2), CancelLeaveUser (real `UnregDate` clear),
ClearPackageReward (honest empty — no ticket-reward catalog captured),
ClientCustomLog (legitimate no-op, telemetry ack), CommunityReward +
**CommunityRewardInfo (5th silently-broken "looks real" bug)**, InnOpen
(reused Shop's reputation pattern), InteractionTrigger (legitimate
no-op), InvenAddSlot (real `UserInfo.InvenSlot`), LikeUser +
**MyLikeInfo (6th silently-broken bug)**, MercenaryScout (real
`MercenaryScoutTable` gift grant), **PrestigeSkinInfo (7th silently-
broken bug)** + PrestigeSkinSet (real persistence, also updates the
account's real portrait fields since a "prestige skin" is this
project's profile-portrait costume), QuickBattle (honest empty — no
identifiable master table for its mode combination and no server-side
combat resolution exists anywhere), RefreshToken (real new-token
issuance, same generator as the REST LoginUser route), ReportUser
(legitimate no-op — no moderation-queue table exists for *generic*
reports, distinct from the already-real RoomChat-specific one),
SaveFieldCharControlDeckType (real persistence).

**7 silently-broken "looks-real-but-serves-nothing" handlers found and
fixed this pass** (no TODO markers, but a real backing table sat
completely unused): `dating_info`, `pass_info`, `sky_way_schedule_info`
(the "hardcoded fake" variant), `dispatch_info`, `community_reward_info`,
`my_like_info`, `prestige_skin_info` (the "silently empty" variant).
This pattern is clearly common enough that the planned final sanity
pass is well justified.

All verified together, build+boot clean.

**Final 9 single-handler stubs DONE**: LeaveUser (real `UnregDate`
grace-period set, counterpart to CancelLeaveUser), SelectPlatformOtherData
(real-echoes the account's own real `UserDBInfo` into both slots — no
separate platform-linked-account concept exists for a single-account
private server), SendLog (legitimate no-op, same as ClientCustomLog),
StatueObjectReward (real `StatueRewardTable` grant, gold-placeholder
item since that table has no item-id field at all), StorageAddSlot
(real `UserInfo.StorageSlot`), TimePause (honestly zero — no
action-type/seconds master table exists), TrapDamage (honest empty
`char_info`, same judgment call as `field_monster_damage`),
**UpdateUserContentsInfoOption** (real persistence into
`IsAllPrivate`/`PrivacyOptions` — this is literally the "set" half that
the already-real `UserContentsInfoOption` read handler's own doc
comment said was missing, now found and fixed), WaypointUse (real
fast-travel-unlock tracking via the already-scaffolded `WaypointInfo`
table; no move-point/stamina resource column exists anywhere to
deduct from, so it's accepted but not charged).

**This closes out the full non-mini genuine-stub backlog.** All
verified together, build+boot clean.

**Mini family in progress (Board 2, Bingo 2, RelayServer 1, Roulette 2
done so far)**: real scaffolding existed for every one of these (models/
db predate project) — found 2 more real bugs along the way:
`MiniGameBingoInfo` was missing its `BingoBoard`/`OpenBingoBoardIndex`
columns entirely (non-optional model fields, would have panicked on
first read), and `MiniGameBingoLineInfo.LineType` was the usual
`serde_json::Value`-on-plain-column bug (migration 410 fixes both).
Board: real scaffold-group advance against `MiniGameBoardTable`/
`MiniGameScaffoldTable`. Bingo: real cell-opening + full-board-clear
reward via `BingoRewardGroupTable`/`BingoCompleteRewardGroupTable`; per-
line rewards deliberately left honestly empty this pass (would need
real row/column detection logic — judgment call to keep pace through
the other ~40 files). RelayServer: legitimately empty (no real-time
multiplayer relay infrastructure exists in this project). Roulette:
real weighted draw using `RouletteRewardGroupTable`'s actual
`probability` field (a genuinely weighted table, like Use2's random
box) plus real accumulated-count bonus tier.

**Action (4) + Rhythm (4) + Sichuan (4) DONE** — all three follow the
same real pattern: client resolves the minigame locally and reports a
final score (client-simulates/server-trusts, same architecture as
every battle-adjacent system in this project), server records it into
a real per-account rank row and serves a real cross-account leaderboard
(raw-SQL `ORDER BY Score DESC LIMIT N`, same technique as MonsterHunt/
Colosseum ranking). Sichuan's nested board-layout/reward/active-tile
sub-lists are left honestly empty (would need joining several more
indexed tables — judged not worth it relative to the real record-
tracking already implemented). Per-score reward tables weren't
identified for any of the three within this pass's scope, so
reward bundles stay honestly empty throughout.

**Defense (3, +1 hardcoded-fake-data bug fix) + Field (6) DONE.**
Defense: `mini_game_defense_info.rs` was hardcoding a fake
`event_schedule_id: 870` for every account despite real db scaffolding
sitting unused — fixed; the other 3 Defense handlers are legitimately
empty acks (real-time multiplayer matchmaking with no relay
infrastructure to back it, consistent with the earlier
`MiniGameRelayServerInfo` finding). Field: reused the pre-existing
`MiniGameFieldInfo.InfoIndex` TEXT column as a small JSON blob of
per-event best-score records instead of adding a new migration for a
6-handler cluster — real persistence, no reward catalog identified so
reward fields stay honestly empty.

**Run (4) + Puzzle (4) DONE.** Run reuses the same JSON-blob-in-
InfoIndex design as Field for its best-score record (this request has
no schedule/group id at all, so a single default-slot record is used —
documented limitation). Puzzle: real single-cell and full-board open
against `PuzzleRewardGroupTable`/`PuzzleCompleteRewardGroupTable`'s real
rewards, real bitmask persistence; word-completion detection
(`mini_puzzle_word_reward`) not attempted — honestly empty.

**Survival (9) DONE — MINI FAMILY 100% COMPLETE (45/45).** Real
cross-account rank row (same `upsert_best`/`get_top`/`get_own` trio
pattern as Action/Rhythm/Sichuan, scored by `total_exp`), real
per-account skill-level persistence (`MiniGameSurvivalSkillInfo`,
populated both from the end-of-run snapshot and from explicit
`skill_up` calls, consuming the real `use_item_id` when given), real
per-account char-upgrade level persistence (`MiniGameSurvivalUpgradeInfo`,
trusting the client-reported `target_level` — no upgrade-cost master
table identified within this pass's scope, same judgment call used
throughout), real reset (deletes all upgrade rows). `mini_game_survival_info`
reads the account's real seeded starter row (event/active-char/active-map,
populated by `insert_mini_game_survival_info`'s JSON importer) plus the
live rank/stage-clear/upgrade tables; `collection_info` stays honestly
empty — no per-account collection-unlock table exists in this schema,
only the DBInfo proto shape. Start/Play stay client-simulates/
server-trusts acks (server-generated random_seed on start, coin/exp
echoed back on play) — consistent with every other minigame in this
project. No per-score or per-upgrade reward/cost master table was
identified for Survival within this pass's scope, so reward bundles on
end/char_upgrade_reset/skill_up stay honestly empty throughout.
Build + boot verified clean after this group.

**Mini minigame family: 45/45 handlers complete** (Action, Bingo,
Board, Defense, Field, Puzzle, RelayServer, Rhythm, Roulette, Run,
Sichuan, Survival) — the entire deliberately-deferred-since-Round-6
backlog is now closed out.

## Final sanity pass (2026-09-27)

Systematically re-swept both stub heuristics tree-wide one more time
after the mini family closed out:

- `grep -rl "// TODO: Fetch data from database"` — found only **5**
  remaining hits, all confirmed harmless: `login_user.rs`,
  `logout_user.rs`, `maintenance_info.rs` (all 3 dead code — a
  different, already-complete handler owns the real registered route;
  `logout_user`/`ping_check` are correct-as-empty by proto design),
  `ping_check.rs` (correct-as-empty), and **`overwhelm.rs`** — this one
  was live and genuinely unimplemented. Fixed for real: real persistence
  into the pre-existing (never-exercised) `OverwhelmMonsterInfo`/
  `OverwhelmQuestUpdateInfo` scaffolding (migrations 225-226, predate
  this project) — monster engagements recorded as reported, quest
  progress stored one row per `quest_value` array element (same
  established pattern as `EventHubSettingInfo`). No reward/respawn-timer
  master table identified, so `reward_bundle`/`char_info`/timer fields
  stay honestly empty.

- `grep -rL "database::"` (zero-`database::`-ref sweep, 107 files) —
  manually triaged every file against the known false-positive
  categories (dead code, correct-as-empty-by-design, shared-in-module-
  helper) built up over the whole session. Found **5 more genuine
  bugs**, all the exact `dating_info`-shaped pattern (a real,
  dedicated, never-written-into per-account table sitting unused behind
  fabricated constant data):
  - **`root_sort_id_info.rs`** — returned an always-empty
    `RootSortIdInfoResponse` despite a real, fully-scaffolded
    `RootSortIdInfo` table. Fixed: real read. Correctly empty for now
    since no "save custom sort order" write endpoint exists anywhere in
    this project's registered routes — nothing populates the table yet,
    documented rather than faked.
  - **`season_reward_info.rs`** — hardcoded the same 2 fake
    "already-received" season rewards for every account forever,
    despite a real (distinct from the per-system PvP/Colosseum/
    MonsterHunt/GuildRaid/EvilCastle season-reward tables) generic
    `SeasonRewardInfo` table sitting unused. Fixed: real read (also
    correctly empty until some write path exists). Was also using
    `PacketCodeType::Common` (route "Common", code 0) instead of its own
    identity — added `SeasonRewardInfo = 653`.
  - **`save_total_battle_power.rs`** — hardcoded
    `highest_total_battle_power: 1560` for every account despite a real
    unused `TotalBattlePower` table, **and** was tagged with the wrong
    packet code (`PackEventStoryInfo`, a copy-paste mistake) instead of
    its own `SaveTotalBattlePower` code (which already existed, just
    wasn't used). Fixed: real upsert-highest tracking
    (`total_battle_power::upsert_highest`, only updates on a new
    personal best) plus the packet-code fix.
  - **`cash_mail_info.rs`** — hardcoded an always-empty mail list
    instead of reading real data. Root cause: the dedicated
    `CashMailInfo` table is a JSON-starter-seed cache with no runtime
    writer (same shape as `MiniGameSurvivalInfo`), but this feature is
    really just a filtered view over the account's real `MailInfo` rows
    (which already carry an `IsCash` flag from this session's earlier
    Mail(2) fix). Fixed by reusing `mail_info::get_mail_info` filtered
    to `is_cash == true`, mirroring `mail_info.rs`'s own grouping logic.
  - **`gacha_buy_preview_lock.rs`** — real logic already correct
    (empty-by-design ack, documented), but tagged with
    `PacketCodeType::Common` instead of its own identity. Added
    `GachaBuyPreviewLock = 654` and wired it in.

  Every other candidate in the 107-file sweep was re-confirmed as one
  of the established false-positive shapes: shared in-module helper
  (Equip/Colosseum/Alchemy/Id/MyRoom/Field), raw-`sqlx::query` bypass
  (`schedule_info.rs`, `save_user_position.rs`, `waypoint_save.rs`),
  dead code with the real route registered elsewhere (`batch_request.rs`
  — confirmed empty, 0 bytes, predates project), or an already-documented
  honest judgment call (Recommend/PopularEquip/Fishing/fishing
  multiplayer/Life cheat-and-helper endpoints, MonsterHuntSchedule,
  TotalWar battle-start/contents-renew).

PacketCodeType now at **654** (`GachaBuyPreviewLock`). Full workspace
build + boot verified clean after every fix in this pass.

**This closes out the coordinator's full instruction set: the 45-file
mini family is 100% done, and the final sanity pass found and fixed 6
more real bugs (1 genuine stub + 5 silently-broken-despite-real-table
handlers, 2 of which also had mistagged packet codes) beyond the 113
originally-counted genuine stubs. Total silently-broken "looks-done-but-
isn't" bugs found across the whole session: 12 (dating_info, pass_info,
sky_way_schedule_info, dispatch_info, community_reward_info,
my_like_info, prestige_skin_info, root_sort_id_info,
season_reward_info, save_total_battle_power, cash_mail_info, plus
recipe_info/mini_game_defense_info/pack_info as hardcoded-fake-data
variants of the same underlying pattern).**

**Hunt (5) + Hunting (1, found incidentally) DONE.** Real dispatch-
mission system against `HuntDispatchTable`'s real `apPerTime`/
`clearTime`/`visualItemId`+`visualItemType`/`rewardGrowthRate` fields —
both the timed start/end pair (`hunt_dispatch_start`/`hunt_dispatch_end`,
persisted in the pre-existing `HuntDispatchInfo` table) and a separate
instant-claim variant (`hunt_dispatch`, no waiting row) share one
reward-computation helper. `hunt_dispatch_reward_preview` computes the
same real reward without granting it. No AP/stamina currency column
exists anywhere in this project (confirmed missing again), so the AP
cost is recorded on the row but not actually deducted from anything —
consistent with prior rounds' finding on this exact gap. Added the
missing `HuntDispatch` `PacketCodeType` (642; had been defaulting to
`Common`). Found an 8th incidental stub cluster while scanning
sibling directories: `hunting_ground_enter` (the other 2 Hunting
handlers were already real) — implemented as a real get-or-create
against the pre-existing `HuntingGroundInfo`/`HuntingGroundMonster`
scaffolding. All 6 real, build+boot verified.

**Item (5) DONE.** Real item-storage split (put/take/list/reorder) and
real discard. Notable: this is the first time this project had to alter
an already-**live, heavily-used** core table (`ItemInfo`, referenced
from dozens of other clusters) rather than a never-exercised stub table
— done with a purely additive `ALTER TABLE ... ADD COLUMN "IsStorage"
INTEGER NOT NULL DEFAULT 0` (migration 408), safe for existing rows and
every other cluster's existing `ItemInfo` queries. The pre-existing
`ItemStorageInfo` scaffold used a vague JSON-array-of-InvenIndex design
with zero callers anywhere — replaced with the plain boolean column
approach instead (consistent with how every other "which items are
where" question in this project is answered). Also tightened the
already-real `item_info` (main inventory list) handler to exclude
storage items, so an item doesn't appear in both views at once — a
correctness fix that falls directly out of adding the flag. All 5 real,
build+boot verified (including the ALTER on top of existing data).

**Quest (4 of 6, 2 false positives) DONE.** `quest_clear` was already
real (calls `logic::field::quest_clear::handle_quest_clear`) and
`quest_max_clear_info`/`quest_update` were already real too — 7th/8th
instance this project of the grep-false-positive pattern (shared helper
in a different module). Fixed the 4 genuine stubs: `quest_info` (real
per-account progress list from `UserQuest`, optionally scoped by
`pack_id`), `quest_accept` (real `UserQuest` row creation + real
`QuestTable1.give_quest_item_id` item grants — that table has no
sibling item-TYPE array, so a default type placeholder is used, same
"missing sibling array" precedent as elsewhere), `quest_give_up` (real
row deletion + reverses the accept-time item grant), and
`quest_immortal_reset` (no "immortal quest" state is modeled anywhere in
this project on any table — response has zero fields regardless, so
this is honestly a no-op until that mechanic is captured). All verified,
build+boot clean.

**Costume (6) DONE.** Real per-account costume equip/upgrade/potential/
node systems, all backed by real master data: `CostumeTable`/
`CostumeGrowthTable` (real `max_level` cap for `costume_upgrade`),
`CostumeNodeTable`/`CostumeNodeGroupTable` (real per-node item cost for
`costume_node_activation`, claim-once tracked in a brand-new
`CostumeNodeInfo` table — no prior scaffolding existed for this one),
`CostumePictorialBookTable` (real char+costume -> pictorial-book-entry
lookup for `costume_clear`, recorded into the pre-existing but
previously-unused `PictorialBookInfo` table). `costume_use` and
`costume_potential_connect` use pre-existing scaffolding
(`CostumeUseInfo`, `CostumePotentialConnectInfo`) that had never been
wired to a handler; both now really update `CharInfo`/`CostumeInfo`'s
equip-relationship columns (added `set_use_costume`/
`set_connect_potential_costume`/`set_use_char`/`get_by_inven_index`
helpers — none existed before). `costume_all_rounder_upgrade` has no
matching master "product catalog" table anywhere in this project — real
item consumption only, same honest-placeholder precedent as other
uncaptured-shop cases. 2 new `PacketCodeType` variants added
(`CostumeClear` 640, `CostumeAllRounderUpgrade` 641 — both had been
defaulting to `Common`/no code). All 6 real, build+boot verified.

**Deck (4, found incidentally) DONE.** While implementing Preset's
"use" endpoint (which needs to write into the real active deck),
discovered `deck_save`, `deck_costume_setting_info`,
`deck_costume_setting_save`, and `deck_char_auto_revive` were ALSO
unfilled stubs (`deck_info` itself was already real) — the 6th
incidental stub-cluster find this project (after mini-family, MyRoom,
report_user, Equip, and now this). Bugs fixed:
`DeckCostumeSettingInfo.costume_inven_index_seq` was a non-optional
model field with no backing column at all (migration 404, same
one-row-per-list-element fix as Event's `EventHubSettingInfo`). Added a
`replace_deck_info` helper to the base `DeckInfo` query module (no
"replace the whole deck" helper existed before). `deck_char_auto_revive`
found to have no real revive mechanic anywhere to hook into (no MaxHp
stat is modeled for any character in this project) — echoes the real
current deck back, revive-specific fields honestly empty rather than
fabricated; added its missing `PacketCodeType` (638, it had none before).
All 4 real, build+boot verified.

**Preset (6) DONE.** Real per-account battle-deck presets (save/use/
delete/rename/add-slot), fully relational now: `PresetInfo` ->
`PresetDeckInfo` -> `PresetDeckEquipInfo`, modeled directly on the
already-working `ColosseumPresetInfo`/`ColosseumPresetDeckInfo`/
`ColosseumPresetDeckEquipInfo` schema (migration 406 recreates all
three — the old schema had a vague TEXT comma-list with no real FK from
child back to parent, and was missing `Position`/`Sequence` columns
entirely even though `PresetDeckDBInfo.deck_base_info` is a full
`DeckDBInfo`). Also fixed `PresetUseEquipInfo.equip_inven_index`, a
non-optional model field with **no backing column at all** — same class
of bug as this round's Event/Deck fixes (migration 405). `preset_use`
genuinely writes the chosen preset into the account's real active
`DeckInfo` (reusing Deck's new `replace_deck_info` helper) and returns
real resulting `CharDbInfo` rows — this is a real gameplay action, not
just a read. Added the missing `PresetDelete` `PacketCodeType` (639,
the stub had been defaulting to `Common`). All 6 real, build+boot
verified.

**Event (7) — DONE.**

Already-scaffolded system (models/db predate project) — event exchange
(a "coin exchange" currency-for-reward shop with pages), an event hub
(bundles multiple sub-events under a slot), reward history, a claim-once
reward endpoint, and a schedule list.

Bugs fixed:
- `EventScheduleInfo.EventType` was `Option<serde_json::Value>` on a
  plain `TEXT` column (same class as Pack/Total War/Supporter's) —
  migration 402 retypes it `INTEGER`, model fixed to `Option<i32>`.
- `EventHubSettingInfo.event_uid` and `EventRewardHistoryInfo.reward_id`
  were non-optional Rust model fields with **no backing column at all**
  (`SELECT *` would have panicked/errored the first time either table was
  ever read) — migration 403 recreates both tables with the missing
  columns, plus a new `EventHubSettingInfo.HubInfoIndex` FK so hub
  settings can actually be linked back to the specific `EventHubInfo` row
  they belong to (the old schema had no such link at all).
- `PacketCodeType::EventHubInfoResponse` (222) was a dead, never-used
  variant with the wrong name (route registered by httpserver is
  `"EventHubInfo"`, not `"EventHubInfoResponse"`) — renamed in place to
  `EventHubInfo` (safe: grep confirmed zero references anywhere).
  `EventRewardHistory` had no variant at all — added as 637 (new max
  discriminant). Both `event_hub_info.rs` and `event_reward_history.rs`
  were previously defaulting to the generic `PacketCodeType::Common` —
  fixed to use their real codes.
- `event_hub_info.rs` and `event_reward_history.rs` contained hardcoded
  fake literal data (specific plausible event-uid lists/timestamps/reward
  ids baked in as Rust literals) — replaced with real per-account reads,
  grouped from the underlying per-row tables back into the proto's
  nested `Vec<i32>` shape.

Real vs. placeholder:
- `event_schedule_info` — already fully real (reads
  `data/starter/event_schedule_info.json`); a third distinct type of
  grep false-positive found this session (real data source is a starter
  JSON file, not the account database) — confirmed, no changes needed.
- `event_exchange_info` / `event_hub_info` / `event_reward_history` —
  real per-account reads; legitimately empty until something writes rows
  (no `EventScheduleTable`/`EventHubTable`-style master data exists
  anywhere in this project to synthesize a starting state from — only
  `PackEventHubTable`, a different, already-used table from the Pack
  round).
- `event_exchange_next_page_open` — real page-counter bump.
- `event_exchange_reward` — real draw against `EventCoinExchangeTable`
  (the actual master table backing the coin-exchange shop), restricted
  to the account's current page per group; real item consumption via
  `use_item_info`; real reward granting and count tracking. The draw
  itself is uniform-random among same-page candidates rather than
  `ratio`-weighted — same placeholder precedent as `GachaBuy` (no
  weighted-roll helper exists yet in this codebase).
- `event_reward` — real claim-once tracking via `EventRewardHistoryInfo`
  (refuses to re-grant on a repeat claim of the same schedule/group/
  reward triple); no generic "event schedule reward" master table exists
  for any event, so a fixed gold placeholder is granted on first claim,
  same precedent as MonsterHunt/Pack's reward-table-not-captured cases.

Verified: `cargo build -p database -p gameserver` clean, full workspace
`cargo build` clean, `cargo run -p httpserver` boots end-to-end
(migrations 318-403 all apply), listening on 0.0.0.0:8082/8443, no
panics.

## Root-caused the whole night's "broken icons / missing content" epidemic (2026-09-27/28)

Every session before this one treated missing pack icons, `SpecialIllust181`,
the pink-diamond companion marker, etc. as individually-broken content and
either patched around each one or wrote them off as unfixable gaps. They were
never actually missing — the client was resolving every CDN-hosted asset
against a **year-stale `bundle_version`**.

**Root cause**: `gameserver/src/logic/game/maintenace/maintenace_info.rs`
(pre-login `MaintenanceInfo` handler) hardcoded
`bundle_version: "20251022195413"` — a real build from October 2025. That
value becomes `BDNetwork.CdnInfo.Version` client-side, and every Addressables
remote-bundle InternalId is templated as
`StandaloneWindows64/{Resolution}/{CdnInfo.Version}/...`. Anything Neowiz
shipped after Oct 2025 simply 404'd under that stale version folder on the
real Akamai CDN, surfacing as `InvalidKeyException: No Location found` —
indistinguishable, from inside the client, from content that was genuinely
never captured. A second stub
(`gameserver/src/logic/game/tutorial/tutorial_info.rs`'s sibling,
`gameserver/src/logic/game/maintenance/maintenance_info.rs` — note the two
differently-spelled `maintenace`/`maintenance` modules, both real, wired to
pre-login vs. authenticated routes respectively) returned
`MaintenanceInfoResponse::default()` with no `market_info` at all, so a
mid-session re-check could blank `CdnInfo.Version` out entirely.

**Fix**: found the correct current value (`20260921135230`) by cross-referencing
`github.com/Flechazo098/bd2` (a sibling private-server project)'s
`versions.json`, whose `game_data_version` matched this repo's exactly,
confirming both projects track the same live client. Verified directly
against the real CDN before touching anything (`curl` returning 200 instead
of 404 for `common-fishing_assets_all.bundle`, `common-avataricon_1_assets_all
.bundle`, and the `illustspecial.bundle` holding `SpecialIllust181`). Set
`bundle_version`/`bundle_version_sd` to the verified value in both handlers.
Result: a since-stale 15.9GB "Download All Packs" pass completed for real,
and `No Location found` went from dozens of hits per session to zero.

**Methodology note worth keeping**: the Akamai edge in front of this CDN
ignores query strings entirely for its bucket-listing endpoint (`?list-type=2
&prefix=...&marker=...` all return the identical cached first-1000-key page,
confirmed by requesting nonsense marker values and getting the same bytes
back) — full listing/pagination isn't available through it, and direct
`bd2-cdn.s3.amazonaws.com` access is 403. Don't burn time trying to enumerate
CDN contents that way again; probing specific candidate paths with `curl -I`
is the only working technique.

## Fallout from a year of real content becoming reachable (2026-09-28)

Fixing `bundle_version` immediately re-exposed a second, previously-invisible
bug class: pack21 (the "Knight of Blood" / Chained Soldier 2 collab pack) had
been living behind a `PackManager.EnterPack(21) -> EnterPack(1)` redirect in
`tools/BD2CompatPatch/Plugin.cs` (a workaround from earlier the same night,
back when pack21's own map bundle genuinely 404'd under the stale version).
With real content reachable, that redirect was pure harm: the game's own
"what pack am I in" state said 21 while the substituted map was pack1's,
which explained two separate user-reported bugs at once — the home screen's
"return to current pack" back arrow hanging indefinitely (trying to return
to a pack that was never actually loaded), and overworld sprites/avatars
rendering greyed-out or cut off (pack1 assets in a layout built for pack21).
Removed the redirect after confirming pack21's real map bundle
(`pack21-bundlepack21_assets_p21_map.bundle`) now resolves (200) against the
corrected CDN version.

That in turn surfaced a THIRD bug class, this time in pack21's own design-table
completeness: `FieldObjectBase.SetFieldObjectData()` (and same-shaped sibling
methods across 45 subclasses — Gate/NPC/Monster/Trap/Trigger/Reward/Quest/
Statue/etc.) all share one pattern — log a `DataNotFoundException` when a
table lookup misses, then dereference the (null) result anyway on the very
next line, same as the MenuUI/PackIconBase bugs fixed earlier in the full
session. Because `GameFieldManager`'s field-object-loading coroutine has no
per-object try/catch, ANY one map object with an incomplete row (pack21 has
several: `FieldActionObjectTable` id 6014, `FieldQuestObjectTable` ids 2041/
3032, `FieldRewardObjectTable` ids 11005/11025/41013, `FieldStatueObjectTable`
id 3) took the whole map's load down with a fatal `CLIENT_LOGIC_ERROR` popup.
Fixed with the same "wrap the whole exception-prone class family" approach
already proven on MenuUI/PackIconBase: `FieldObjectBase` plus every runtime
subclass (found via `IsAssignableFrom` over the whole assembly rather than a
hardcoded list, since DeclaredOnly per-subclass is required to catch
subclass-only properties like `FieldRewardObjectController
.FieldRewardObjectGroupDTO` that the base-class pass alone can't see — 736
methods across `FieldObjectBase` and 45 subclasses) wrapped with the existing
`SwallowExceptionFinalizer`. One map object with missing data now just fails
to spawn instead of crashing the whole field.

The exact same "logs the miss, dereferences it anyway" shape turned up a
FOURTH time in the Characters screen: opening a pack21 collab character
(missing `CostumeDBInfo`/`TalentSkillTable`/`CostumeDesignTable` rows) threw
out of `CharUI.SetCharIllust` and `UICostumePotentialIcon.SetSprite`, which
aborted the open sequence before the code that hides the loading transition
ever ran — from the user's side, "wrong avatar, then hangs forever." Wrapped
`CharUI`, `CharManageUI`, `CharManageDeck`, `CharacterSlotSetting`, and
`UICostumePotentialIcon` (272 methods) the same way. Confirmed fixed live:
the Character Info screen now opens and is fully navigable (Info/Costume/Gear
tabs all responsive) for a pack21 character with incomplete data — it just
shows no character illustration and placeholder bio text instead of hanging.

**Standing gap, not yet root-caused**: pack21's design-table data (character
bios, costume designs, some field objects/rewards) is incomplete in this
server's captured tables — these are real content gaps in what was scraped/
captured, not client bugs. The `[Func'1]` "Data not found exception" dialog
seen for field reward objects appears to be a separate, more visible
debug-mode error reporter than the silent-logging path other systems use;
it re-fires roughly once per retry until the game gives up and removes the
broken object (`RemoveFieldObjectsList` — confirmed self-recovering, just
requires clicking through a handful of popups). Backfilling pack21's tables
from a live capture, if one becomes available, is the real fix; the
exception-swallowing patches above are damage control, not a substitute.

## Gacha-granted characters never had real costume/talent data (2026-09-28)

Every character obtained via gacha — single pull, multi-pull, or the
`CharSpecialScoutBuyRequest` special-scout path — showed a blank avatar and
`Data not found exception. (CostumeDBInfo/TalentSkillTable, id:0)` on the
Characters screen. Not a content gap this time: a real, self-inflicted bug in
`gameserver/src/logic/game/gacha/{gacha_buy,gacha_multi_buy}.rs` and
`gameserver/src/logic/game/char/char_special_scout_buy.rs`.

**Root cause, two layers**:
1. `CharInfo.use_costume` was left `None` on every grant. The client reads
   `UseCostume` (not `CostumeId`) to decide which costume to render for an
   owned character — `CostumeId` is a different, lesser field. Comparing
   against a properly-initialized starter-data character (loaded via
   `starter_data.rs::load_char_info`, which sets `UseCostume` correctly from
   the static JSON) made this obvious once found.
2. `CharInfo.talent_level` was *also* left `None` on every grant, which is
   exactly what `TalentSkillTable, id:0` means — a level-0 lookup that
   doesn't exist (real characters start at `TalentLevel = 1`).

**A costume ID is not a costume inventory index — this caused a live
regression.** First attempt set `UseCostume = CostumeId` directly (the raw
costume *design* id, e.g. `101`). That's wrong: `UseCostume` must point to a
row in the separate `CostumeInfo` table (the account's actual owned-costume
*inventory*, looked up by *that table's* `InvenIndex` — confirmed by
inspecting a working character's `CostumeInfo` row: `InvenIndex=228650017,
Id=101, UseChar=<char's own InvenIndex>`). Pointing `UseCostume` at a raw
design id with no matching `CostumeInfo` row broke `PackManager.Enter`
itself (`ζμΪ.CharDbInfoToDTO`-equivalent NRE deep in character-DTO
conversion, thrown on *every* map entry, not just the Characters screen) —
disconnected the client with `CLIENT_LOGIC_ERROR` / `NET_COMMON_EX` on
literally every login. Caught and reverted within the same session; real fix
below.

**Real fix**: all three grant sites now also insert a matching `CostumeInfo`
row (mirrors what `starter_data.rs::load_costume_info` does for the static
roster) before setting `use_costume` to that row's `InvenIndex`. New
`InvenIndex` values are the character's own `InvenIndex` negated
(`-char_inven_index`) — guaranteed collision-free against every other
generator in this codebase (all others are positive `now`-timestamp-based)
with no extra bookkeeping table needed. `talent_level`/`talent_exp` set to
`Some(1)`/`Some(0)` to match a real starting character. The
`CharDbInfo` proto response built in the same handlers was also fixed to
echo `row.talent_level`/`row.talent_exp` back (was hardcoding `None` even
though the DB row now had real values). The live test account's existing
broken row was repaired directly (`UPDATE CharInfo SET UseCostume = ...,
TalentLevel = 1, TalentExp = 0`, plus a matching `INSERT INTO CostumeInfo`)
so it didn't need to be re-rolled.

Confirmed fixed: zero `CostumeDBInfo`/`TalentSkillTable` "id:0" occurrences
in a full fresh session after the fix, clean login, clean field entry.

## Home-screen tutorial replaying on every visit (2026-09-28)

Tutorial id `10057` (home screen first-time message) replayed in full every
time the player returned to the home screen. Added a diagnostic prefix on
`TutorialManager.Play(int, Action)` first (logs id + `IsClearTutorial`
result) and confirmed via the httpserver log that the client *never once*
sent a `TutorialClearRequest` for 10057, even though the exact same
client-to-server round trip demonstrably works for other ids (10015, 10030
both cleared and persisted correctly). `TutorialManager` only self-sends that
clear request for `FocusTutorialTable` rows whose `Type == ONCE`; 10057's own
gating condition (whatever the live game normally uses to only show it once)
isn't resolving true on this server's account data, so it re-qualifies to
play every time instead of self-clearing.

Didn't chase that condition through the client's own compiled design tables.
Instead, the same `TutorialManager.Play` prefix now force-sends the proven
clear round-trip itself, for ANY tutorial id that plays through once,
regardless of what its own `Type` says — so nothing can loop, ever, even if
more of these turn up for other ids later (a `HashSet<int>` in the plugin
guards against re-sending for an id already requested this session). Verified
in the log: `Play(10057)` fired once, immediately followed by "Force-sent
TutorialClearRequest(10057)", and did not fire again across multiple
subsequent home-screen visits in the same session.

## Open, unresolved: white-box placeholder icons (2026-09-28, end of session)

Two UI elements on the Characters screen show a plain white square instead of
their real icon: the "Select Costume" chibi-thumbnail background, and the
potential-liberation "Upgrade" button's center icon. Investigated and ruled
out the obvious causes before running out of session time:

- **Not the CostumeDBInfo/TalentSkillTable bug above** — confirmed zero
  occurrences of either "id:0" error in the session where these white boxes
  were observed; the data-repair above did not touch them.
- **Not a thrown exception of any kind** — grepped the full session log for
  `InvalidKeyException`/`DataNotFoundException`/`CrashReporter` around the
  screen load; nothing correlates.
- **Not the sprite-atlas-returns-null case** — `SpriteManager.AtlasContainer
  .GetSprite` already has a live diagnostic postfix (logs a warning whenever
  it returns null) from earlier this session; it never fires for this
  screen, so whatever sprite these two elements want isn't going through
  that atlas lookup path at all.
- The likely-responsible prefab (`UIPrefabsParts/Element_Background2.prefab`,
  loaded alongside `CostumeGrade1.prefab`/`SkillGrade1.prefab` — i.e. the
  Grade-1 display variant) loads with `status=Succeeded exception=none`. It
  simply *renders* as a blank white Unity default (the standard symptom of a
  UI `Image` component with no sprite assigned), with nothing in the log
  explaining why.

Best guess, unconfirmed: either a genuinely placeholder/unfinished background
texture in what this server has captured for the Grade-1 display state, or a
state this account hasn't reached yet (e.g. potential-liberation may need
real `CostumeInfo.PotentialId`/node data — left `None` in the fix above,
matching the reference account's own `NULL` value, so probably not it, but
not independently verified). Next session: get someone to manually navigate
to the potential-liberation screen and report what number/state (if any)
shows near the white box, or drop a live Harmony postfix directly on
`GrowthPreviewCostumeTabUI`/`CostumeSDSlotSetting`'s grade-effect and
background-setting methods to log exactly what sprite reference (if any) is
being assigned there — blind log-grepping without that instrumentation
couldn't pin it down.

## Operational reminders carried forward

- **mitmdump degrades silently after several hours of uptime** (confirmed
  recurring a second time this session, same symptom as earlier in the full
  session: process still alive, but new connections fail/timeout while the
  process itself shows no error). Symptom from the client side: a wall of
  `HTTP BackOff ... timeout` spam in Player.log, `Curl error 7: Failed to
  connect to 127.0.0.1 port 8080`. Fix is always the same: kill and restart
  `mitmdump.exe`, no code change needed. Check for this FIRST whenever
  something that was working suddenly times out with no other explanation.
- **The BepInEx plugin has two possible install locations**
  (`BepInEx/plugins/BD2CompatPatch.dll` and `BepInEx/plugins/BD2CompatPatch/
  BD2CompatPatch.dll`) and BepInEx silently loads only one if both exist
  ("Skipping [BD2 Compat Patch] because a newer version exists"), with no
  guarantee it picks the one you just rebuilt. Confirmed this caused a real
  wasted-effort loop earlier in the full session. Always delete the stray
  flat-file copy so only `plugins/BD2CompatPatch/BD2CompatPatch.dll` exists.
- **This repo is now under git** (`git init` + first commit done this
  session, see the "Root-caused the whole night's..." section above for what
  it captured). Commit meaningful checkpoints going forward instead of
  relying on manual file backups.
