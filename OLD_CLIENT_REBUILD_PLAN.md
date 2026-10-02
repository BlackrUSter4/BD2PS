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

### State as of this entry - genuinely unresolved, do not claim otherwise

`CharTable`, `CostumeNodeGroupTable`, `CostumeTable` (both paths), and
`CharLevelTable`'s `id:0` case are all confirmed fixed and reloaded live.
But the client was still showing the "Disconnected from server,
Restarting..." loop as of this entry, with a background monitor armed to
catch whatever table gap surfaces next (this flow clearly touches more
tables than the 4 found so far - `CharLevelTable` was only found because
`CharTable`/`CostumeTable` fixing let the flow get one step further, and
the same will likely happen again). **Nothing here has been confirmed
working end-to-end yet (no screenshot has shown anything past this same
disconnect screen with all current fixes applied) - do not mark this
resolved until a live screenshot shows the account actually past login,
ideally with the 3 characters visible.**

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
