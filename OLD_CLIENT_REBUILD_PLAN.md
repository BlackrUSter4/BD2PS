# Old-client-as-baseline pivot — planning notes (2026-10-02)

Deferred for a later session. This documents the decision context and findings so far, not a committed plan.

## Why this came up

`CLIENT_UPDATE.md` shows BD2PS-main (the Rust reimplementation targeting the
*current* official client) has had repeated, severe, unresolved client-side
failures across multiple sessions: the chief's-house entry gate hangs forever
(permanently documented as unfixed), the cutscene/dialogue freeze is
unreliable at best, synthetic movement input never worked, and the
battle-crash fix chain is still unconfirmed as of the last checkpoint.

Meanwhile, a separate old client+server pair was found and tested the same
night:

- Location: `A:\Private Servers\bd\棕色尘埃2\棕色尘埃2\棕色尘埃2\棕色尘埃2\Browndust2_10000001`
  (originally buried inside a Chinese ad-spam-laden download; cleaned up,
  ad/spam files and an unneeded 220MB bundler installer removed)
- Unity 2022.3.22f1, built Aug 26 2024, client-reported version `1.68.11`
- Its own `server/Bd2.Server.Api.exe` (the real, non-reimplemented, original
  C# backend for that era) was launched standalone on port 5000 and the
  client connected, logged in, created an account, fetched batch data — all
  cleanly, **zero crashes**, zero client patching, zero cert/DNS tricks
  needed.
- Note on reproducing this test: the client's audio engine (FMOD) crashes
  reliably and deterministically on this machine right after the first
  login response (`RPE_NEON_API_ACCESS_TOKEN_EXPIRE`) — same stack trace
  (`fmodstudio.dll`, `FMOD::ChannelControl::stop` etc.) three times in a row
  this session. Launch with `-force-disable-audio` to get past it; this
  looks environment/driver related, not a server-logic bug.

## Key technical facts established tonight

- **Old client is unprotected**: `Assembly-CSharp.dll` is a plain,
  uncompressed 40MB .NET assembly. No `.7z` wrapper, strings search found
  nothing obfuscated.
- **New (current) client is hardened**: ships `Assembly-CSharp.7z`
  (compressed), and BD2PS-main's own `crypto/aestools.rs` + `network.rs`
  confirm the current client speaks AES-encrypted, Protobuf-serialized
  packets with per-route keys — a fundamentally different wire protocol
  from the old server's plain JSON/REST. The current client's decompiled
  method names are also obfuscated (Greek-letter mangled, per
  `CLIENT_UPDATE.md`'s decompile notes) — the old client was not checked
  for this but is very likely not obfuscated given the plain assembly.
- **Content/feature gap is much bigger than "more data"**: old server's
  `server/Data/` has 14 table files total. BD2PS-main's real captured data
  (`data/tables/`, pulled from the actual current game) has **441** distinct
  table types — **427 systems with no old-server equivalent at all**,
  including Character Awakening, Character Imprint, a whole Cafeteria
  feature, three separate minigame types (Action/Rhythm/MGD), Achievements,
  Attendance, Bingo, an overhauled Battle Deck/Power/Buff system, and more.
  These require actual new client-side code/scenes/assets that do not exist
  anywhere in the old compiled binary — not something addable via data
  alone.
- **Where old and new data DO overlap, schemas matched exactly**: old
  server's `CharTable.txt` (1877 rows, 663 unique characters) has the exact
  same field set as the newer `root` copy's `CharTable.txt`. (Caveat: that
  `root` copy's table is itself probably not a fully-current authoritative
  source — BD2PS-main's own captured `CharTable.json` only has 89 rows,
  confirming their capture is incomplete; the old server's own data is
  actually more complete for *this specific table* than BD2PS-main's real
  capture.)
- Root copy's server config was found mid-session with an inconsistent
  Kestrel setup (Http advertised as `:5001` in `GameServer` field reading
  `:5000`, plus an HTTPS `:443` binder with the `mt.bd2.pmang.cloud.pfx`
  cert) — this is the setup that requires the `127.0.0.1 mt.bd2.pmang.cloud`
  hosts-file entry already present on this machine. That server (PID 33236
  as of tonight) and nothing else should still be running on 443/5001.
- `mitmdump` (reverse-proxying `:5000 → :5001` for the root setup) was
  stopped tonight to free port 5000 for the old server test. It was not
  restarted — restart it if the root/current-client setup needs that proxy
  again (`mitmdump --mode reverse:http://127.0.0.1:5001 --listen-port 5000
  --set flow_detail=3`).

## Three candidate starting points discussed, not yet chosen

1. **Feasibility pilot**: take one character's asset bundle from the root
   (current, Unity 2022.3.62f2) client and try loading it via the old
   client's (2022.3.22f1) Addressables catalog (`StreamingAssets/aa/`).
   Same Unity 2022.3 LTS branch, so bundle-format compatibility is
   plausible but unconfirmed. This is the cheapest way to find out whether
   porting real current-game content into the old client is possible at
   all before investing further.
2. **One small new system end-to-end**: pick the smallest missing system
   (e.g. Attendance rewards — likely just login-streak bookkeeping, no new
   3D/scene assets) and try to fully implement it in the old client/server
   as a concrete template for how much work each additional system costs.
3. **Data-only new content** (no new systems): just port new
   characters/equipment into the old server's existing tables, reusing
   mechanics the old client already has. Already confirmed low-risk/
   feasible; doesn't touch the "427 missing systems" problem at all.

No option chosen yet — explicitly deferred by the user ("document we will
do later"). Pick back up here next time this is revisited instead of
re-deriving the above.

## Option chosen and attempted (2026-10-02, later the same day): option 3, data-only character port

The user chose option 3 ("port new characters/equipment into the old
server's existing tables") after a session that started on BD2PS-main (the
*current*-client reimplementation) got redirected here mid-session — see
`CLIENT_UPDATE.md`'s own pinned failure section at the top of that file for
why BD2PS-main was abandoned as the target: repeated, severe, previously
undisclosed client-side failures across multiple past sessions that were
never actually fixed despite other parts of that doc claiming "RESOLVED."
**That mismatch (one part of a doc saying resolved, the pinned section
saying failed) is exactly the trap this session fell into once already
today — read the pinned section first, every time, before trusting any
other claim in that file.**

### Goal picked: port characters 202, 203, 204

Of the 11 characters present in the *current* client's data but missing
from this old server's own `CharTable.txt` (663 of 674 total already
present), only 3 (202/203/204) have real client-side assets already
baked into this old build's catalog (confirmed via asset-path grep
against `server/wwwroot/ServerData/.../catalog_alpha.json` - the other 8
have zero asset references anywhere and would need real new art, not
just data). These 3 were chosen specifically because they're the
cheapest real (non-fabricated) win available.

