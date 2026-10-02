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
