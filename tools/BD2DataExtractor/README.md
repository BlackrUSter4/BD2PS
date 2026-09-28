# BD2DataExtractor

A BepInEx/Harmony plugin that captures live game-table data straight out of the
running PC client, so we don't have to reverse-engineer the client's local
cache/encryption format at all.

## Why this exists

Brown Dust 2's static "exceldb" table data (item defs, quest defs, etc. — the
stuff that feeds `data/tables/*.json` in this repo) is **not** bundled in the
client build. It's not in `Assembly-CSharp.dll`, and it's not listed in the
Addressables `catalog.json`. It gets fetched/decrypted at runtime and held in
memory by the client's own `RawDataManager` class, using ordinary
`Google.Protobuf` message types under the `Proto.Design.*` namespaces (one
namespace per "db", e.g. `Proto.Design.common`, `Proto.Design.pack1`, ...).

Rather than reverse-engineer whatever local cache/encryption scheme the client
uses, this plugin uses Harmony to patch every `Proto.Design.*` message type's
generated `MergeFrom(CodedInputStream)` method (the method Google.Protobuf's
C# runtime calls once per row while decoding a repeated table field). The
postfix hook just records the fully-populated row object — decrypted and
parsed by the game's own code — then serializes everything to JSON using
`Google.Protobuf.JsonFormatter.Default`, which produces the same lowerCamelCase
shape already used in `data/tables/*.json`.

## How to build

```
cd tools/BD2DataExtractor
dotnet build -c Release -p:GameDir="A:\Neowiz\Browndust2\BrownDust2_10000002"
```

Produces `bin/Release/BD2DataExtractor.dll`.

## How to deploy

1. Install BepInEx 5.4.x (Windows x64, **Mono** build — this client uses the
   Mono scripting backend, not IL2CPP; confirmed via `MonoBleedingEdge/` and a
   plain managed `Assembly-CSharp.dll`) into the game's install root by
   extracting the release zip there. This adds `winhttp.dll`,
   `doorstop_config.ini`, `.doorstop_version`, and a `BepInEx/` folder — all
   trivially removable to uninstall.
2. Copy `BD2DataExtractor.dll` into
   `<game root>\BepInEx\plugins\BD2DataExtractor\BD2DataExtractor.dll`.
3. (Optional but recommended) create
   `<game root>\BepInEx\config\BepInEx.cfg` with:
   ```
   [Logging.Console]
   Enabled = true
   [Logging.Disk]
   Enabled = true
   ```
   so you get a visible console window confirming the plugin loaded.
4. Launch the game normally (however you normally start it — Doorstop hooks in
   via `winhttp.dll` before Unity even boots, so the launch method doesn't
   matter) and log in with a real account.
5. Play/browse for a bit. Output lands in `<game root>\ExtractedTables\*.json`,
   one file per table type, auto-refreshed every 30s while the game runs.

## Force-loading every database (avoids needing to click through the whole game)

`RawDataManager` (global namespace, `Assembly-CSharp.dll`) has a generic
per-database async loader: `DBLoad(dbNameStruct, Action callback)`, where the
struct comes from `GetDBName(dbTypeEnum, packId)`. The enum has members
`DB_COMMON`, `DB_BLOCK`, `DB_FILED_OBJECT_SCENE` (sic — typo in the game's own
code), `DB_PACK` (needs an explicit pack id, e.g. 1001, 3012, ... — otherwise
it defaults to whatever pack the player currently has selected), `DB_INTRO`,
`DB_CACHE`, `DB_CLIENT_LOCAL`. There's also an `AllLoadExcel()` method that
looked like an obvious one-call solution but decompiles to an empty stub —
dead code, don't bother with it.

The plugin now calls `DBLoad` reflectively for every db id (`DB_COMMON`,
`DB_BLOCK`, `DB_FILED_OBJECT_SCENE`, `DB_INTRO`, and every known `pack*` id)
a few seconds after the `RawDataManager` singleton instance becomes
resolvable, instead of only relying on whatever the player happens to
navigate to. The singleton instance is located by matching the static
property's *signature* on the closed generic `gamfs.Singleton<RawDataManager>`
type (returns `RawDataManager`, static, no params) rather than by its
obfuscated name, since obfuscated identifiers aren't safe to hand-transcribe.

This is more invasive than the passive parse-hook (it's actively calling into
client internals outside their normal call sites/timing) — worst case
observed risk is a crash/hang, recoverable by restarting the game; it never
writes anything, so no save-data risk.

## Gotchas already hit (don't redo this debugging)

- **Don't use `UnityEngine.Input` / `MonoBehaviour.Update()` for a hotkey or
  polling loop.** This project appears to run on Unity's newer Input System;
  the legacy `Input` class did not reliably fire and no hotkey/timer logic
  living in `Update()` ever ran (confirmed: zero log output past plugin init
  even after minutes of play and repeated key presses). Fixed by using a
  plain `System.Threading.Timer` started in `Awake()` — it doesn't touch any
  Unity API and isn't affected by which Input System backend is active.
- **Don't rely solely on `OnApplicationQuit` to flush data.** Also confirmed
  unreliable in practice — closing the game window did not trigger it (this
  game's platform wrapper, PlayPcSdk / Google Play Games, likely tears down
  the process without going through Unity's normal quit path). The periodic
  timer dump is the primary mechanism; quit-time dump is just a bonus.
- Harmony patching ~3790 `Proto.Design.*` message types at `Awake()` works
  fine and is fast (well under a second observed in practice).
- Master/reference tables (most of `common.db`) load in full at login
  regardless of account progress — they're needed for tooltips/shop
  previews/locked-content display. Per-character `pack*.db` battle/skill data
  is more progress-gated, but often still loads for characters shown in a
  roster/gacha-preview screen even if unowned.

## After capturing data

`ExtractedTables/*.json` on the client machine is the raw dump — it still
needs to be merged into this repo's `data/tables/*.json`, and
`data/src/exceldb/*.rs` (marked "auto-generated... do not edit manually", but
the original generator script is not in this repo — write one, or hand-port)
needs regenerating to match any new/changed fields. See the repo root
`CLIENT_UPDATE.md` for the full picture of what else changed in this client
patch (new tables, new pack DBs, new network messages) and what's left to do.