### Server-side data: done, low risk, straightforward

Added real `CharTable.txt`/`CostumeTable.txt` rows (schema matched
field-for-field against existing working rows, not guessed) to the old
server's own `server/data/`, backed up first. Granted the 3 characters to
the test account (`Uid 10003`, username `blue`, password `123456`) by
replicating the EXACT grant sequence from the decompiled
`GameGachaService.GachaBuyCostume` (not guessed): `CostumeInfo` inserted
first to get its `InvenIndex`, then `CharInfo` referencing it
(`Hp = round(HealthValue)`, `UseCostume = that InvenIndex`,
`ConnectPotentialCostume = costume.SkillGroupId`), then
`CostumeInfo.UseChar` updated back. Confirmed correct via direct SQLite
inspection. This part of the port is solid and done.

### Client-side: the real, much bigger problem - still NOT fully resolved

**The client has its own separate copy of design data** (same
architecture already known for the *current* client - server data and
client data are not the same store). Granting the account these
characters without also fixing the client side crashed it instantly on
login (`NullReferenceException` in `IntroUI`, traced to
`Data not found exception. (CharTable, id:2020)` etc. logged then
dereferenced anyway - the same bug *shape* already seen and fixed
repeatedly on the current client, just a different assembly).

**New tool built**: `tools/BD2OldClientCompatPatch/` - a second
BepInEx/Harmony plugin (separate from `BD2CompatPatch`, which targets the
*current* client) targeting this old client's `Assembly-CSharp.dll`
specifically. Deployed to this old client's own
`BepInEx/plugins/BD2OldClientCompatPatch/`. BepInEx itself was freshly
installed here too (cached package from an earlier session's scratchpad,
`BepInEx_win_x64_5.4.23.5.zip`) since this client never had it before.

**Key technical findings, worth keeping for next time**:
- The old client's design-data layer is a class literally named
  `RawDataManager` (not obfuscated) backed by a *real* local SQLite
  database queried with actual SQL strings (`SELECT * FROM CharTable
  Where id = ...`) - confirmed by decompiling it directly.
- Most of the *calling* code around it IS obfuscated, and - same lesson
  CLIENT_UPDATE.md already documents for the current client - **static
  decompile output (ilspycmd) is not reliable alone**. Two class-name
  guesses from a first-pass decompile turned out wrong when checked
  against a *live* Harmony-captured stack trace (`StackTrace(true)` on a
  patched `DataNotFoundException` constructor). Always prefer live
  evidence over a static guess when the two disagree, same rule as the
  current client.
- A single table can have **multiple independent lookup methods**
  reaching the same underlying data via different call paths (confirmed
  for both `CharTable` - 3 methods on one class - and `CostumeTable` - 2
  *different* classes/paths, one going through generic cache
  infrastructure). Patching one path is not enough; each table needed
  checking for siblings.
- **The real fix, found after a long detour**: everything ultimately
  funnels through one shared, non-obfuscated, generic primitive -
  `RawDataManager.GetValueObject<T>(DatabaseType dbType, string
  tableName, int key) where T : IMessage<T>, new()`. Patching this ONE
  method (per concrete `T` - see below) fixes every table's gap in one
  place instead of chasing each table's own wrapper/cache maze
  individually. This is the method to patch first next time, not a
  last resort.
- **Harmony/MonoMod cannot patch this method's OPEN generic definition**
  directly - throws `System.NotSupportedException: Specified method is
  not supported` from `MMReflectionImporter.ImportGenericParameter` (a
  real MonoMod limitation). The fix: patch a **closed** generic
  instantiation instead (`openGeneric.MakeGenericMethod(typeof(SomeTable))`),
  one per concrete table type - this patches cleanly since the closed
  signature has no unresolved generic parameters left. Always wrap this
  in try/catch regardless - an unguarded failure here silently aborted
  the rest of plugin `Awake()` once already this session, including
  patches that otherwise work fine.
- **A separate, generic client bug**: id `0` is used throughout this
  codebase as a "nothing selected" sentinel, but at least some calling
  code does an unconditional lookup without checking for it first -
  same "log then dereference anyway" shape as the main bug, just
  table-agnostic. Confirmed for `CharTable`, `CostumeTable`, and
  `CharLevelTable` so far, each a *different* table, same `id:0`
  pattern. Fix: for `GetValueObject<T>`, when `key == 0` and the real
  lookup returns null, return `Activator.CreateInstance<T>()` (a
  legitimate empty/zeroed row, not fabricated data) instead of letting
  the exception propagate. This needs no per-table data and should
  apply to any table, not just the 3 actually granted.
- Built a **self-healing mechanism**: a Harmony prefix on
  `DataNotFoundException`'s constructor that, when it fires for a table
  name we care about, resolves that name directly to a `Type` and
  patches `GetValueObject<ThatType>` reactively on the spot (no redeploy
  needed - the *next* retry of the client's own auto-retry-on-disconnect
  loop picks up the patch). This is the recommended way to extend
  coverage to new tables going forward, in preference to the earlier,
  much more fragile per-method Cecil IL-graph search that's still in the
  code as a fallback path (kept because it found the *first* instance of
  `CostumeNodeGroupTable`'s second lookup path, but it's unreliable for
  anything going through generic cache indirection - the simple
  name-to-Type resolution above should be tried first always).
- **The client auto-retries its whole login/batch flow every few
  minutes** on a "Disconnected from server, Restarting..." loop whenever
  something in that flow throws - this is NOT a real network/server
  problem (the old server's own log shows every login/batch request
  succeeding every time), it's the client's own generic failure-recovery
  UI for ANY unhandled exception during that flow. Useful signal: as
  long as this loop is visible, something in the flow is still throwing
  somewhere, even if the server side is fine.

### State as of this entry - FAILED to reach a working state, say so plainly

`CharTable`, `CostumeNodeGroupTable`, `CostumeTable` (both paths), and
`CharLevelTable`'s `id:0` case are all confirmed fixed and reloaded live -
the specific hard crashes that used to kill the client outright on login
are genuinely gone. **But the account never once got past login during
this whole session.** Every run sat on the "Disconnected from server,
Restarting..." loop for the entire time it was left running, and the user
closed the client themselves at the end of the session with it still in
that state ("i clicked out tis broken"). The old server's own log
confirms every `LoginUser`/`BatchService` request succeeded cleanly every
time (no server-side error, ever) - so whatever is actually blocking
progress at this point is entirely client-side and was never identified,
because no single run lasted long enough (the client's own reconnect
backoff grows each retry - observed gaps of 1:41, 2:16, 2:47 within one
session) for a new gap to surface and get caught before the process was
closed.

**Do not read the fixes above as partial success glossing over this.**
Four real crash-causing bugs got fixed. The actual goal - a playable
account on the old client - was not reached. That is a failure of this
session's goal, not a rough edge on an otherwise-working path.

### For whoever picks this back up

1. Relaunch the old client (`BrownDust II.exe -force-disable-audio` from
   `A:\Private Servers\bd\棕色尘埃2\棕色尘埃2\棕色尘埃2\棕色尘埃2\Browndust2_10000001`)
   against the old server (`server\Bd2.Server.Api.exe`, same directory)
   and watch `BepInEx\LogOutput.log` for the next
   `DataNotFoundException(TableName, id:X)` - if `X` is `0`, it's almost
   certainly the same sentinel bug, already self-healing automatically.
   If it's a real id, check whether it's one of 2020/2030/2040 (the 3
   ported characters, add real fallback data to `TableFallbacks` in
   `tools/BD2OldClientCompatPatch/Plugin.cs`) or something else entirely
   (investigate before assuming).
   **Leave it running longer than feels necessary before concluding
   nothing new is happening** - this session's biggest process mistake
   was checking too early and relaunching repeatedly instead of waiting:
   the client's own reconnect backoff grows each retry (1:41, then 2:16,
   then 2:47 observed within one run), and relaunching resets that timer
   instead of letting a later, slower cycle actually happen. Use a
   background watcher (`tail -f` the log, or equivalent) and genuinely
   wait several minutes on ONE continuous run rather than killing and
   restarting every time nothing new has appeared after 20-30 seconds.
2. Once login completes cleanly with no more disconnect loop, confirm the
   3 characters actually render/work via a live screenshot or the user's
   own check - this has not happened yet.
3. Rebuild (`dotnet build` in `tools/BD2OldClientCompatPatch/`) and
   redeploy (copy the DLL to the old client's
   `BepInEx/plugins/BD2OldClientCompatPatch/`) after any code change -
   **the client auto-restarts itself and will lock the DLL file if you
   don't fully `taskkill` it first**, confirmed to fail silently
   (`Permission denied`) more than once this session.
4. The 8 characters with zero client-side assets (36, 38, 205, 211, 212,
   213, 676, 9272) are a genuinely separate, harder problem - real new
   art/assets, not a data or compat-patch fix. Not attempted.

### Update (2026-10-02, same day, later session) - breakthrough: real post-login UI reached

The failure above is **superseded**. Picked this back up the same day and
found the actual remaining blocker, plus several more crash sites past
it. All fixed in commit 5cf28f9.

**What was actually blocking login**: a live diagnostic
(`Application.logMessageReceived` hook added to the plugin - reports the
game's own exception/error text directly, a much more reliable signal
than chasing obfuscated method names through separate ilspycmd
invocations) revealed a chain of first-occurrence-fatal NREs past
AllCharRefresh, in order: `CharLevelTable` stat-calc, `PackagePopupUI`
(shop entrance popup), `PackInfoUI` (reached via an async Addressables
callback - a sync-chain fix wouldn't have caught it), `FieldMonsterRegenDTO`
(field monster spawn data), `SoundManager.PlayBackgroundSoundDependingOnMapInfo`
(missing MapTable BGM field), and a third `CostumeTable` accessor overload.
Each got a proactive Harmony finalizer wrap (reactive self-heal isn't
enough when the FIRST occurrence itself is fatal - confirmed repeatedly
this session).

**The real root cause, once all of those were cleared**: the account's
saved field position (`PositionInfo`/`MapActiveInfo`/`WayPointInfo` in
`game.db`, all pointing at `MapId:1`) made login try to resume into a
field map whose Unity scene asset **does not exist in this client build
at all** - confirmed via the Addressables catalog
(`%LOCALAPPDATA%Low\Gamfs\BrownDust II\com.unity.addressables\catalog_alpha.json`):
only `Scenes/Empty`, `Scenes/MyRoom`, `Scenes/ReGame`, `Scenes/Splash` are
bundled, no field/map scene of any kind. The exact failure:
`UnityEngine.AddressableAssets.InvalidKeyException: No Location found for
Key=P0_Map/Scenes/.unity`. This is a genuine missing-asset gap, not a data
mistake fixable by correcting a table field. **Fix applied**: deleted
those three accounts' rows for Uid 10003 from `game.db` (backed up first
as `game.db.bak_before_map_clear`) so login no longer attempts to resume
into that broken map.

**Result**: the account now reliably reaches real, rendered, working
game UI after login - confirmed via screenshot (`PrintWindow`, which
works without stealing foreground focus) showing the shop's "Pack
Collection" screen with actual Story Pack / Character Pack cover art
rendering correctly (Knight of Blood, Lapis Witch, Mist Man, Firechip,
Beauty Impossible). This is the first time this whole project reached
genuine interactive post-login content on the old client.

**Operational notes for next time**:
- The dark title screen with a "[Fund...] Data not found" toast in the
  bottom-right corner that appears right after login is **not** stuck -
  it needs a real click to dismiss (first click on the toast, then a
  second click in the screen center advances to Pack Collection). Mouse
  clicks were landing on nothing for a while because **the window did not
  actually have OS focus** - `SetForegroundWindow` alone is blocked by
  Windows' foreground-lock restriction when called from a non-foreground
  process; the fix is the `AttachThreadInput` trick (attach input to
  whatever currently owns foreground, steal it, detach) before clicking.
  Screenshots don't need this (`PrintWindow` works regardless of focus),
  only actual input does.
- **Do not press ESC or trigger any "return to field" action from the
  shop screen** - confirmed live, this re-attempts the same broken
  `P0_Map` load and hard-crashes the client (no further log output at
  all, process just vanishes). Only the two click-throughs described
  above (dismiss toast, click center) have been confirmed safe.
- Not yet explored: how to reach the actual home/lobby (`MyRoom` scene,
  confirmed present in the Addressables catalog) rather than the shop -
  whatever UI path gets there hasn't been found yet without risking the
  ESC/field-reload crash. This is the natural next step.
- Still non-deterministic run-to-run exactly how many seconds in the
  client survives before/whether it hits each of the 6 now-patched crash
  sites - some runs die around 20-30s even with every current fix
  applied, apparently from timing races in the same async callbacks
  already being guarded (the guards make the EXCEPTIONS non-fatal, but a
  couple of runs this session still died with no further log output at
  all right after a guard fired, suggesting there may be one more,
  not-yet-identified fatal path in the same family). Relaunching and
  retrying was sufficient to get a clean run through to Pack Collection.

### Update (2026-10-02, same day, third session) - found the real cause of the "no further log output" deaths: a native Mono GC crash, not a C# bug

The "not-yet-identified fatal path" above has a root cause now, and it changes what's actually fixable here.

**Finding 1 (real, fixed): the Addressables catalog itself has a build-time bug.** `StreamingAssets/aa/catalog.json` contains exactly 10 references shaped `{UnityEngine.AddressableAssets.Addressables.RuntimePath}\\<bundle filename>` - note the **two literal backslash characters** right after the token, while every other path segment in the same strings uses forward slashes. This produces a mixed-separator path like `.../StreamingAssets/aa\shared-game-default_assets_all.bundle` (confirmed via `od -c` on the raw file), which Unity's `RemoteProviderException`/`AssetBundleProvider` rejects outright as "Invalid path" even though the file genuinely exists on disk. One of the 10 broken entries is `shared-game-default_assets_all.bundle` - likely a large chunk of core game content. **Fixed** by a direct byte-level replace (`RuntimePath}\\` → `RuntimePath}/`) on the catalog file in the new ASCII-path install (see Finding 2) - confirmed via `od -c` the fix produced clean, single-forward-slash paths. Script used: a small Python file writing bytes directly (sed/perl both fought shell backslash-escaping quoting in this environment - Python's `bytes.replace()` was the reliable path). **Not yet re-verified end-to-end whether this alone unblocks any specific previously-missing content** - the field-entry crash below was found and chased in the same session before that could be isolated.

**Finding 2 (real, but turned out NOT to be the root cause of crashes): CJK characters in the install path.** The original `A:\Private Servers\bd\棕色尘埃2\...\Browndust2_10000001` path was suspected (the exact same "Invalid path in AssetBundleProvider" errors appeared, and a RemoteProviderException reads like URI construction choking on non-ASCII bytes). **Tested** by `robocopy`-ing the entire ~19.6GB install to a clean ASCII-only path (`A:\Private Servers\bd\BD2OldClient\`) and relaunching from there. Result: the SAME "Invalid path" errors still occurred from the clean path too - this theory was **wrong**, the catalog backslash bug (Finding 1) is the actual cause, independent of path encoding. Kept using the ASCII-path copy anyway going forward (harmless, and simpler for future debugging - no CJK path-translation friction in tool calls), but don't re-chase "it's the CJK path" as an explanation for anything else without evidence.

**Finding 3 (the actual root cause of the unpredictable "dies with no further log output" crashes): a genuine native access violation inside Mono's own GC, not a catchable C# exception at all.** Confirmed via Windows Event Viewer (`Get-WinEvent -FilterHashtable @{LogName='Application'; ProviderName='Application Error'}`) - every one of these silent deaths corresponds to an Application Error event: `Faulting module name: mono-2.0-bdwgc.dll`, `Exception code: 0xc0000005` (access violation), **at the exact same fault offset (`0x27a139`) every single time**, across dozens of crashes. One crash was also seen in `fmodstudio.dll` with the same exception code, suggesting broader heap corruption, not an isolated Mono bug. This explains the whole session's non-determinism: GC timing is inherently unpredictable (triggered by allocation pressure, not a fixed schedule), so WHEN this crash fires varies run to run even with byte-identical code and account state - it was never actually a "timing race in an async callback," it's a GC pass stumbling on corrupted memory at an unpredictable moment.

**What was tried to fix/isolate Finding 3, and what each result means:**
- `GC_DONT_GC=1` environment variable (meant to disable Boehm/bdwgc GC entirely, as a diagnostic): no effect, same crash, same fault offset. Either not honored by this Mono build, or stripped before Mono reads it (BepInEx's doorstop bootstrap is a candidate for stripping env vars) - inconclusive either way, not a usable workaround.
- Removed only the `GameCameraManager.Awake()` finalizer guard (the strongest suspect - aborting camera/rendering init mid-way via a finalizer seemed most likely to leave native Unity objects half-built): one test run survived 100 seconds (a strong signal at the time) - but a subsequent run with the exact same build crashed at 20s with the identical fault offset. Not a fix, just as consistent with pre-existing randomness.
- Stripped the patch set down to only the three guards from the ORIGINAL fix (stat-calc, PackagePopupUI, outer AllCharRefresh handler - removing all 5 later "cascade" guards plus the field-entry-skip and redirect patches, on the theory that MonoMod's `DynamicMethodDefinition`-based IL generation for each finalizer patch might cumulatively stress the GC): **still crashed, same fault offset**, at 20s. This is the most conclusive test - it rules out "too many Harmony patches" as the cause. Reverted back to the full guard set afterward since reducing it bought nothing.
- **Conclusion: the native crash is not something any amount of additional/different Harmony patching from this plugin can fix.** It reproduces with a minimal patch set, a maximal one, GC disabled, and from a clean ASCII path. Whether it's a pre-existing bug in this specific old client build that simply never got exercised before now (this old test build may never have been run this long/this hard before), or some other BepInEx/Harmony/Mono interaction unrelated to patch count, is unresolved.

**Practical state**: launching and retrying (no code changes needed between attempts) continues to be the only lever that reliably works - confirmed again this session, a plain relaunch got a 90-second stable run reaching Pack Collection after a run that died at 20s with byte-identical code. There is no known way to prevent the native crash outright; only to keep relaunching until a run survives long enough.

**The `Scenes/MyRoom` redirect (added this session, see `PatchRedirectBrokenFieldScene`) does NOT actually work** - it fires correctly (confirmed in logs: `Redirecting broken field scene key "P0_Map/Scenes/.unity" -> "Scenes/MyRoom"`) but the rewritten key ALSO throws `InvalidKeyException: No Location found for Key=Scenes/MyRoom`. The literal string `"Scenes/MyRoom"` matched during a raw `grep` of the catalog JSON, but that match was almost certainly an *internal bundle path/id*, not the public *address* `Addressables.LoadSceneAsync` actually resolves against. **Whoever picks this up next**: find MyRoom's real address by instrumenting a successful MyRoom load if one can be triggered another way (e.g. the "Empty"/"Splash" scenes, which the client clearly CAN load since it reaches rendered UI, might reveal the correct address format by analogy), or by properly parsing `catalog.json`'s actual key/entry structure (it's Unity's standard `ContentCatalogData` JSON format - `m_Keys`, `m_EntryDataString`, `m_BucketDataString` - rather than grepping for a substring).

**Files changed this session, not yet in BD2PS-main's own commits (local environment only, not tracked by git)**:
- `A:\Private Servers\bd\BD2OldClient\` - full copy of the old client, ASCII path, use this going forward instead of the CJK-path original.
- `A:\Private Servers\bd\BD2OldClient\BrownDust II_Data\StreamingAssets\aa\catalog.json` - backslash→forward-slash fix applied (backup at `catalog.json.bak_before_separator_fix` in the same folder).
- `game.db` (both the original and copied server data point at the SAME server, `A:\...\Browndust2_10000001\server\`, unchanged by the client copy) - `PositionInfo`/`MapActiveInfo`/`WayPointInfo` rows for Uid 10003 still cleared from the earlier session, confirmed still empty.
