using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;
using System.Reflection.Emit;
using BepInEx;
using BepInEx.Logging;
using HarmonyLib;
using Proto.Design.common;
using UnityEngine.ResourceManagement.AsyncOperations;
using UnityEngine;
using UnityEngine.U2D;
using UnityEngine.UI;
using UnityEngine.EventSystems;
using gamfs;

namespace BD2CompatPatch
{
    // Real client bug (not ours): RawDataManager's many "GetXByY(...)" lookups return `null`
    // instead of an empty List<T> when zero rows match, and several callers never null-check
    // before using the result (e.g. `list.AddRange(GetMercernaryScoutInfoByType(Normal))`
    // NullReferenceException when the real, current production MercenaryScoutTable happens to
    // have zero rows of that type). ~93 sibling methods share the exact same shape. Rather than
    // chasing each one down as it's hit, patch every List<T>-returning public method on
    // RawDataManager at once: if it returns null, replace it with an empty list of the right
    // element type instead.
    [BepInPlugin("bd2.compatpatch", "BD2 Compat Patch", "1.0.0")]
    public class Plugin : BaseUnityPlugin
    {
        internal static ManualLogSource Log;

        // Keeps one-shot System.Threading.Timer instances (the LoadCameraAsset watchdog below)
        // alive until they fire -- a Timer with no other live reference is eligible for GC at
        // any point, which would silently cancel it before its callback ever runs.
        private static readonly System.Collections.Concurrent.ConcurrentDictionary<System.Threading.Timer, byte> _loadCameraAssetWatchdogs = new();

        // Self-reference so static Harmony patch methods can start coroutines on this
        // MonoBehaviour (BepInEx only ever creates one instance of a given plugin).
        private static Plugin _instance;

        private void Awake()
        {
            Log = Logger;
            _instance = this;
            var harmony = new Harmony("bd2.compatpatch");
            int patched = 0;

            var targets = typeof(RawDataManager)
                .GetMethods(BindingFlags.Public | BindingFlags.Instance | BindingFlags.DeclaredOnly)
                .Where(m => m.ReturnType.IsGenericType
                         && m.ReturnType.GetGenericTypeDefinition() == typeof(List<>)
                         && !m.IsGenericMethodDefinition)
                .ToList();

            var postfix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(NullListPostfix), BindingFlags.Static | BindingFlags.NonPublic));

            foreach (var m in targets)
            {
                try
                {
                    harmony.Patch(m, postfix: postfix);
                    patched++;
                }
                catch (Exception e)
                {
                    Log.LogWarning($"[BD2CompatPatch] Failed to patch {m.Name}: {e.Message}");
                }
            }

            Log.LogInfo($"[BD2CompatPatch] Patched {patched} RawDataManager list-returning methods against null-instead-of-empty-list results.");

            // NetworkConnectivityMonitor does its own raw-socket connectivity probe (google.com/
            // apple.com/1.1.1.1 on port 80, bypassing any proxy) independent of anything we
            // control, and something in the login/intro flow appears to gate on its status
            // before showing/enabling Start Game. HarmonyX refused to patch the enum-returning
            // status property directly (int-for-enum __result isn't supported here), so patch
            // every plain-bool surface instead — whichever one gates the UI, this covers it.
            try
            {
                var monitorType = AccessTools.TypeByName("NetworkConnectivityMonitor");
                var alwaysTruePostfix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(AlwaysTruePostfix), BindingFlags.Static | BindingFlags.NonPublic));
                int boolPatched = 0;
                foreach (var m in monitorType?.GetMethods(BindingFlags.Public | BindingFlags.Instance | BindingFlags.DeclaredOnly) ?? Array.Empty<MethodInfo>())
                {
                    if (m.ReturnType == typeof(bool) && m.GetParameters().Length == 0 && !m.IsSpecialName)
                    {
                        harmony.Patch(m, postfix: alwaysTruePostfix);
                        boolPatched++;
                    }
                }
                foreach (var p in monitorType?.GetProperties(BindingFlags.Public | BindingFlags.Instance) ?? Array.Empty<PropertyInfo>())
                {
                    if (p.PropertyType == typeof(bool) && p.GetGetMethod() != null)
                    {
                        harmony.Patch(p.GetGetMethod(), postfix: alwaysTruePostfix);
                        boolPatched++;
                    }
                }
                Log.LogInfo($"[BD2CompatPatch] Patched {boolPatched} NetworkConnectivityMonitor bool members to always report connected/available.");
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch NetworkConnectivityMonitor: {e.Message}");
            }

            // REMOVED (was a temp diagnostic, turned out to be an active bug): patching the
            // closed <SpriteAtlas> instantiation of this generic loader via Harmony/MonoMod
            // causes Mono's shared generic code for reference types to route EVERY other
            // reference-type instantiation of the same generic method through this same patched
            // path too -- confirmed by the fact that literally every "missing asset" error this
            // session (fonts, TitleVoice audio, UIBlur material, MapTypeIcon PNGs, even unrelated
            // .prefab loads) reported "which is not assignable from the requested
            // Type=UnityEngine.U2D.SpriteAtlas": they were never actually missing, they were being
            // misrouted into a SpriteAtlas-typed load path by this patch and failing a type check
            // that has nothing to do with their real (working) type. Removing this patch entirely
            // is the actual fix for the "white box" icons, not a workaround.

            // THE actual silent-hang root cause: GameCameraManager.LoadCameraAsset loads 120
            // battle-camera/cutscene timeline assets and spins `while (loadedCount < 120) yield
            // return ...` waiting for a counter that only increments in the per-asset SUCCESS
            // callback (LoadedTimelineAsset). Several of those 120 keys are missing from our
            // Addressables catalog and fail, so the success callback never fires for them and
            // the counter never reaches 120 -- an infinite, completely silent loop.
            //
            // CORRECTION (found later the same project, after the fix below caused a full
            // regression): the original fix here skipped the ENTIRE coroutine outright, on the
            // theory that "these timelines are already degraded/missing regardless." Wrong --
            // skipping the whole method means NONE of the 120 slots ever load, not just the
            // broken ones, since the coroutine's own request-kickoff loop (the part that
            // actually calls LoadAssetAsync for each of the 120 real, mostly-WORKING assets)
            // never runs either. Confirmed live: every single cutscene in the game (story
            // Quest_Main_XX timelines, not just battle cameras) started rendering as a plain
            // black screen -- PlayDirector/GetTimelineWaitForSeconds's own safe-fallback logic
            // (below) was firing for every timeline index, every time, because the backing
            // array was permanently empty.
            //
            // Real fix: let the real coroutine run (do NOT skip it), so the 115+ genuinely
            // available timeline assets actually load and cutscenes work again. Instead, guard
            // only against the specific failure mode (a few keys that never resolve, forever)
            // with a plain time-based watchdog: a few seconds after LoadCameraAsset starts, if
            // the loaded-count field still hasn't reached 120, force it to 120 directly via
            // reflection. The coroutine's own `while (loadedCount < 120) yield return ...` loop
            // re-checks that field every frame regardless of what set it, so this unblocks the
            // wait without needing to touch the coroutine's compiler-generated state machine at
            // all. Whichever specific slots never loaded stay null in the timeline array --
            // already handled safely, per-index, by the PlayDirector/GetTimelineWaitForSeconds
            // prefixes below (that's the mechanism that was supposed to be handling this all
            // along).
            try
            {
                var loadCameraAsset = AccessTools.Method(typeof(GameCameraManager), "LoadCameraAsset");
                if (loadCameraAsset != null)
                {
                    var watchdogPrefix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(StartLoadCameraAssetWatchdogPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(loadCameraAsset, prefix: watchdogPrefix);
                    Log.LogInfo("[BD2CompatPatch] Patched GameCameraManager.LoadCameraAsset with a load-count watchdog instead of skipping it outright.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find GameCameraManager.LoadCameraAsset to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch GameCameraManager.LoadCameraAsset: {e.Message}");
            }

            // Project-wide helper: `ThrowIfNull<T>(row, () => new DataNotFoundException(...))`,
            // used everywhere a master-data table lookup might come back null for our (incomplete)
            // captured production data mirror. It doesn't actually throw -- it shows a visible
            // error popup/log (gated by a debug flag) and returns normally, after which the
            // caller's very next line dereferences the still-null row and crashes anyway (already
            // absorbed harmlessly by the CrashReporter patch above). This is the single choke
            // point behind every "DataNotFoundException" popup seen this session (MercenaryScout,
            // PackTable, now TalentTable) -- skip the whole method so the popup never shows,
            // leaving only the same silent, already-handled crash underneath.
            // Patching ThrowIfNull<T> itself hits the same "IL Compile Error" this session found
            // for every other open generic method definition -- this HarmonyX build can't patch
            // open generics at all. Instead, patch the concrete (non-generic) popup method it
            // calls: ThrowIfNull passes the exception TYPE NAME as the popup's title, formatted
            // as exactly "[DataNotFoundException]" -- check for that specific title so only these
            // calls are skipped; every other legitimate popup (confirmations, warnings, tutorial
            // hints) that goes through the same method is untouched.
            try
            {
                var showPopup = AccessTools.Method(typeof(ὩὭὨὪὨὨὮὣὪὣὥ), "ὡὫὤὤὮὮὨὩὤὮὮ");
                if (showPopup != null)
                {
                    var skipDataNotFoundPopup = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SkipDataNotFoundPopupPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(showPopup, prefix: skipDataNotFoundPopup);
                    Log.LogInfo("[BD2CompatPatch] Patched the message-popup method to suppress only DataNotFoundException popups.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find the message-popup method to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch the message-popup method: {e.Message}");
            }

            // A fresh/empty-progression account has no talent skills equipped, and
            // TalentSkillQuickSlot.SetSlot doesn't null-check before using whatever data it
            // expects to find for an equipped skill -- throws and aborts the rest of
            // GameFieldDefaultUI.Init() (currency, quest counts, event banner, etc. all come
            // after it). This quick-slot bar has nothing to show for a zero-progression account
            // anyway, so skip its init entirely rather than fix data completeness for a feature
            // that's correctly empty.
            try
            {
                var quickSlotContainType = AccessTools.Inner(typeof(GameFieldDefaultUI), "TalentSkillQuickSlotContain");
                var quickSlotInit = quickSlotContainType?.GetMethods(BindingFlags.Public | BindingFlags.Instance)
                    .FirstOrDefault(m => m.GetParameters().Length == 0 && m.ReturnType == typeof(void));
                if (quickSlotInit != null)
                {
                    var skipPrefix2 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SkipMethodPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(quickSlotInit, prefix: skipPrefix2);
                    Log.LogInfo("[BD2CompatPatch] Patched TalentSkillQuickSlotContain init to skip (empty for zero-progression accounts).");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find TalentSkillQuickSlotContain init method to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch TalentSkillQuickSlotContain: {e.Message}");
            }

            // Same root cause, next step in Init(): RefreshTalentAvailable() also assumes talent
            // data exists for the account's characters and doesn't null-check. Skip it too --
            // "is a talent available" has nothing to show for zero-progression characters anyway.
            try
            {
                var refreshTalent = AccessTools.Method(typeof(GameFieldDefaultUI), "RefreshTalentAvailable");
                if (refreshTalent != null)
                {
                    var skipPrefix4 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SkipMethodPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(refreshTalent, prefix: skipPrefix4);
                    Log.LogInfo("[BD2CompatPatch] Patched GameFieldDefaultUI.RefreshTalentAvailable to skip (empty for zero-progression accounts).");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find RefreshTalentAvailable to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch RefreshTalentAvailable: {e.Message}");
            }

            // Same underlying talent-availability chain is ALSO called from a second private
            // Init() helper (not just RefreshTalentAvailable), so patching that one call site
            // wasn't enough -- the exact same crash still happened from the other caller. Patch
            // the actual root method (3 levels down the call chain, the true "does this character
            // have this talent" check with no null-guard) directly instead, which covers every
            // caller project-wide in one patch rather than chasing each one individually.
            try
            {
                var talentUtilType = AccessTools.TypeByName("ὨὣὠὨὯὬὯὥὤὢὣ");
                var refreshAllTalents = talentUtilType?.GetMethod("ὤὥὤὯὥὯὡὤὢὧὬ", BindingFlags.Public | BindingFlags.Static);
                if (refreshAllTalents != null)
                {
                    var skipPrefix5 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SkipMethodPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(refreshAllTalents, prefix: skipPrefix5);
                    Log.LogInfo("[BD2CompatPatch] Patched the talent-availability refresh root method to skip (empty for zero-progression accounts).");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find the talent-availability refresh root method to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch the talent-availability refresh root method: {e.Message}");
            }

            // The bare Component stubs created for chat/channel/avatar-customization fields (see
            // the stubbing logic further down, run when the pack-load coroutine is abandoned)
            // crash the moment any of their OWN methods actually runs internal logic that assumes
            // proper setup (confirmed: MainChat.SetActive threw next, immediately after the
            // NoticeTalentAvailable fix). Rather than discover and patch each such method one
            // crash at a time, neuter every public instance method on these specific
            // known-unsupported feature types up front -- these are chat, channel selection, and
            // avatar customization, none of which our server implements, so no method on them
            // ever needs to do real work.
            try
            {
                int neutered = 0;
                var skipPrefix6 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SkipMethodPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                foreach (var typeName in new[] { "MainChat", "AvatarCharInfo", "HitStatueEventUIController", "AvatarZoomMode" })
                {
                    var t = AccessTools.TypeByName(typeName);
                    if (t == null)
                    {
                        Log.LogWarning($"[BD2CompatPatch] Could not find stub type {typeName} to neuter.");
                        continue;
                    }
                    foreach (var m in t.GetMethods(BindingFlags.Public | BindingFlags.Instance | BindingFlags.DeclaredOnly))
                    {
                        if (m.IsGenericMethodDefinition || m.IsAbstract || m.IsSpecialName) continue;
                        try
                        {
                            harmony.Patch(m, prefix: skipPrefix6);
                            neutered++;
                        }
                        catch { /* best-effort; some methods (generics, etc.) just won't patch */ }
                    }
                }
                Log.LogInfo($"[BD2CompatPatch] Neutered {neutered} methods across unsupported-feature stub types (chat/channel/avatar customization).");
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to neuter stub type methods: {e.Message}");
            }

            // ROOT CAUSE of "every single icon is broken/white": MenuUI.Init() calls UpdateIssue()
            // very early (before SetCurrency/SetPortrait/SetBattlePower/SetNickName/SetMenuGacha/
            // SetMenuStory/SetGuild/SetSkinBanner/etc. all run later in the SAME method), and
            // UpdateIssue() itself calls ~20 sub-updaters in sequence, one of which is
            // UpdateMenuMonthlySubIcon(). That method's first line dereferences _objStandardIcon /
            // _objPremiumIcon, which are found via a by-name child lookup under the package-shop
            // button ("Image - St" / "Image - Pr") with no null-check -- on our server there's no
            // real cash-package/monthly-subscription data, so this throws a NullReferenceException
            // every time. Because C# has no per-statement try/catch here, that exception aborts
            // UpdateIssue() (skipping every later issue-badge updater) AND aborts the rest of
            // MenuUI.Init() (skipping the currency/portrait/battle-power/nickname/gacha/story/
            // guild/skin-banner setup calls that come after UpdateIssue() in Init()). Skipping just
            // this one monthly-subscription-icon method (cosmetic, and genuinely unsupported -- no
            // real IAP backend) lets the entire rest of the menu bar actually initialize.
            try
            {
                var menuUIType = AccessTools.TypeByName("MenuUI");
                var updateMenuMonthlySubIcon = menuUIType?.GetMethod("UpdateMenuMonthlySubIcon", BindingFlags.Public | BindingFlags.Instance);
                if (updateMenuMonthlySubIcon != null)
                {
                    var skipPrefix7 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SkipMethodPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(updateMenuMonthlySubIcon, prefix: skipPrefix7);
                    Log.LogInfo("[BD2CompatPatch] Patched MenuUI.UpdateMenuMonthlySubIcon to skip (no cash-package data on this server; was aborting the rest of MenuUI.Init()).");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find MenuUI.UpdateMenuMonthlySubIcon to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch MenuUI.UpdateMenuMonthlySubIcon: {e.Message}");
            }

            // UpdateMenuMonthlySubIcon was not the only unguarded null in this chain --
            // UpdateStoryTimelineIssue() crashed next (_objStoryIssue null), aborting everything
            // AFTER it in UpdateIssue() (UpdateFriendshipIssue, guild/friend/dating/mini-game-hub
            // badge updates, UpdateCharacterVotingIssue, _packIconBase.SetTotalPackIssue) the same
            // way. Rather than keep whack-a-moling each of the ~27 sub-updaters one crash at a
            // time, replace UpdateIssue() itself with a version that runs each sub-call in its own
            // try/catch, so any one broken/unsupported badge feature can never take down the rest.
            try
            {
                var menuUIType2 = AccessTools.TypeByName("MenuUI");
                var updateIssueMethod = menuUIType2?.GetMethod("UpdateIssue", BindingFlags.Public | BindingFlags.Instance);
                if (updateIssueMethod != null)
                {
                    var safeUpdateIssuePrefix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SafeUpdateIssuePrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(updateIssueMethod, prefix: safeUpdateIssuePrefix);
                    Log.LogInfo("[BD2CompatPatch] Patched MenuUI.UpdateIssue to run each sub-updater in its own try/catch.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find MenuUI.UpdateIssue to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch MenuUI.UpdateIssue: {e.Message}");
            }

            // SetMenuStory() crashed next (right after UpdateIssue() in Init()'s own top-level call
            // sequence), and Init() has ~30 more such calls after it (guild, skin banner, promotion
            // banner, newbie-pass navigator, etc.) -- the exact same "one null aborts everything
            // after it" bug, just at the Init()-body level instead of inside UpdateIssue(). Rather
            // than keep discovering and patching each one via relaunch cycles, wrap EVERY other
            // method MenuUI declares with a Harmony finalizer that swallows any exception at the
            // point it's thrown. Unlike SkipMethodPrefix this does NOT skip the method -- it still
            // runs completely normally, with all its real side effects, every time; the finalizer
            // only engages if that specific method throws, logging it and letting the CALLER (Init,
            // or whatever called it) continue to its next statement as if the call had returned
            // normally. This covers every remaining and future "no data for this widget on a
            // fresh/private-server account" crash in one patch instead of one relaunch per crash.
            try
            {
                var menuUIType3 = AccessTools.TypeByName("MenuUI");
                if (menuUIType3 != null)
                {
                    var swallowFinalizer = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SwallowExceptionFinalizer), BindingFlags.Static | BindingFlags.NonPublic));
                    int wrapped = 0;
                    foreach (var m in menuUIType3.GetMethods(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Instance | BindingFlags.DeclaredOnly))
                    {
                        if (m.IsGenericMethodDefinition || m.IsAbstract || m.IsSpecialName) continue;
                        // Init/UpdateIssue/UpdateMenuMonthlySubIcon already have more precise,
                        // targeted patches above -- leave them alone rather than double-patch.
                        if (m.Name == "Init" || m.Name == "UpdateIssue" || m.Name == "UpdateMenuMonthlySubIcon") continue;
                        try
                        {
                            harmony.Patch(m, finalizer: swallowFinalizer);
                            wrapped++;
                        }
                        catch { /* best-effort; a handful of methods (generics, etc.) just won't patch */ }
                    }
                    Log.LogInfo($"[BD2CompatPatch] Wrapped {wrapped} MenuUI methods with an exception-swallowing finalizer so one broken/unsupported widget can never abort the rest of MenuUI.Init().");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find MenuUI type to wrap with exception-swallowing finalizers.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to wrap MenuUI methods with finalizers: {e.Message}");
            }

            // DIAGNOSTIC: several icons on the now-working Home screen (e.g. the pack-icon badge
            // next to the player name, PackIconBase.SetIcon -> UISprite.SetLocalizeSprite) still
            // render blank. UISprite's sprite-atlas lookup fails completely silently in most code
            // paths (no exception, no log line) when a sprite name doesn't resolve, so it's
            // invisible to every crash-based fix above. SpriteManager.AtlasContainer.GetSprite(name)
            // is the single choke point every atlas-based sprite lookup in the whole game goes
            // through -- log every call and whether it resolved, so we can tell a missing/renamed
            // sprite from a genuinely broken atlas instead of guessing from a blank Image.
            try
            {
                var atlasContainerType = AccessTools.Inner(typeof(SpriteManager), "AtlasContainer");
                var getSpriteMethod = atlasContainerType?.GetMethod("GetSprite", BindingFlags.Public | BindingFlags.Instance);
                if (getSpriteMethod != null)
                {
                    var diagPostfix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(AtlasGetSpriteDiagnosticPostfix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(getSpriteMethod, postfix: diagPostfix);
                    Log.LogInfo("[BD2CompatPatch] Patched SpriteManager.AtlasContainer.GetSprite for sprite-lookup diagnostics.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find SpriteManager.AtlasContainer.GetSprite to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch SpriteManager.AtlasContainer.GetSprite: {e.Message}");
            }

            // DIAGNOSTIC: user reports every button on the Home screen is unresponsive to clicks,
            // even though everything now visually renders. MenuUI.OnClickUI has a private "hidden
            // UI mode" flag checked FIRST, before any button's own handler: if it's true, EVERY
            // click except the back button gets silently rerouted to _menuIllustController.OnClickUI
            // instead of the button's real handler. That flag is set true by the "hide UI" button
            // and is only meant to be cleared by tapping again -- if that clearing path is broken
            // (e.g. by a swallowed exception from our own finalizer wrapper above), every click
            // would silently do nothing, exactly matching what's reported. Log the clicked object's
            // name and the flag's value on every click so the next tap tells us definitively.
            try
            {
                var menuUIType4 = AccessTools.TypeByName("MenuUI");
                var onClickUIMethod = menuUIType4?.GetMethod("OnClickUI", BindingFlags.Public | BindingFlags.Instance);
                var hiddenFlagField = menuUIType4?.GetField("ὢὮὮὨὮὥὩὭὦὮὩ", BindingFlags.NonPublic | BindingFlags.Instance);
                if (onClickUIMethod != null)
                {
                    _menuUiHiddenFlagField = hiddenFlagField;
                    var diagPrefix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(MenuUiOnClickDiagnosticPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(onClickUIMethod, prefix: diagPrefix);
                    Log.LogInfo($"[BD2CompatPatch] Patched MenuUI.OnClickUI for click diagnostics (hidden-flag field {(hiddenFlagField != null ? "found" : "NOT found")}).");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find MenuUI.OnClickUI to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch MenuUI.OnClickUI: {e.Message}");
            }

            // DIAGNOSTIC: user reports the field character-setting popup (FieldCharSettingPopupUI)
            // is completely unresponsive to clicks (Cancel/Confirm/tabs, everything), and the
            // confirm coroutine we just wrapped above never even logs ENTER -- meaning either the
            // click never reaches OnClickUI at all (a raycast blocker, or the global IsUIOpening
            // flag stuck true from some other UI's incomplete open/close animation), or it reaches
            // OnClickUI but the popup's own local "busy" flag is somehow already stuck true before
            // any click happens. Log the clicked object's name plus both flags on every click so
            // the next tap settles which of these it actually is.
            try
            {
                var fieldCharSettingType = AccessTools.TypeByName("FieldCharSettingPopupUI");
                var fcOnClickUIMethod = fieldCharSettingType?.GetMethod("OnClickUI", BindingFlags.Public | BindingFlags.Instance);
                var fcBusyFlagField = fieldCharSettingType?.GetField("ὥὨὪὯὢὮὯὡὫὬὧ", BindingFlags.NonPublic | BindingFlags.Instance);
                var uiManagerType = AccessTools.TypeByName("ὩὭὨὪὨὨὮὣὪὣὥ");
                var isUiOpeningGetter = uiManagerType?.GetProperty("ὭὫὩὬὥὡὦὪὩὠὯ", BindingFlags.Public | BindingFlags.Static)?.GetGetMethod();
                if (fcOnClickUIMethod != null)
                {
                    _fieldCharSettingBusyFlagField = fcBusyFlagField;
                    _isUiOpeningGetter = isUiOpeningGetter;
                    var diagPrefix2 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(FieldCharSettingOnClickDiagnosticPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(fcOnClickUIMethod, prefix: diagPrefix2);
                    Log.LogInfo($"[BD2CompatPatch] Patched FieldCharSettingPopupUI.OnClickUI for click diagnostics (busy-flag {(fcBusyFlagField != null ? "found" : "NOT found")}, IsUIOpening getter {(isUiOpeningGetter != null ? "found" : "NOT found")}).");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find FieldCharSettingPopupUI.OnClickUI to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch FieldCharSettingPopupUI.OnClickUI: {e.Message}");
            }

            // ROOT CAUSE of "the field character-setting popup is completely unresponsive to
            // every click, and the background looks washed out white": TutorialRoot.prefab loads
            // right alongside it every time, and TutorialRoot has a literal full-screen
            // "Image - InputBlock" GameObject (see TutorialRoot._uiInputBlocker) that TutorialManager
            // activates to prevent interaction with anything except whatever it's currently
            // highlighting. Our server has no matching tutorial-step progress/completion data for
            // this account, so the tutorial's own "did the player click the highlighted target"
            // detection never fires -- it activates the full-screen blocker and then never
            // deactivates it, since that only happens in response to the very click it's blocking.
            // That washes out the whole screen (a full-screen Image covering everything) AND
            // silently swallows every click before it ever reaches the real UI underneath. Rather
            // than patch a `false` return into every fragile branch of TutorialManager's flow,
            // stop the blocker from ever actually turning on: patch GameObject.SetActive itself,
            // scoped narrowly to only the GameObject literally named "Image - InputBlock" (this
            // input blocker's own name per TutorialRoot.Reset()), so no other GameObject in the
            // game is affected.
            try
            {
                var setActiveMethod = AccessTools.Method(typeof(GameObject), nameof(GameObject.SetActive));
                if (setActiveMethod != null)
                {
                    var blockTutorialInputBlockerPrefix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(BlockTutorialInputBlockerPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(setActiveMethod, prefix: blockTutorialInputBlockerPrefix);
                    Log.LogInfo("[BD2CompatPatch] Patched GameObject.SetActive to keep the tutorial's full-screen input blocker ('Image - InputBlock') from ever activating.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find GameObject.SetActive to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch GameObject.SetActive for the tutorial input blocker: {e.Message}");
            }

            // Blocking 'Image - InputBlock' wasn't the whole story: TutorialFocusController.Focus()
            // (a SEPARATE coroutine from TutorialRoot, started independently by TutorialManager) has
            // its own full-screen '_background' overlay it activates and never deactivates until its
            // internal `while (!done) yield return null;` loop sees a completion condition -- for
            // ScheduleType.Custom that's a content-completion check (PackPurchasableNewUI /
            // CharUI .IsCompleteForTutorialOnClick()), for Click it's an actual click on the
            // highlighted target via OnFocusBtnPointerClick. On a private server with no matching
            // tutorial-progress data, that condition never becomes true, so `_background` -- a
            // separate raycast-blocking overlay from the one we already blocked -- stays up forever,
            // both washing out the screen white and eating every click underneath it. Skip the whole
            // coroutine so this tutorial step never starts at all.
            try
            {
                var tutorialFocusControllerType = typeof(TutorialFocusController);
                var focusMethod = AccessTools.Method(tutorialFocusControllerType, "Focus");
                if (focusMethod != null)
                {
                    var skipFocusPrefix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SkipFocusCoroutinePrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(focusMethod, prefix: skipFocusPrefix);
                    Log.LogInfo("[BD2CompatPatch] Patched TutorialFocusController.Focus to skip entirely (its '_background' overlay was the real click-blocking/screen-wash layer, stuck forever on unmet tutorial-progress data).");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find TutorialFocusController.Focus to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch TutorialFocusController.Focus: {e.Message}");
            }

            // DIAGNOSTIC: user reports the home-screen first-time tutorial "plays over and over".
            // Log every TutorialManager.Play(id) call plus whether the game already thinks that id
            // is cleared, so we can tell a genuinely-looping single id apart from a long chain of
            // distinct onboarding steps that only looks repetitive.
            try
            {
                var tutorialManagerType = AccessTools.TypeByName("TutorialManager");
                var playMethod = tutorialManagerType?.GetMethod("Play", BindingFlags.Public | BindingFlags.Instance);
                var isClearMethod = tutorialManagerType?.GetMethod("IsClearTutorial", BindingFlags.Public | BindingFlags.Instance);
                if (playMethod != null)
                {
                    var diagPrefix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(TutorialPlayDiagnosticPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(playMethod, prefix: diagPrefix);
                    Log.LogInfo("[BD2CompatPatch] Patched TutorialManager.Play for repeat-tutorial diagnostics.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find TutorialManager.Play to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch TutorialManager.Play for diagnostics: {e.Message}");
            }

            // DIAGNOSTIC: even with both known blockers (tutorial input-block overlay, tutorial
            // focus coroutine) confirmed disabled, FieldCharSettingPopupUI.OnClickUI STILL never
            // fires on a real click (its diagnostic prefix, which runs before any internal state
            // check, never logs) -- meaning the click never reaches that method AT ALL. That points
            // above the UI-framework layer entirely, to Unity's own raycast/event-dispatch step.
            // Hook GraphicRaycaster.Raycast (the actual method every UI click physically goes
            // through) and log what it hits, but ONLY on an actual mouse press/release, to avoid
            // spamming every frame's hover raycasts -- this will show definitively whether clicks
            // are hitting some other blocking object, or not registering as UI raycasts at all.
            try
            {
                var raycastMethod = AccessTools.Method(typeof(GraphicRaycaster), "Raycast", new[] { typeof(PointerEventData), typeof(List<RaycastResult>) });
                if (raycastMethod != null)
                {
                    var raycastDiagPostfix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(GraphicRaycastDiagnosticPostfix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(raycastMethod, postfix: raycastDiagPostfix);
                    Log.LogInfo("[BD2CompatPatch] Patched GraphicRaycaster.Raycast for click-hit diagnostics.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find GraphicRaycaster.Raycast to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch GraphicRaycaster.Raycast: {e.Message}");
            }

            // DIAGNOSTIC: after stripping every known click-catcher from each individual
            // GraphicRaycaster's contribution, clicks STILL never reach FieldCharSettingPopupUI's
            // OnClickUI. EventSystem.RaycastAll is the method that calls every raycaster in turn
            // (feeding them the same accumulating list our patch above already cleaned) and is also
            // responsible for the final sort that decides which single entry wins as the actual
            // click target. Hook it to see what EventSystem itself believes the winning target is,
            // post-filtering -- if it's still not the real button, something other than raycast
            // hit-order (e.g. a distance/depth comparison, or a completely separate physics-based
            // click path) is deciding the outcome.
            try
            {
                var raycastAllMethod = AccessTools.Method(typeof(EventSystem), "RaycastAll");
                if (raycastAllMethod != null)
                {
                    var raycastAllDiagPostfix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(EventSystemRaycastAllDiagnosticPostfix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(raycastAllMethod, postfix: raycastAllDiagPostfix);
                    Log.LogInfo("[BD2CompatPatch] Patched EventSystem.RaycastAll for final winning-target diagnostics.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find EventSystem.RaycastAll to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch EventSystem.RaycastAll: {e.Message}");
            }

            // ROOT CAUSE of the Story screen's loading spinner spinning forever (and, by the same
            // mechanism, any other screen that loads a Spine illustration/prefab by address):
            // confirmed via the log -- "No Location found for Key=UI/Prefabs/Spine/IllustSpecial/
            // SpecialIllust181.prefab" (this specific story illustration is genuinely absent from
            // this server's asset catalog; that's a content gap, not something a patch can create).
            // The REAL bug is how the loader reacts: this completion handler has TWO failure
            // branches -- "op.Status != Succeeded" correctly calls callback(null) and cleans up, but
            // the EARLIER "op.OperationException != null" branch (which is what an InvalidKeyException
            // hits) instead THROWS, so the callback the caller is waiting on to know loading is done
            // (and hide its spinner) never fires. Make the exception branch behave like its sibling:
            // invoke callback(null) and self-destroy instead of throwing, so a missing asset just
            // shows nothing instead of leaving the UI stuck waiting forever.
            try
            {
                var loaderType = AccessTools.TypeByName("ὤὣὪὥὨὡὣὯὧὯὯ");
                var closureType = AccessTools.Inner(loaderType, "ὤὬὡὬὢὮὢὨὯὮὦ");
                var completionMethod = closureType?.GetMethod("ὮὣὭὬὪὫὧὪὪὥὪ", BindingFlags.NonPublic | BindingFlags.Instance);
                if (completionMethod != null)
                {
                    var gracefulFailPrefix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(GracefulPrefabLoadFailurePrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(completionMethod, prefix: gracefulFailPrefix);
                    Log.LogInfo("[BD2CompatPatch] Patched the prefab-load completion handler to call back gracefully on a missing asset instead of throwing (was leaving loading spinners stuck forever).");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find the prefab-load completion handler to patch.");
                }

                // Same exact bug, second confirmed instance -- this one backs GetPrefabAsset itself
                // (the loader used by nearly every UI popup in the game, not just Spine
                // illustrations), so it's likely the single highest-impact copy of this pattern.
                // Both this and the one above take a concrete (non-generic) AsyncOperationHandle
                // <GameObject>, so patching them is safe -- unlike the OPEN GENERIC AsyncOperationHandle
                // <T> completion handlers elsewhere in this same loader class, which must NOT be
                // patched: Harmony/MonoMod sharing the JIT'd code for one closed generic instantiation
                // across every other reference-type instantiation of the same generic method caused
                // the major "everything is SpriteAtlas-typed" regression earlier this session.
                var closureType2 = AccessTools.Inner(loaderType, "ὧὭὮὠὥὥὠὯὩὦὡ");
                var completionMethod2 = closureType2?.GetMethod("ὣὨὧὤὥὪὦὮὯὣὬ", BindingFlags.NonPublic | BindingFlags.Instance);
                if (completionMethod2 != null)
                {
                    var gracefulFailPrefix2 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(GracefulPrefabLoadFailurePrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(completionMethod2, prefix: gracefulFailPrefix2);
                    Log.LogInfo("[BD2CompatPatch] Patched the GetPrefabAsset completion handler to call back gracefully on a missing asset instead of throwing.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find the GetPrefabAsset completion handler to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch the prefab-load completion handler: {e.Message}");
            }

            // See SafePlayDirectorPrefix below for the full story: a not-yet-loaded cutscene
            // timeline shouldn't be able to hang the client on the logo splash screen forever.
            try
            {
                var cameraManagerType = AccessTools.TypeByName("GameCameraManager");
                var playDirectorMethod = cameraManagerType?.GetMethod("PlayDirector", BindingFlags.Public | BindingFlags.Instance);
                if (playDirectorMethod != null)
                {
                    var safePlayDirectorPrefix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SafePlayDirectorPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(playDirectorMethod, prefix: safePlayDirectorPrefix);
                    Log.LogInfo("[BD2CompatPatch] Patched GameCameraManager.PlayDirector to skip a not-yet-loaded cutscene instead of hanging the client on the logo screen.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find GameCameraManager.PlayDirector to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch GameCameraManager.PlayDirector: {e.Message}");
            }

            // Confirmed root cause of "can move and back out but can't enter house": the live IL
            // crash-site diagnostic (LogCrashLocationAndCapturedState) pinpointed the gate-move
            // coroutine's NullReferenceException to a `callvirt GameCameraManager.
            // GetTimelineWaitForSeconds` right after `call GameCameraManager.get_Instance` -- i.e.
            // GameCameraManager.Instance is null at exactly this point (the singleton hasn't been
            // (re)assigned yet, or was cleared, specifically when the gate-move coroutine redirects
            // to the player's own current map rather than a genuinely different one -- a normal
            // cross-map transition apparently re-establishes this singleton reference somewhere
            // else in its own lifecycle before this call, which the "stay where you are" redirect
            // bypasses).
            //
            // FIRST ATTEMPT (prefix on GetTimelineWaitForSeconds itself) did NOT work -- confirmed
            // live, the exception moved to "(wrapper dynamic-method)
            // GameCameraManager.DMD<GameCameraManager::GetTimelineWaitForSeconds>", meaning
            // HarmonyX's own generated dispatch trampoline dereferences/null-checks the "this"
            // instance as part of ITS OWN glue code before a prefix ever gets a chance to run --
            // a prefix can't rescue a call on a genuinely null receiver, only a call whose
            // receiver is real but whose INTERNAL state is broken (which is what
            // SafePlayDirectorPrefix, a similar-looking but different situation, actually relies
            // on). Real fix: don't try to intercept the doomed call at all -- patch
            // GameCameraManager.Instance itself (a postfix) so it's never null in the first place.
            // If the backing static field is null, fall back to UnityEngine.Object.FindObjectOfType
            // -- there's still exactly one real GameCameraManager alive in the scene at this point
            // (this is a MonoBehaviour singleton, not a destroyed object), the static reference to
            // it has just gone stale/unset for this specific redirect-to-current-map code path.
            try
            {
                var cameraManagerType2 = AccessTools.TypeByName("GameCameraManager");
                // GetMethod("get_Instance", ...) directly, NOT GetProperty("Instance",
                // ...)?.GetGetMethod() -- confirmed live just now that the property-level lookup
                // returns null on this assembly ("Could not find GameCameraManager.Instance getter
                // to patch") even though the underlying get_Instance METHOD resolves fine (it's
                // literally in the disassembly above as "call GameCameraManager.get_Instance").
                // Exact same property-vs-method resolution gap already hit on GateSpotData earlier
                // tonight -- Type.GetProperty is unreliable on this assembly, Type.GetMethod("get_X")
                // has worked every single time.
                var instanceGetter = cameraManagerType2?.GetMethod("get_Instance", BindingFlags.Public | BindingFlags.Static);
                if (instanceGetter != null)
                {
                    var fixNullInstancePostfix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(FixNullCameraManagerInstancePostfix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(instanceGetter, postfix: fixNullInstancePostfix);
                    Log.LogInfo("[BD2CompatPatch] Patched GameCameraManager.Instance getter to fall back to FindObjectOfType instead of returning null.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find GameCameraManager.Instance getter to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch GameCameraManager.Instance getter: {e.Message}");
            }

            // Live retest of the Instance-getter fix: it works (many "GameCameraManager.Instance
            // was null -- falling back..." lines during early loading), but none of them are near
            // the gate-move crash -- meaning Instance is genuinely non-null again by the time the
            // gate is touched, and the SAME exact crash still happens anyway. So the NRE isn't
            // about a null Instance at all (that part really is fixed) -- it's inside
            // GetTimelineWaitForSeconds' OWN body, on some other null. Almost certainly the exact
            // same shape as the already-solved SafePlayDirectorPrefix above (a per-transition-type
            // array slot that hasn't loaded for this specific index) -- both methods plausibly
            // share the same backing `_playableDirector`/timeline-array fields. This time a prefix
            // *can* work: __instance is real now (confirmed by the Instance-getter fix), so
            // Harmony's dispatch trampoline has no null "this" to choke on before reaching it.
            try
            {
                var cameraManagerType3 = AccessTools.TypeByName("GameCameraManager");
                var getTimelineWaitMethod2 = cameraManagerType3?.GetMethod("GetTimelineWaitForSeconds", BindingFlags.Public | BindingFlags.Instance);
                if (getTimelineWaitMethod2 != null)
                {
                    var safeGetTimelineWaitPrefix2 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SafeGetTimelineWaitForSecondsPrefix2), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(getTimelineWaitMethod2, prefix: safeGetTimelineWaitPrefix2);
                    Log.LogInfo("[BD2CompatPatch] Patched GameCameraManager.GetTimelineWaitForSeconds (again) to skip an unloaded timeline slot instead of crashing the gate-move coroutine.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find GameCameraManager.GetTimelineWaitForSeconds to patch (second attempt).");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch GameCameraManager.GetTimelineWaitForSeconds (second attempt): {e.Message}");
            }

            // See SkipBundleDownloadFailedPrefix below for the full story: one dead file on
            // Neowiz's real CDN shouldn't be able to abort the whole "Download All Packs" preload
            // and force-restart the client.
            try
            {
                var introType = AccessTools.TypeByName("IntroUI");
                var dispatchMethod = introType?.GetMethod("ὩὮὨὩὣὢὦὠὪὦὨ", BindingFlags.NonPublic | BindingFlags.Instance);
                if (dispatchMethod != null)
                {
                    var skipBundleFailPrefix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SkipBundleDownloadFailedPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(dispatchMethod, prefix: skipBundleFailPrefix);
                    Log.LogInfo("[BD2CompatPatch] Patched IntroUI's download-failure dispatcher to tolerate a single failed bundle instead of restarting the client.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find IntroUI's download-failure dispatcher to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch IntroUI's download-failure dispatcher: {e.Message}");
            }

            // Confirmed via the log: PackIconBase.SetSubPackIcon throws a NullReferenceException for
            // certain pack list entries (a null _spritePvpPackRank/_spriteEventPackLevel reference on
            // a fresh account), and since SetIcon() calls it partway through a sequence of otherwise-
            // independent setup calls (SetSubPackIcon, then progress bar, PvP effect, event-hub
            // banner, play-state text, reward summary, pack level -- see PackIconBase.SetIcon), that
            // one throw silently aborts EVERY step after it for that pack list entry. Same "one
            // broken step kills the rest" shape as MenuUI.Init(); same fix: wrap the whole class with
            // an exception-swallowing finalizer so one bad pack entry can't blank out its neighbors.
            try
            {
                var packIconBaseType = AccessTools.TypeByName("PackIconBase");
                if (packIconBaseType != null)
                {
                    var swallowFinalizer2 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SwallowExceptionFinalizer), BindingFlags.Static | BindingFlags.NonPublic));
                    int wrapped2 = 0;
                    foreach (var m in packIconBaseType.GetMethods(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Instance | BindingFlags.DeclaredOnly))
                    {
                        if (m.IsGenericMethodDefinition || m.IsAbstract || m.IsSpecialName) continue;
                        try
                        {
                            harmony.Patch(m, finalizer: swallowFinalizer2);
                            wrapped2++;
                        }
                        catch { /* best-effort */ }
                    }
                    Log.LogInfo($"[BD2CompatPatch] Wrapped {wrapped2} PackIconBase methods with an exception-swallowing finalizer so one broken pack list entry can't blank out the rest.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find PackIconBase type to wrap with exception-swallowing finalizers.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to wrap PackIconBase methods with finalizers: {e.Message}");
            }

            // Confirmed live now that pack21 actually loads for real (see the EnterPack(21)
            // redirect removal above): FieldObjectBase.SetFieldObjectData() logs "Data not found
            // exception. (FieldActionObjectTable, id:6014)" -- our data tables are missing that row
            // for pack21's map -- and then immediately dereferences the null DTO anyway
            // (fieldObjectDTO.<field>), NullReferenceException, which aborts the whole
            // GameFieldManager field-object-loading coroutine for the WHOLE map, not just this one
            // object, and throws up an error popup. Same "logs the miss, then crashes anyway" shape
            // as everything else this session. Patching just SetFieldObjectData wasn't enough --
            // the very next map object hit the identical bug one method over
            // (FieldObjectBase.<privateCoroutineHelper>, id:2041 in FieldQuestObjectTable), because
            // Initialize() calls THREE sibling methods that each independently read the same
            // lazily-cached FieldObjectDTO property and can each NRE on a missing table row. Same
            // fix as PackIconBase/MenuUI: wrap the whole class so any one broken map object is
            // skipped instead of taking the whole map down, no matter which method hits the miss.
            // Wrapping FieldObjectBase itself wasn't enough on its own: the very next map object hit
            // the identical bug one level down, in a SUBCLASS-only property (FieldRewardObjectController
            // .FieldRewardObjectGroupDTO, FieldRewardObjectTable id:11005) that DeclaredOnly on the
            // base class can't see. There are 27+ FieldObjectBase subclasses (Gate/NPC/Monster/Trap/
            // Trigger/Reward/Quest/Action/Board/WayPoint/etc.), each free to add its own DTO
            // properties with the same "log the miss, dereference it anyway" bug. Wrap every
            // subclass's own declared methods too (DeclaredOnly per type, so we don't re-patch
            // inherited MethodInfos already covered by the base-class pass above).
            //
            // Confirmed live, still not enough: GateSpotData.MapPositionData (a PROPERTY getter
            // chaining ѕeveral other property getters down to the base FieldObjectDTO lookup) threw
            // on a gate with an incomplete FieldGateTable row, and NOTHING caught it -- because
            // property accessors are compiler-flagged IsSpecialName, and the original filter below
            // excluded every IsSpecialName method to stay away from operators/event add-remove.
            // That silently skipped every property getter/setter in this entire class family,
            // including exactly the kind of lazy DTO-lookup property this bug lives in. Confirmed
            // via a live repro: clicking a quest auto-navigate icon walks the player into a gate,
            // GameFieldManager.MoveMap reads GateSpotData.MapPositionData directly (bypassing every
            // method-level check RefreshQuest() etc. already had), throws before the coroutine's
            // first yield, and the whole map transition just dies -- "freezes and never enters",
            // no popup, because nothing was left to report it. Only skip true operators/events now
            // (their names start with op_/add_/remove_), not get_/set_.
            try
            {
                var fieldObjectBaseType = AccessTools.TypeByName("FieldObjectBase");
                if (fieldObjectBaseType != null)
                {
                    var swallowFinalizer3 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SwallowExceptionFinalizer), BindingFlags.Static | BindingFlags.NonPublic));
                    int wrapped3 = 0;
                    int subclassCount = 0;
                    var gateSpotDataType = AccessTools.TypeByName("GateSpotData");
                    foreach (var t in fieldObjectBaseType.Assembly.GetTypes())
                    {
                        if (!fieldObjectBaseType.IsAssignableFrom(t)) continue;
                        if (t != fieldObjectBaseType) subclassCount++;
                        foreach (var m in t.GetMethods(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Instance | BindingFlags.DeclaredOnly))
                        {
                            if (m.IsGenericMethodDefinition || m.IsAbstract) continue;
                            if (m.IsSpecialName && !(m.Name.StartsWith("get_") || m.Name.StartsWith("set_"))) continue;
                            if (m.GetParameters().Any(p => p.ParameterType.IsByRef)) continue;
                            try
                            {
                                harmony.Patch(m, finalizer: swallowFinalizer3);
                                wrapped3++;
                            }
                            catch (Exception patchEx)
                            {
                                // TEMP DIAGNOSTIC: GateSpotData's gate-transition property chain still
                                // NRE'd uncaught after this wrap was extended to cover property
                                // accessors -- log any patch failure on that specific type so we can
                                // see whether Harmony is silently refusing to patch these getters.
                                if (t == gateSpotDataType)
                                {
                                    Log.LogWarning($"[BD2CompatPatch] Failed to patch GateSpotData.{m.Name}: {patchEx.Message}");
                                }
                            }
                        }
                    }
                    Log.LogInfo($"[BD2CompatPatch] Wrapped {wrapped3} methods (including property accessors) across FieldObjectBase and {subclassCount} subclasses with an exception-swallowing finalizer so one map object with no matching design-table row can't crash the whole map's load.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find FieldObjectBase type to wrap with exception-swallowing finalizers.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to wrap FieldObjectBase methods with finalizers: {e.Message}");
            }

            // Confirmed live: opening the Characters screen for a pack21 (Knight of Blood /
            // Chained Soldier 2 collab) character shows the wrong avatar and then hangs on a
            // stuck black transition overlay forever. Root cause is the exact same "log the
            // miss, dereference it anyway" bug, twice over, on this server's collab-character data:
            // CharUI.SetCharIllust does ~6 sequential CostumeDesignTable/CharTable/CostumeDBInfo
            // lookups and NREs on whichever one is missing -- which means the code further down
            // that hides the black loading overlay never runs, so the screen hangs instead of
            // just showing a fallback. Separately, UICostumePotentialIcon.SetSprite (called from
            // CharacterSlotSetting.SetCharacter, itself called from CharManageDeck/CharManageUI as
            // part of the same "set up every slot in sequence" pattern as PackIconBase.SetIcon)
            // NREs on a missing CostumePotentialTable row for the same character. Wrap all five
            // classes in this chain so one collab character with incomplete table data degrades
            // (wrong/blank icon, no potential display) instead of hanging the whole screen.
            try
            {
                string[] charUiChainTypeNames = new[]
                {
                    "CharUI", "CharManageUI", "CharManageDeck", "CharacterSlotSetting", "UICostumePotentialIcon"
                };
                var swallowFinalizer4 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SwallowExceptionFinalizer), BindingFlags.Static | BindingFlags.NonPublic));
                int wrapped4 = 0;
                foreach (var typeName in charUiChainTypeNames)
                {
                    var t = AccessTools.TypeByName(typeName);
                    if (t == null)
                    {
                        Log.LogWarning($"[BD2CompatPatch] Could not find {typeName} to wrap with exception-swallowing finalizers.");
                        continue;
                    }
                    foreach (var m in t.GetMethods(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Instance | BindingFlags.DeclaredOnly))
                    {
                        if (m.IsGenericMethodDefinition || m.IsAbstract || m.IsSpecialName) continue;
                        try
                        {
                            harmony.Patch(m, finalizer: swallowFinalizer4);
                            wrapped4++;
                        }
                        catch { /* best-effort */ }
                    }
                }
                Log.LogInfo($"[BD2CompatPatch] Wrapped {wrapped4} methods across the CharUI/CharManageUI/CharManageDeck/CharacterSlotSetting/UICostumePotentialIcon chain so incomplete collab-character data can't hang the Characters screen.");
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to wrap the CharUI chain with finalizers: {e.Message}");
            }

            // ROOT CAUSE of the "Replace Companion" screen's loading spinner spinning forever
            // (client "hangs" from the user's perspective, though the engine itself keeps running):
            // SpineManager's list-population code loads each companion's Spine illustration one at
            // a time; when a specific character's Spine skeleton data genuinely isn't available on
            // this server (confirmed: "[SpineDataMissing] prefab=illust_char000101_1" logged twice),
            // it calls a PURELY DIAGNOSTIC method whose only job is to build a detailed log string
            // and then throw -- it has zero success-path responsibility (the real success path,
            // SetSpineAnimation(), is a completely separate branch this method is never called
            // from). That uncaught throw aborts whatever loop/continuation was populating the
            // remaining list entries partway through, leaving the "loading more" spinner spinning
            // forever with the list stuck incomplete. Skip this diagnostic method entirely so a
            // missing Spine asset just gets silently skipped instead of freezing the whole list.
            try
            {
                var spineDiagMethod = AccessTools.Method(typeof(SpineManager), "ὠὦὮὦὠὭὠὠὪὣὫ");
                if (spineDiagMethod != null)
                {
                    var skipPrefix8 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SkipMethodPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(spineDiagMethod, prefix: skipPrefix8);
                    Log.LogInfo("[BD2CompatPatch] Patched SpineManager's missing-data diagnostic method to skip instead of throw (was freezing companion/character lists on a missing Spine asset).");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find SpineManager's missing-data diagnostic method to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch SpineManager's missing-data diagnostic method: {e.Message}");
            }

            // The loading screen after entering a pack does `yield return new WaitUntil(() =>
            // !FieldDeckManager.IsRequestPending)` with zero logging either way. That flag is
            // set true right before a direct (non-batched) FieldDeckInfoRequest send and only
            // clears in the matching response callback. Our server does return 200 OK for
            // FieldDeckInfo (confirmed in our own logs), so this looks like a client-side
            // response-routing issue specific to this non-batch send path, not a missing
            // server response. Force the flag to always read false so the wait resolves
            // immediately regardless.
            try
            {
                var fieldDeckType = AccessTools.TypeByName("ὩὤὧὩὭὨὭὥὯὬὣ");
                var pendingGetter = fieldDeckType?.GetProperty("ὣὭὠὭὡὭὬὣὮὯὢ", BindingFlags.Public | BindingFlags.Static)?.GetGetMethod();
                if (pendingGetter != null)
                {
                    var alwaysFalsePostfix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(AlwaysFalsePostfix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(pendingGetter, postfix: alwaysFalsePostfix);
                    Log.LogInfo("[BD2CompatPatch] Patched FieldDeckManager pending-flag to never block the loading wait.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find FieldDeckManager pending-flag getter to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch FieldDeckManager pending-flag: {e.Message}");
            }

            // TEMP DIAGNOSTIC: log every GetPrefabAsset call (used for GameFieldManager,
            // BattlePlayManager, and the map start-spot prefab during LoadingUI's pack-load
            // coroutine) plus its eventual completion status, to find exactly which step of
            // the loading screen is stalling with no error. Remove once resolved.
            try
            {
                var loaderType = AccessTools.TypeByName("ὤὣὪὥὨὡὣὯὧὯὯ");
                var loadMethod = loaderType?.GetMethod("ὤὣὤὬὫὢὢὮὠὮὠ", BindingFlags.Public | BindingFlags.Static);
                if (loadMethod != null)
                {
                    var assetLoadPostfix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(AssetLoadDiagnosticPostfix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(loadMethod, postfix: assetLoadPostfix);
                    Log.LogInfo("[BD2CompatPatch] Patched GetPrefabAsset loader for diagnostic logging.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find GetPrefabAsset loader to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch GetPrefabAsset diagnostic: {e.Message}");
            }

            // RESOLVED: this was diagnostic instrumentation (including a full call-stack dump,
            // removed now) that tracked down "why does the client enter pack21 instead of
            // pack1" -- traced through IntroUI's fresh-account path (lastPlayPackDTO null,
            // playingPackDTO null -> falls through to tutorialPackDTO) to
            // GameDefaultTable.initPackId, a plain captured data value that was simply 21. Not a
            // client bug or a UserInfo/UserPosition mismatch at all -- fixed by editing that one
            // field in httpserver/data/tables/GameDefaultTable.json to 1. Left this id+path log
            // in place (cheap, useful for future pack-transition debugging).
            try
            {
                var enterPackDiag = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(EnterPackDiagnostic), BindingFlags.Static | BindingFlags.NonPublic));
                foreach (var m in typeof(PackManager).GetMethods(BindingFlags.Public | BindingFlags.Instance).Where(mi => mi.Name == "EnterPack"))
                {
                    harmony.Patch(m, prefix: enterPackDiag);
                }
                Log.LogInfo("[BD2CompatPatch] Patched PackManager.EnterPack overloads for diagnostic logging.");
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch EnterPack diagnostic: {e.Message}");
            }

            // The actual "why does a fresh-ish account enter pack21 instead of pack1" answer,
            // found via the diagnostic above: IntroUI's no-history path (lastPlayPackDTO null,
            // playingPackDTO null) falls through to `GameDefaultTable(0).InitPackId` as the
            // account's starting pack. This server's own copy of that table has `initPackId: 21`
            // -- but editing our copy alone doesn't help: like every other design table this
            // session found, GameDefaultTable is loaded by the client from its own local
            // Addressables cache, not served by us over the network, so our copy of the JSON is
            // never actually read by a live client. Patching the real (non-obfuscated,
            // Google.Protobuf-generated) property getter directly is the only fix that reaches
            // the client regardless of what its local cache says.
            //
            // CORRECTION (2026-09-29): pack1 (Knight of Blood, the main starting story) is
            // confirmed the intended real default, and its field data has since been imported
            // for real (FieldGateTable/FieldMonsterTable/FieldWaypointTable, from the old
            // reference server package) -- but pack1's COLD-START entry still crashes on a
            // separate, unresolved bug: the client's own local encrypted SQLite cache file for
            // field data (Data/t/<hash>) fails to open ("unable to open database file" / generic
            // "out of memory"), independent of anything our server sends. This is the same bug
            // documented as unresolved at the end of the previous session. Tried the pack21
            // cold-start workaround (enters cleanly, no local-cache crash) but it has its OWN
            // separate unresolved bug: GameCameraManager.Instance is null for the ENTIRE session
            // (FindObjectOfType never finds one either, unlike an earlier debugging round where
            // it was only transiently null during early loading) -- no camera ever renders the
            // 3D world, producing a black screen even though UI/input keep working fine. Since
            // the auth-middleware UID bug (real root cause of the earlier "no valid field deck"
            // failure) and the missing starting-quest gap are now BOTH fixed server-side, and
            // pack1 now has real field data, reverting to pack1 as the real cold-start default
            // to retest -- the original local-cache crash may behave differently now, and pack1
            // is confirmed the actually-intended default anyway.
            try
            {
                var initPackIdGetter = typeof(Proto.Design.common.GameDefaultTable).GetMethod("get_InitPackId", BindingFlags.Public | BindingFlags.Instance);
                if (initPackIdGetter != null)
                {
                    var forcePack1 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(ForceInitPackId1Postfix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(initPackIdGetter, postfix: forcePack1);
                    Log.LogInfo("[BD2CompatPatch] Patched GameDefaultTable.InitPackId to always return 1 (Knight of Blood) instead of the live game's current 21.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find GameDefaultTable.get_InitPackId to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch GameDefaultTable.InitPackId: {e.Message}");
            }

            // Root cause of "stuck on branded loading spinner after clicking Start Game, zero
            // further log output": IntroUI.SetServerDataComplete does
            // `await UniTask.WaitUntil(() => SoundManager.instance.<TitleCallReady>)` with NO
            // timeout, gating IntroWorkFinished (and therefore OnClickUI ever doing anything)
            // behind it. That flag is only ever set by LoadTitleCallSound's Addressables
            // success/failure callbacks -- but the generic Addressables loader helper appears to
            // throw synchronously (InvalidKeyException, same family of bug as the missing camera
            // timeline/sound-bank assets) instead of invoking either callback when the title-call
            // voice line asset is missing, so the flag is never set and the wait blocks forever
            // with no exception ever reaching CrashReporter/logs. Force the getter to always
            // report ready.
            try
            {
                var titleCallReadyGetter = AccessTools.PropertyGetter(typeof(SoundManager), "ὥὨὢὪὭὤὥὠὩὦὦ");
                if (titleCallReadyGetter != null)
                {
                    var alwaysTruePostfix2 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(AlwaysTruePostfix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(titleCallReadyGetter, postfix: alwaysTruePostfix2);
                    Log.LogInfo("[BD2CompatPatch] Patched SoundManager title-call-ready flag to never block SetServerDataComplete.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find SoundManager title-call-ready getter to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch SoundManager title-call-ready flag: {e.Message}");
            }

            // The real kill switch: CrashReporter.FinalWorking(LogStruct, bool) treats ANY
            // LogType.Exception whose message doesn't contain one specific whitelisted wrapper
            // type's name as fatal and calls Application.Quit(). Under our private-server setup
            // (real internet round-trips through a local TLS-intercepting proxy, no CDN/account
            // infra beyond what we implement) plenty of legitimate, harmless exceptions don't
            // happen to be wrapped in that one type, and each one is an instant, silent quit —
            // exactly what we've been chasing one at a time all session. Force its `ignore`
            // parameter (2nd arg) to true so it never treats anything as fatal.
            try
            {
                var crashReporterMethod = AccessTools.Method(typeof(CrashReporter), "FinalWorking");
                if (crashReporterMethod != null)
                {
                    var prefix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(NeverFatalPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(crashReporterMethod, prefix: prefix);
                    Log.LogInfo("[BD2CompatPatch] Patched CrashReporter.FinalWorking to never Application.Quit() on uncaught exceptions.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find CrashReporter.FinalWorking to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch CrashReporter.FinalWorking: {e.Message}");
            }

            // TEMP DIAGNOSTIC: the game field HUD (GameFieldDefaultUI/CurrencyManageUI) is now
            // instantiated but never actually shown -- screen stays on the plain "BROWN DUST II"
            // logo/black background forever, log goes completely silent right after the last
            // IntGUI1 atlas load. The remaining call chain (LoadingUI -> CoLoadTrainPlayerCharacterInFieldMap
            // -> SetChar -> SetAnimatorCostumeDesign -> the FieldObjectBase Addressables loader)
            // has zero logging of its own. Wrap these coroutines so each MoveNext is tracked --
            // this will show exactly which one never returns. Remove once resolved.
            try
            {
                var wrapPostfix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(WrapCoroutinePostfix), BindingFlags.Static | BindingFlags.NonPublic));
                int wrapped = 0;

                foreach (var m in typeof(CharacterAnimationController).GetMethods(BindingFlags.Public | BindingFlags.Instance | BindingFlags.DeclaredOnly)
                             .Where(mi => mi.Name == "SetAnimatorCostumeDesign" && mi.ReturnType == typeof(IEnumerator)))
                {
                    harmony.Patch(m, postfix: wrapPostfix);
                    wrapped++;
                }

                var playerSetChar = AccessTools.Method(typeof(PlayerController), "SetChar");
                if (playerSetChar != null) { harmony.Patch(playerSetChar, postfix: wrapPostfix); wrapped++; }

                var baseSetChar = AccessTools.Method(typeof(FieldCharacterController), "SetChar");
                if (baseSetChar != null) { harmony.Patch(baseSetChar, postfix: wrapPostfix); wrapped++; }

                var loadTrainChar = AccessTools.Method(typeof(GameFieldManager), "CoLoadTrainPlayerCharacterInFieldMap");
                if (loadTrainChar != null) { harmony.Patch(loadTrainChar, postfix: wrapPostfix); wrapped++; }

                var changeToField = AccessTools.Method(typeof(GameFieldManager), "IChangeToFieldPlayerCharacter");
                if (changeToField != null) { harmony.Patch(changeToField, postfix: wrapPostfix); wrapped++; }

                var moveMapComplete = AccessTools.Method(typeof(GameFieldManager), "MoveMapComplete");
                if (moveMapComplete != null) { harmony.Patch(moveMapComplete, postfix: wrapPostfix); wrapped++; }

                // EnteredPackField() (called after MoveMapComplete, the very next step, and the
                // last one with zero logging of its own) fires a private coroutine that waits
                // `while (!isLoadedUI) yield return null;` for GameFieldDefaultUI + NoticeUI +
                // OverheadManageUI to all register as created -- with NO timeout. Neither NoticeUI
                // nor OverheadManageUI appear anywhere in the logs, so this is the prime suspect
                // for the silent freeze.
                var enteredPackFieldCoroutine = AccessTools.Method(typeof(GameFieldManager), "ὬὭὢὤὥὨὤὦὧὧὤ");
                if (enteredPackFieldCoroutine != null)
                {
                    harmony.Patch(enteredPackFieldCoroutine, postfix: wrapPostfix);
                    wrapped++;
                    Log.LogInfo($"[BD2CompatPatch] Wrapped candidate EnteredPackField coroutine: {enteredPackFieldCoroutine.Name}");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find EnteredPackField's private coroutine by signature.");
                }

                Log.LogInfo($"[BD2CompatPatch] Wrapped {wrapped} character-load coroutines for stall diagnostic.");
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to wrap character-load coroutines: {e.Message}");
            }

            // Confirmed root cause of "clicking the quest auto-navigate icon walks into a gate/house
            // and freezes forever, never enters": GameFieldManager.MoveMap(GateSpotData) starts a
            // coroutine whose very FIRST line (before its first yield -- so it runs synchronously
            // inside StartCoroutine, before MoveMap even returns) reads GateSpotData.MapPositionData,
            // a property chain (MapPositionData -> a "Data" property -> a DTO property -> the base
            // FieldObjectBase.GetFieldObjectDTO lookup) that throws on this gate's incomplete
            // FieldGateTable row. Extending the FieldObjectBase exception-swallowing wrap to cover
            // property accessors (see above) was NOT enough here: MapPositionData is a STRUCT, and a
            // Harmony finalizer swallowing an exception partway through a struct-returning property
            // getter did not reliably yield a safe default the way it does for the class-typed
            // GetFieldObjectDTO calls elsewhere (2000+ successful swallows there, confirmed in the
            // log) -- the exception still escaped uncaught from this specific chain. Rather than
            // debug HarmonyX's struct-return finalizer semantics further, validate the gate's data
            // defensively in a PREFIX before MoveMap's real body ever runs (specifically before it
            // sets SetPlayerMoveState(DontMove), which is what leaves the player stuck -- nothing
            // after that call ever ran to undo it). If the read throws, skip the whole transition:
            // the player just doesn't walk through this one broken gate, instead of freezing.
            try
            {
                var moveMapGateMethod = AccessTools.Method(typeof(GameFieldManager), "MoveMap", new[] { typeof(GateSpotData) });
                if (moveMapGateMethod != null)
                {
                    var safeMoveMapGatePrefix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SafeMoveMapGatePrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(moveMapGateMethod, prefix: safeMoveMapGatePrefix);
                    Log.LogInfo("[BD2CompatPatch] Patched GameFieldManager.MoveMap(GateSpotData) to skip a gate with broken destination data instead of freezing the player.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find GameFieldManager.MoveMap(GateSpotData) to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch GameFieldManager.MoveMap(GateSpotData): {e.Message}");
            }

            // Second confirmed occurrence of the exact same struct-getter hang, found while
            // investigating a report that entering a gate/hut *while quest auto-navigation ("Auto
            // Mode") is active* still freezes even with the MoveMap(GateSpotData) prefix above in
            // place. GameFieldManager's private gate-move coroutine (started by MoveMap above) reads
            // GateSpotData.MapPositionData a SECOND time, later, mid-coroutine, on a *different*
            // GateSpotData instance: when entering the gate also completes a quest, it plays that
            // quest's clear timeline, and afterwards does
            // `mapPositionData = TimelineSignalManager.instance.<warp point>.MapPositionData` to
            // reposition the player at the timeline's own exit spot. Auto Mode walks the player
            // toward quest objectives specifically, so an auto-navigated gate entry is far more
            // likely to also be a quest-clearing one than a manual/incidental walk-in -- explaining
            // why this second occurrence shows up under Auto Mode specifically. The prefix above
            // can't help here (it only guards MoveMap's own synchronous entry, not a statement deep
            // inside an already-running coroutine on an unrelated instance), and this is a raw
            // struct-returning property read, not a method call, so there's no natural
            // "skip the caller" boundary to prefix at that point either.
            //
            // Real fix: patch the property getter itself (GateSpotData.get_MapPositionData) so it
            // can never throw in the first place, for every caller. Reimplemented defensively using
            // ONLY its two sibling properties (int MapId-equivalent, reference-typed Data-equivalent)
            // -- both already proven safe by the broad FieldObjectBase wrap above (ints/references
            // reliably default via the finalizer; it's specifically the struct return that doesn't).
            // On total failure this yields MapId=0, which matches an already-existing convention in
            // this same class (see the `MapId <= 0` check in GameFieldManager's battle-return
            // coroutine) rather than inventing a new sentinel. SafeMoveMapGatePrefix above is updated
            // to treat MapId<=0 as "broken" too, alongside its existing try/catch, so entry point #1
            // keeps its exact previous behavior even though the getter no longer throws for it to catch.
            try
            {
                var gateSpotDataType2 = AccessTools.TypeByName("GateSpotData");
                var mapPositionDataGetter = gateSpotDataType2?.GetMethod("get_MapPositionData", BindingFlags.Public | BindingFlags.Instance);
                if (mapPositionDataGetter != null)
                {
                    var safeGetterPrefix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SafeGateSpotMapPositionDataPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(mapPositionDataGetter, prefix: safeGetterPrefix);
                    Log.LogInfo("[BD2CompatPatch] Patched GateSpotData.MapPositionData getter to fail safe (MapId=0) instead of throwing, for every caller.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find GateSpotData.MapPositionData getter to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch GateSpotData.MapPositionData getter: {e.Message}");
            }

            // A THIRD, separate crash site on the exact same "gate has no FieldGateTable row"
            // condition: GateSpotData.IsPossibleJoinGate(ref string) -- called from
            // PlayerController.OnTriggerEnter, i.e. every time the player's collider touches a
            // gate trigger, before MoveMap is ever reached. Its own body null-checks the backing
            // table row, LOGS a warning, then dereferences it anyway two lines later (identical
            // "logged and dereferenced anyway" shape as every other DataNotFoundException-
            // adjacent bug patched this session) -- confirmed live via Player.log's
            // "CrashReporter Exception Catched" entries naming this exact method/NRE, not caught
            // by any existing wrap (it isn't part of the FieldObjectBase-subclass sweep's method
            // list). Fails OPEN (__result = true, "yes you may attempt this gate") rather than
            // blocking the player from ever using a gate with incomplete data: MoveMap itself is
            // already safe on broken destination data (the two patches directly above), so
            // letting the join attempt through just means "try, and fail safely downstream" —
            // failing closed here instead would silently block the gate forever with no
            // downstream safety net able to help, which is exactly the reported "can't leave the
            // house" symptom.
            try
            {
                var gateSpotDataType3 = AccessTools.TypeByName("GateSpotData");
                var isPossibleJoinGate = gateSpotDataType3?.GetMethods(BindingFlags.Public | BindingFlags.Instance)
                    .FirstOrDefault(m => m.Name == "IsPossibleJoinGate"
                        && m.ReturnType == typeof(bool)
                        && m.GetParameters().Length == 1
                        && m.GetParameters()[0].ParameterType == typeof(string).MakeByRefType());
                if (isPossibleJoinGate != null)
                {
                    var finalizer = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SafeIsPossibleJoinGateFinalizer), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(isPossibleJoinGate, finalizer: finalizer);
                    Log.LogInfo("[BD2CompatPatch] Patched GateSpotData.IsPossibleJoinGate to fail open (allow join attempt) instead of crashing on a gate with no FieldGateTable row.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find GateSpotData.IsPossibleJoinGate(ref string) to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch GateSpotData.IsPossibleJoinGate: {e.Message}");
            }

            // A FOURTH symptom traced to this same broken-transition family: the full-screen
            // "Black" scene-transition overlay (a plain UI Image, confirmed live via the
            // GraphicRaycaster diagnostics above -- click hits stop at "Black" on a canvas with
            // sortOrder=600, blocking every button underneath) and the cinematic depth-of-field
            // blur (GameCameraManager's _blurBackground TranslucentImage, confirmed by decompile
            // to only ever get explicitly set back to non-blurred... actually only ever set TO
            // alpha=1/full-blur at Cinema_End, with the real reset elsewhere not traced) can both
            // get stuck ON indefinitely. Root cause is the same as everything above: these are
            // toggled on at the START of a camera/cutscene transition
            // (GameCameraManager.SetActiveSceneMoveUI(true, ...) /
            // SetActiveSceneMoveUIBlur(true)) and only toggled back off by a LATER step in the
            // same transition sequence (a specific PlayDirector call, a Timeline signal, etc.) --
            // if anything in between throws or gets skipped (exactly what several of our own
            // safety patches above do, deliberately, to avoid a worse crash/freeze), the "turn it
            // back off" call simply never happens, and the player is left with a black overlay
            // eating every click, or a permanently blurred world, or both. Rather than chase the
            // exact broken step for every possible transition type (Main/Cinema/BattleEncount/
            // CutScenes/Airway/EvilCastleFloor all funnel through the same two toggles), patch
            // both toggles with a safety-net watchdog: whenever either is turned ON, schedule a
            // real-time delayed call (a coroutine on this plugin's own MonoBehaviour -- Unity
            // API calls are main-thread-only, unlike the plain-int LoadCameraAsset watchdog
            // above, so a background Timer can't be used here) to force it back OFF a few
            // seconds later regardless of whether the "real" turn-off ever ran. A transition that
            // completes normally just gets turned off twice (harmless no-op the second time); one
            // that gets stuck no longer stays stuck forever.
            try
            {
                var setActiveSceneMoveUI = AccessTools.Method(typeof(GameCameraManager), "SetActiveSceneMoveUI");
                if (setActiveSceneMoveUI != null)
                {
                    var watchdogPostfix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SetActiveSceneMoveUIWatchdogPostfix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(setActiveSceneMoveUI, postfix: watchdogPostfix);
                    Log.LogInfo("[BD2CompatPatch] Patched GameCameraManager.SetActiveSceneMoveUI with a stuck-transition-overlay watchdog.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find GameCameraManager.SetActiveSceneMoveUI to patch.");
                }

                var setActiveSceneMoveUIBlur = AccessTools.Method(typeof(GameCameraManager), "SetActiveSceneMoveUIBlur");
                if (setActiveSceneMoveUIBlur != null)
                {
                    var watchdogPostfix2 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(SetActiveSceneMoveUIBlurWatchdogPostfix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(setActiveSceneMoveUIBlur, postfix: watchdogPostfix2);
                    Log.LogInfo("[BD2CompatPatch] Patched GameCameraManager.SetActiveSceneMoveUIBlur with a stuck-blur watchdog.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find GameCameraManager.SetActiveSceneMoveUIBlur to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch the scene-move-UI watchdogs: {e.Message}");
            }

            // Live retest of the two fixes above: the freeze IS gone (confirmed by the user --
            // "can move and back out but cant enter house"), but the gate-move coroutine itself
            // now throws a real NullReferenceException once it actually starts running (it used to
            // never get this far), caught only by Unity's own generic top-level exception handler
            // (visible as "HandleException Catched"/"CrashReporter Exception Catched" in
            // Player.log, with no useful line info). Need real diagnostics on exactly what's null
            // and where, without more blind guessing -- reuse the EXACT SAME proven technique
            // already used successfully elsewhere in this file for other stuck/crashing coroutines
            // (LoggingCoroutineWrapper via WrapCoroutinePostfix): wrap the METHOD that returns this
            // coroutine's IEnumerator (called via StartCoroutine(...) inside MoveMap(GateSpotData))
            // so its exceptions get logged with a FULL stack trace and captured-state visibility,
            // instead of Unity's terse generic handler.
            //
            // Found name-agnostically by signature, not by a guessed/decompiled name (tonight's
            // whole session says guessed names on this assembly cannot be trusted) -- the private
            // coroutine method takes exactly (GateSpotData, bool) and returns IEnumerator, which is
            // distinctive enough to find by scanning GameFieldManager's own declared methods.
            try
            {
                MethodInfo moveMapCoroutineMethod = null;
                foreach (var m in typeof(GameFieldManager).GetMethods(BindingFlags.NonPublic | BindingFlags.Public | BindingFlags.Instance | BindingFlags.DeclaredOnly))
                {
                    if (m.ReturnType != typeof(IEnumerator)) continue;
                    var ps = m.GetParameters();
                    if (ps.Length == 2 && ps[0].ParameterType == typeof(GateSpotData) && ps[1].ParameterType == typeof(bool))
                    {
                        moveMapCoroutineMethod = m;
                        break;
                    }
                }
                if (moveMapCoroutineMethod != null)
                {
                    var wrapPostfix3 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(WrapCoroutinePostfix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(moveMapCoroutineMethod, postfix: wrapPostfix3);
                    Log.LogInfo($"[BD2CompatPatch] Wrapped GameFieldManager's gate-move coroutine ({moveMapCoroutineMethod.Name}) with full exception diagnostics.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find GameFieldManager's gate-move coroutine (GateSpotData,bool)->IEnumerator to wrap.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to wrap GameFieldManager's gate-move coroutine: {e.Message}");
            }

            // Confirmed root cause of "Confirm button on the field character-setup popup freezes
            // the whole popup forever": FieldCharSettingPopupUI's confirm-click coroutine does
            // `while (!isNext) yield return null;` waiting on a network Send(...) callback to flip
            // a captured closure bool, with NO timeout -- and the surrounding "busy" flag it sets
            // beforehand is only ever cleared inside that same coroutine, so if the callback never
            // fires the ENTIRE popup's OnClickUI (gated on that flag) permanently ignores every
            // future click, including Cancel. Reuse the same stuck-coroutine safety net already
            // proven on the field-load hang: periodic stuck-bool-flag flipping plus a hard abandon.
            try
            {
                var fieldCharSettingConfirmCoroutine = AccessTools.Method(typeof(FieldCharSettingPopupUI), "ὬὫὦὠὯὪὡὥὮὦὠ");
                if (fieldCharSettingConfirmCoroutine != null)
                {
                    var wrapPostfix2 = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(WrapCoroutinePostfix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(fieldCharSettingConfirmCoroutine, postfix: wrapPostfix2);
                    Log.LogInfo("[BD2CompatPatch] Wrapped FieldCharSettingPopupUI confirm coroutine so a missing server response can't freeze the popup forever.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find FieldCharSettingPopupUI confirm coroutine to wrap.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to wrap FieldCharSettingPopupUI confirm coroutine: {e.Message}");
            }

            // THE actual root cause confirmed via the coroutine-wrap diagnostic above: after
            // entering a pack, GameFieldManager's private post-load coroutine does
            // `while (!isLoadedUI) yield return null;` gating on UIManager-style "wait for T to
            // register as created" callbacks for GameFieldDefaultUI, then NoticeUI, then
            // OverheadManageUI -- with NO timeout. GameFieldDefaultUI resolves fine (it's already
            // on screen), but NoticeUI and OverheadManageUI never appear anywhere in the logs, so
            // their registration callback never fires and isLoadedUI never becomes true -- an
            // infinite, silent per-frame loop, exactly matching "stuck on the branded loading
            // background forever after Start Game, zero further log output". Force those two
            // specific UI-ready waits to resolve immediately (with the live instance if one
            // exists, else null) instead of waiting on an Addressables load that never completes.
            // Both a closed-generic patch (type confusion -- Mono shares JIT code across
            // reference-type generic instantiations, confirmed by an earlier diagnostic
            // mislabeling) and an open-generic patch ("IL Compile Error", same failure mode seen
            // elsewhere this session for other generic methods) are unsafe/impossible here. The
            // REAL root cause, found by reading the generic method's own logic: it early-returns
            // and drops the callback entirely (no load attempt, no error, nothing) whenever a
            // non-generic helper -- call it "IsUIOpeningOrLoaded(name)" -- reports the name as
            // already loading/loaded. For NoticeUI/OverheadManageUI that helper is apparently
            // stuck reporting true forever (likely a leftover "in progress" marker from an earlier
            // failed load that never got cleared), so every subsequent request silently no-ops
            // instead of retrying the Addressables load. Patch that ordinary non-generic method
            // instead -- fully safe, no generic sharing involved -- to report false for just these
            // two names, forcing the real load path to actually run.
            try
            {
                var isUiOpeningMethod = typeof(ὩὭὨὪὨὨὮὣὪὣὥ)
                    .GetMethods(BindingFlags.Public | BindingFlags.Static)
                    .FirstOrDefault(m => m.Name == "ὮὬὧὦὣὠὠὤὮὦὧ" && m.ReturnType == typeof(bool)
                        && m.GetParameters().Length == 1 && m.GetParameters()[0].ParameterType == typeof(string));
                if (isUiOpeningMethod != null)
                {
                    var unstickPostfix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(UnstickUiOpeningPostfix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(isUiOpeningMethod, postfix: unstickPostfix);
                    Log.LogInfo("[BD2CompatPatch] Patched UIManager IsUIOpening check to unstick NoticeUI/OverheadManageUI.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find UIManager IsUIOpening check to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to unstick UI-ready waits: {e.Message}");
            }

            // Real crash observed: MercenaryScout's lazy per-id cache calls
            // RawDataManager.GetValueObject<MercenaryScoutTable>(...) (a legitimate "not found ->
            // null" single-row lookup) and feeds the result straight into a mapper that
            // unconditionally dereferences it (e.g. `row.Id`) with no null check ->
            // NullReferenceException whenever the real, current production table has no row for
            // that id. Patch the mapper itself: skip it and return a default T when the row is null.
            try
            {
                var scoutHelperType = AccessTools.TypeByName("ὢὢὬὤὭὧὤὧὠὯὡ");
                var mapperMethodOpen = scoutHelperType?.GetMethod("ὤὢὩὨὫὮὢὨὣὢὬ", BindingFlags.NonPublic | BindingFlags.Static);
                // Only one concrete instantiation exists in the decompiled source
                // (T = ὫὦὠὨὧὭὩὨὨὯὤ) — Harmony patches closed generic methods far more
                // reliably than open generic method definitions (which failed with an IL
                // compile error).
                var elementType = AccessTools.TypeByName("ὫὦὠὨὧὭὩὨὨὯὤ");
                var mapperMethod = (mapperMethodOpen != null && elementType != null)
                    ? mapperMethodOpen.MakeGenericMethod(elementType)
                    : null;
                if (mapperMethod != null)
                {
                    var prefix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(NullRowPrefix), BindingFlags.Static | BindingFlags.NonPublic));
                    harmony.Patch(mapperMethod, prefix: prefix);
                    Log.LogInfo("[BD2CompatPatch] Patched MercenaryScout row-mapper against a null row input.");
                }
                else
                {
                    Log.LogWarning("[BD2CompatPatch] Could not find MercenaryScout row-mapper method to patch.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to patch MercenaryScout row-mapper: {e.Message}");
            }

        }

        // Something drives new accounts into Pack21 (a "season 2, chapter 1"-shaped pack,
        // PackType==1000) instead of the account's real, owned Pack1 — root cause not fully
        // pinned down (likely Pack1's PackType being absent/null in our captured master data
        // rather than the real value), but redirecting the entry point directly here is
        // guaranteed correct regardless: an account that only owns Pack1 should never enter
        // anything else.
        // [HarmonyArgument(0)] binds by position regardless of the real (obfuscated) parameter
        // name — using a genuine `ref` parameter here since object[] __args mutation was NOT
        // propagating to the original call in testing (confirmed via two separate failed
        // attempts, for both a value-type and a reference-type argument).
        // __args[1] is the popup's title parameter; ThrowIfNull<T> always passes exactly
        // "[" + exceptionType.Name + "]" as the title, so this only matches its own popups.
        private static bool SkipDataNotFoundPopupPrefix(object[] __args)
        {
            if (__args.Length > 1 && __args[1] is string title && title == "[DataNotFoundException]")
            {
                Log.LogInfo($"[BD2CompatPatch] Suppressing DataNotFoundException popup: {(__args.Length > 0 ? __args[0] : "")}");
                return false;
            }
            return true;
        }

        private static IEnumerable EmptyEnumerable()
        {
            yield break;
        }

        // See the registration comment above for the full story (this used to unconditionally
        // skip the whole coroutine, which broke every cutscene in the game). Starts a one-shot
        // background timer that force-completes the loaded-asset counter if it's still stuck
        // after a few seconds, then lets the REAL method run (returns true) so the 115+ working
        // timeline assets actually load.
        private static bool StartLoadCameraAssetWatchdogPrefix(object __instance)
        {
            try
            {
                var instanceType = __instance.GetType();
                var counterField = AccessTools.Field(instanceType, "ὨὡὭὫὨὫὮὭὮὪὯ");
                if (counterField == null)
                {
                    Log.LogWarning("[BD2CompatPatch] LoadCameraAsset watchdog: could not find the loaded-asset counter field, letting the original method run unguarded.");
                    return true;
                }
                var instanceRef = __instance;
                System.Threading.Timer timer = null;
                timer = new System.Threading.Timer(timerState =>
                {
                    try
                    {
                        int current = (int)counterField.GetValue(instanceRef);
                        if (current < 120)
                        {
                            Log.LogWarning($"[BD2CompatPatch] LoadCameraAsset: only {current}/120 timeline assets loaded after the watchdog timeout -- forcing the count to 120 so cutscenes/battles aren't blocked forever by whichever ones never resolve.");
                            counterField.SetValue(instanceRef, 120);
                        }
                    }
                    catch (Exception e)
                    {
                        Log.LogWarning($"[BD2CompatPatch] LoadCameraAsset watchdog tick failed: {e.Message}");
                    }
                    finally
                    {
                        // One-shot: dispose immediately after firing so it doesn't linger, and
                        // drop it from the keep-alive set (GC would otherwise collect a Timer
                        // with no other live reference before it ever fires).
                        timer?.Dispose();
                        _loadCameraAssetWatchdogs.TryRemove(timer, out _);
                    }
                }, null, 6000, System.Threading.Timeout.Infinite);
                _loadCameraAssetWatchdogs.TryAdd(timer, 0);
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] LoadCameraAsset watchdog setup failed: {e.Message}");
            }
            return true; // let the real method run for real -- this is the actual fix
        }

        private static bool SkipMethodPrefix(MethodBase __originalMethod)
        {
            Log.LogInfo($"[BD2CompatPatch] Skipping {__originalMethod.DeclaringType?.Name}.{__originalMethod.Name} entirely.");
            return false;
        }

        private static FieldInfo _menuUiHiddenFlagField;

        private static void MenuUiOnClickDiagnosticPrefix(object __instance, GameObject ὩὨὫὩὨὪὩὡὥὫὪ)
        {
            try
            {
                bool? hidden = (bool?)_menuUiHiddenFlagField?.GetValue(__instance);
                Log.LogInfo($"[BD2CompatPatch] MenuUI.OnClickUI: clicked '{(ὩὨὫὩὨὪὩὡὥὫὪ != null ? ὩὨὫὩὨὪὩὡὥὫὪ.name : "null")}', hiddenUiFlag={hidden}");
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] MenuUI.OnClickUI diagnostic failed: {e.Message}");
            }
        }

        private static void GraphicRaycastDiagnosticPostfix(GraphicRaycaster __instance, PointerEventData eventData, List<RaycastResult> resultAppendList)
        {
            try
            {
                bool isClick = UnityEngine.Input.GetMouseButtonDown(0) || UnityEngine.Input.GetMouseButtonUp(0);
                if (isClick)
                {
                    string hitsBefore = resultAppendList.Count == 0
                        ? "(none)"
                        : string.Join(", ", resultAppendList.ConvertAll(r => r.gameObject != null ? r.gameObject.name : "null"));
                    Log.LogInfo($"[BD2CompatPatch] GraphicRaycaster '{__instance.name}' click at {eventData.position}: hits=[{hitsBefore}]");
                }

                // ROOT CAUSE of "literally every click in the game does nothing, on every screen":
                // confirmed via the diagnostic above -- across dozens of clicks on completely
                // different screens (field, popups), the raycast hit list is topped by one of a
                // small family of generic, purely-structural "catcher" objects -- "Collider"
                // (Default/Touch/Touch.prefab's gesture-detection surface), "Blocker" and
                // "Image - InputBlock" (tutorial-system overlays that are apparently already active
                // from prefab instantiation, not just from the explicit SetActive(true) calls we
                // already blocked separately). Since Unity's EventSystem dispatches the click to
                // whichever hit sorts first, any one of these sitting ahead of the real button in
                // sibling/depth order eats the click before the button ever sees it. Strip this
                // family out of every raycast result so whatever real, interactive UI is underneath
                // always gets the click instead.
                // NOTE: "Collider" was REMOVED from this list -- it's very likely the legitimate
                // button wired to IntroUI.OnClickUI() (the "TOUCH TO START" splash screen's actual
                // click target, confirmed by IntroUI having a parameterless OnClickUI() meant for a
                // standard Button.onClick, not the custom GameObject-dispatch OnClickUI(GameObject)
                // used elsewhere). Stripping it broke "tap to start" entirely.
                // NOTE: "TouchScreen" was ALSO REMOVED -- confirmed via GameFieldDefaultUI.cs
                // ("FindChild(gameObject, \"TouchScreen\").SetPad(_touchPad)") that this is the
                // TouchPadScreen movement/drag-to-move input surface for the field, not a stray
                // blocker. It showing up first in a popup canvas's OWN hit list never actually
                // prevented that canvas's real button from receiving the click (EventSystem already
                // picked the popup's Button - Cancel correctly in confirmed testing) -- stripping it
                // was both unnecessary for clicks AND broke field movement entirely. Same mistake
                // shape as "Collider": a generic-sounding object winning lots of raycasts is not by
                // itself evidence it's a bug. Keep stripping only names with no legitimate purpose
                // found anywhere in the decompiled source.
                // "Black" added (2026-09-29): a generic full-screen transition-fade overlay on a
                // GameObject literally named "Canvas" (sortOrder=600, high priority) that sorted
                // first in every raycast on a pack1 warm-switch whose intro cutscene got skipped
                // (unloaded timeline data, same root cause as everything else tonight) -- neither
                // of the two existing stuck-overlay watchdogs (SetActiveSceneMoveUI/-Blur) ever
                // fired for it, meaning it's toggled by some OTHER method entirely, not those two.
                // Symptom matched exactly: the world/message popups still render fine underneath,
                // but every click forever after is eaten by this one element. Same generic-blocker
                // shape as "Blocker"/"Image - InputBlock" below -- stripping it from raycast
                // results only affects click routing, not whatever visually renders it.
                int removed = resultAppendList.RemoveAll(r => r.gameObject != null && (
                    r.gameObject.name == "Blocker" ||
                    r.gameObject.name == "Image - InputBlock" ||
                    r.gameObject.name == "Text - Enter" ||
                    r.gameObject.name == "Black"));
                if (isClick && removed > 0)
                {
                    Log.LogInfo($"[BD2CompatPatch] Removed {removed} generic click-catcher hit(s) from raycast so the real UI underneath gets the click.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] GraphicRaycaster diagnostic/fix failed: {e.Message}");
            }
        }

        private static void EventSystemRaycastAllDiagnosticPostfix(PointerEventData eventData, List<RaycastResult> raycastResults)
        {
            try
            {
                if (!UnityEngine.Input.GetMouseButtonDown(0) && !UnityEngine.Input.GetMouseButtonUp(0))
                {
                    return;
                }
                if (raycastResults.Count == 0)
                {
                    Log.LogInfo("[BD2CompatPatch] EventSystem.RaycastAll: NO results at all for this click.");
                    return;
                }
                string all = string.Join(" | ", raycastResults.ConvertAll(r => $"{(r.gameObject != null ? r.gameObject.name : "null")}(canvas={(r.module as GraphicRaycaster)?.name ?? "?"},depth={r.depth},sortOrder={r.sortingOrder},distance={r.distance})"));
                Log.LogInfo($"[BD2CompatPatch] EventSystem.RaycastAll final list ({raycastResults.Count}): {all}");
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] EventSystem.RaycastAll diagnostic failed: {e.Message}");
            }
        }

        private static bool GracefulPrefabLoadFailurePrefix(object __instance, AsyncOperationHandle<GameObject> op)
        {
            if (op.OperationException == null)
            {
                return true; // let the normal success/status-failure path run untouched
            }
            try
            {
                var instanceType = __instance.GetType();
                string assetAddress = AccessTools.Field(instanceType, "assetAddress")?.GetValue(__instance) as string;
                var callback = AccessTools.Field(instanceType, "callback")?.GetValue(__instance) as Action<GameObject>;
                object selfCleanup = AccessTools.Field(instanceType, "selfCleanup")?.GetValue(__instance);
                Log.LogWarning($"[BD2CompatPatch] Prefab load failed for '{assetAddress}' ({op.OperationException.GetType().Name}: {op.OperationException.Message}) -- calling back with null instead of throwing so the caller's loading state resolves.");
                callback?.Invoke(null);
                selfCleanup?.GetType().GetMethod("SelfDestroy", BindingFlags.Public | BindingFlags.Instance)?.Invoke(selfCleanup, null);
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] GracefulPrefabLoadFailurePrefix itself failed: {e.Message}");
            }
            return false; // skip the original, which would throw
        }

        // Confirmed live: field entry throws a NullReferenceException inside
        // GameCameraManager.PlayDirector, stuck-looping on the plain "BROWN DUST II" logo splash
        // forever instead of reaching the home screen. Root cause: PlayDirector indexes a private
        // per-map-transition array (ὥὯὢὢὧὩὢὩὩὩὤ, one slot per ὦὮὦὧὦὤὠὤὣὤὡ enum value) that only
        // gets populated once that slot's TimelineAsset finishes an async Addressables load
        // (LoadedTimelineAsset); if that specific transition's timeline hasn't loaded yet (or
        // never will), the slot is still null and PlayDirector NREs trying to read
        // .ὣὭὢὬὯὭὩὮὤὩὣ off it. A missing/slow cutscene shouldn't be able to hang the whole
        // client -- skip playback for that one transition instead of throwing.
        private static bool SafeMoveMapGatePrefix(GateSpotData __0)
        {
            try
            {
                // NOT AccessTools.Property(typeof(GateSpotData), "MapPositionData") -- confirmed
                // live that this specific Harmony helper mysteriously fails to find the property
                // ("AccessTools.Property: Could not find property for type GateSpotData and name
                // MapPositionData") once SafeGateSpotMapPositionDataPrefix (below) patches that same
                // getter, even though the property really does exist and a plain
                // GetMethod("get_MapPositionData", ...) by name finds and Invokes it fine (also
                // confirmed live -- that's exactly how the registration below finds it). Direct C#
                // access (`__0.MapPositionData`) doesn't work either: BD2CompatPatch.csproj's own
                // Roslyn compile-time reference to Assembly-CSharp.dll doesn't see this member at
                // all (compile error), even though it's really there and the game's own Mono runtime
                // reflection finds it fine once loaded live -- see the longer note in
                // SafeGateSpotMapPositionDataPrefix. So: plain GetMethod + Invoke, matching the style
                // that's actually proven working, not AccessTools.Property and not direct access.
                var mapPositionDataGetterM = typeof(GateSpotData).GetMethod("get_MapPositionData", BindingFlags.Public | BindingFlags.Instance);
                object mapPos = mapPositionDataGetterM?.Invoke(__0, null);
                var mapIdProperty = mapPos?.GetType().GetProperty("MapId");
                int mapId = mapIdProperty != null ? (int)mapIdProperty.GetValue(mapPos) : 0;
                // The getter itself is now patched safe (see SafeGateSpotMapPositionDataPrefix) and
                // will no longer throw here -- but it fails safe to MapId=0 rather than skipping
                // silently, so this call site still needs to recognize that as "broken" itself,
                // exactly like it used to recognize the exception.
                if (mapId <= 0)
                {
                    Log.LogWarning("[BD2CompatPatch] GameFieldManager.MoveMap(GateSpotData): this gate's destination data is broken (MapId<=0) -- skipping the transition instead of freezing the player.");
                    return false;
                }
            }
            catch (Exception e)
            {
                Exception inner = e.InnerException ?? e;
                Log.LogWarning($"[BD2CompatPatch] GameFieldManager.MoveMap(GateSpotData): this gate's destination data is broken ({inner.GetType().Name}: {inner.Message}) -- skipping the transition instead of freezing the player.");
                return false; // skip the original MoveMap entirely -- never sets DontMove, nothing to undo
            }
            return true;
        }

        // See the registration comment above (near IsPossibleJoinGate) for the full story.
        // Standard HarmonyX finalizer shape: Exception __exception plus a ref to the original
        // return value. Only acts when the original threw; otherwise leaves __result (already
        // set by the real method) untouched.
        private static Exception SafeIsPossibleJoinGateFinalizer(Exception __exception, ref bool __result)
        {
            if (__exception != null)
            {
                Log.LogWarning($"[BD2CompatPatch] GateSpotData.IsPossibleJoinGate threw ({__exception.GetType().Name}: {__exception.Message}) -- failing open (allow join attempt) instead of blocking the gate.");
                __result = true;
            }
            return null;
        }

        // See the registration comment above (near the "FOURTH symptom" note) for the full
        // story. Fires whenever SetActiveSceneMoveUI(true, ...) runs; harmless if the transition
        // finishes normally and turns itself off before the delay elapses.
        private static void SetActiveSceneMoveUIWatchdogPostfix(object __instance, bool __0)
        {
            if (__0 && _instance != null)
            {
                _instance.StartCoroutine(ForceSetActiveSceneMoveUIOffAfterDelay(__instance));
            }
        }

        private static IEnumerator ForceSetActiveSceneMoveUIOffAfterDelay(object instance)
        {
            yield return new WaitForSeconds(10f);
            try
            {
                var method = AccessTools.Method(instance.GetType(), "SetActiveSceneMoveUI");
                method?.Invoke(instance, new object[] { false, null });
                Log.LogWarning("[BD2CompatPatch] SetActiveSceneMoveUI watchdog: forced the scene-transition overlay off after timeout (harmless if it already turned off normally).");
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] SetActiveSceneMoveUI watchdog failed: {e.Message}");
            }
        }

        private static void SetActiveSceneMoveUIBlurWatchdogPostfix(object __instance, bool __0)
        {
            if (__0 && _instance != null)
            {
                _instance.StartCoroutine(ForceSetActiveSceneMoveUIBlurOffAfterDelay(__instance));
            }
        }

        private static IEnumerator ForceSetActiveSceneMoveUIBlurOffAfterDelay(object instance)
        {
            yield return new WaitForSeconds(10f);
            try
            {
                var method = AccessTools.Method(instance.GetType(), "SetActiveSceneMoveUIBlur");
                method?.Invoke(instance, new object[] { false });
                Log.LogWarning("[BD2CompatPatch] SetActiveSceneMoveUIBlur watchdog: forced blur off after timeout (harmless if it already turned off normally).");
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] SetActiveSceneMoveUIBlur watchdog failed: {e.Message}");
            }
        }

        // See the registration comment above (near MoveMap(GateSpotData)) for the full story.
        // Reimplements GateSpotData.MapPositionData using only its two sibling properties that the
        // broad FieldObjectBase finalizer wrap already handles safely (an int-returning MapId
        // equivalent, and a reference-typed Data equivalent) instead of the fragile original
        // struct-returning path. Always skips the original getter (returns false) -- there's no
        // partial-success case worth preserving here, and re-running the fragile original is exactly
        // what we're trying to avoid.
        private static bool SafeGateSpotMapPositionDataPrefix(object __instance, ref object __result)
        {
            try
            {
                // NOTE: BD2CompatPatch.csproj's own compile-time reference to Assembly-CSharp.dll
                // (via `dotnet build`/Roslyn) does NOT see MapID/CurrentMapData/MapPositionData as
                // members of GateSpotData at all -- direct C# access to them is a compile error here,
                // even though the live game's own Mono runtime reflection (what HarmonyX actually
                // uses once this plugin is loaded) finds and uses them correctly, confirmed live:
                // this exact reflection lookup already produced many successful
                // "underlying gate data missing -- returning MapId=0" log lines in real play.
                // Best guess: Roslyn/ilspycmd (both modern .NET 8 tooling) and the old-school
                // Mono/.NET-Framework runtime the game itself runs on parse this specific
                // assembly's string heap differently -- possibly an obfuscator heap-compression
                // trick the two disagree on. Since the RUNTIME reflection is the one that actually
                // matters (that's what's really executing), stick with plain Type.GetProperty
                // string lookups here rather than compile-time member access.
                var type = __instance.GetType();
                var mapIdProp = type.GetProperty("MapID", BindingFlags.Public | BindingFlags.Instance);
                var dataProp = type.GetProperty("CurrentMapData", BindingFlags.Public | BindingFlags.Instance);
                int mapId = mapIdProp != null ? (int)mapIdProp.GetValue(__instance) : 0;
                object data = dataProp?.GetValue(__instance);
                bool usedFallback = false;
                string fallbackPropName = null;
                if (mapId <= 0)
                {
                    // Confirmed live: just returning MapId=0 here and having the caller (see
                    // SafeMoveMapGatePrefix) skip the whole transition prevents the crash/hang at
                    // THIS layer, but still leaves other systems hanging -- specifically, Auto Mode's
                    // quest-navigation coroutine appears to wait for an arrival/completion signal
                    // that a fully-skipped MoveMap never sends, so the player still froze at the same
                    // spot. Real fix: don't abort, redirect to a genuinely valid destination so
                    // MoveMap's real body runs to completion normally (full cleanup/callback/
                    // nav-state-clearing) instead of being skipped.
                    //
                    // A guessed literal name ("BeforeMapID") turned out wrong -- confirmed live, every
                    // single broken gate hit "no usable BeforeMapID fallback" that whole session, for
                    // gates on completely different quest paths, which only makes sense if the NAME
                    // itself doesn't resolve (matches tonight's broader lesson: don't trust a specific
                    // guessed name on this assembly). Name-agnostic fix instead: this gate's own
                    // MapID-equivalent property is confirmed broken (0), but the player is physically
                    // STANDING on this gate right now, so *some* other public Int32 property on this
                    // same object almost certainly holds the map the player is already on (a
                    // "before"/"current" id sibling) -- enumerate all public Int32-returning
                    // properties other than the one just tried, and use the first one that's actually
                    // nonzero, whatever it happens to be named.
                    foreach (var prop in type.GetProperties(BindingFlags.Public | BindingFlags.Instance))
                    {
                        if (prop.PropertyType != typeof(int) || prop.GetIndexParameters().Length != 0) continue;
                        if (mapIdProp != null && prop.MetadataToken == mapIdProp.MetadataToken) continue;
                        int candidate;
                        try { candidate = (int)prop.GetValue(__instance); }
                        catch { continue; }
                        if (candidate > 0)
                        {
                            mapId = candidate;
                            fallbackPropName = prop.Name;
                            usedFallback = true;
                            break;
                        }
                    }
                    if (usedFallback)
                    {
                        // Best-effort: also try to find a Data-typed sibling (the nested
                        // GateSpotData+Data class -- a [Serializable] type, so its own name is
                        // expected to survive same as [SerializeField] field names have all night)
                        // so the player lands at a real recorded position instead of Vector3.zero.
                        // Not critical to fixing the hang (worst case is landing at zero position on
                        // the CORRECT map, a minor visual glitch, not a freeze), so keep this simple
                        // and just fall back to null/zero-position if it doesn't resolve.
                        data = null;
                        foreach (var prop in type.GetProperties(BindingFlags.Public | BindingFlags.Instance))
                        {
                            if (prop.GetIndexParameters().Length != 0 || prop.PropertyType.Name != "Data") continue;
                            try { data = prop.GetValue(__instance); }
                            catch { continue; }
                            if (data != null) break;
                        }
                    }
                }
                Vector3 playerPosition = default;
                Vector3[] colleaguePositions = null;
                if (data != null)
                {
                    var dataType = data.GetType();
                    object rawPlayerPos = dataType.GetField("movePlayerPosition")?.GetValue(data);
                    if (rawPlayerPos is Vector3 v) playerPosition = v;
                    colleaguePositions = dataType.GetField("moveColleaguePosition")?.GetValue(data) as Vector3[];
                }
                __result = new MapPositionData { MapId = mapId, PlayerPosition = playerPosition, ColleaguePositions = colleaguePositions };
                if (usedFallback)
                {
                    Log.LogWarning($"[BD2CompatPatch] GateSpotData.MapPositionData ({__instance}): underlying gate data missing -- redirecting to {fallbackPropName}={mapId} (stay where you are) instead of throwing or aborting.");
                }
                else if (mapId <= 0)
                {
                    Log.LogWarning($"[BD2CompatPatch] GateSpotData.MapPositionData ({__instance}): underlying gate data missing AND no other Int32 property was usable as a fallback -- returning MapId=0, transition will be skipped.");
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] GateSpotData.MapPositionData: safe reconstruction itself failed ({e.GetType().Name}: {e.Message}) -- returning empty MapPositionData.");
                __result = default(MapPositionData);
            }
            return false; // always skip the fragile original getter
        }

        private static bool SafePlayDirectorPrefix(object __instance, object __0, ref WaitForSeconds __result)
        {
            try
            {
                var instanceType = __instance.GetType();
                object director = AccessTools.Field(instanceType, "_playableDirector")?.GetValue(__instance);
                var array = AccessTools.Field(instanceType, "ὥὯὢὢὧὩὢὩὩὩὤ")?.GetValue(__instance) as Array;
                int idx = Convert.ToInt32(__0);
                object slot = (array != null && idx >= 0 && idx < array.Length) ? array.GetValue(idx) : null;
                if (director == null || slot == null)
                {
                    Log.LogWarning($"[BD2CompatPatch] GameCameraManager.PlayDirector: timeline for index {idx} isn't loaded (director null={director == null}, slot null={slot == null}) -- skipping this cutscene instead of hanging on the logo screen.");
                    __result = null;
                    return false; // skip the original, which would throw
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] SafePlayDirectorPrefix check itself failed: {e.Message}");
            }
            return true;
        }

        // See the registration comment above for the full story, including why a prefix on
        // GetTimelineWaitForSeconds itself (the first attempt) didn't work: HarmonyX's own
        // dispatch trampoline null-checks "this" before a prefix ever runs, for a call whose
        // receiver is genuinely null. Patching the Instance getter instead avoids that problem
        // entirely -- nothing downstream ever sees a null Instance in the first place.
        private static void FixNullCameraManagerInstancePostfix(ref GameCameraManager __result)
        {
            if (__result == null)
            {
                // includeInactive: true -- a disabled-but-present GameCameraManager (e.g. one left
                // inactive after a failed async load step) would be invisible to the default
                // FindObjectOfType overload, which skips inactive GameObjects entirely.
                __result = UnityEngine.Object.FindObjectOfType<GameCameraManager>(true);
                Log.LogWarning($"[BD2CompatPatch] GameCameraManager.Instance was null -- falling back to FindObjectOfType ({(__result != null ? "found one" : "found NONE")}) instead of letting a caller crash on it.");
            }
        }

        // Second attempt at GetTimelineWaitForSeconds: the Instance-getter fix confirmed Instance
        // is genuinely non-null by the time the gate-move coroutine reaches this call, so the NRE
        // must be inside this method's own body instead. Same exact shape as the already-proven
        // SafePlayDirectorPrefix above (reusing its own field-name discovery, since both methods
        // very plausibly share the same backing timeline-director/per-index-slot-array fields): a
        // per-transition-type array slot that hasn't loaded for the specific index this call uses.
        private static bool SafeGetTimelineWaitForSecondsPrefix2(object __instance, object __0, ref WaitForSeconds __result)
        {
            try
            {
                var instanceType = __instance.GetType();
                object director = AccessTools.Field(instanceType, "_playableDirector")?.GetValue(__instance);
                var array = AccessTools.Field(instanceType, "ὥὯὢὢὧὩὢὩὩὩὤ")?.GetValue(__instance) as Array;
                int idx = Convert.ToInt32(__0);
                object slot = (array != null && idx >= 0 && idx < array.Length) ? array.GetValue(idx) : null;
                if (director == null || slot == null)
                {
                    Log.LogWarning($"[BD2CompatPatch] GameCameraManager.GetTimelineWaitForSeconds: timeline for index {idx} isn't loaded (director null={director == null}, slot null={slot == null}) -- skipping instead of crashing the gate-move coroutine.");
                    __result = null;
                    return false;
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] SafeGetTimelineWaitForSecondsPrefix2 check itself failed: {e.Message}");
            }
            return true;
        }

        // Confirmed live: a single dead file on Neowiz's real CDN (pack12006's
        // goblindoomsday.bundle -- verified with a direct curl outside the game: consistent
        // HTTP 504 Gateway Timeout from Akamai's own edge, an external infrastructure issue,
        // not ours or our bundle_version) aborts IntroUI's whole "Download All Packs" preload
        // and force-restarts the client with a CLIENT_LOGIC_ERROR/BUNDLE_COMMON popup. That
        // preload is just a prefetch for smoother later loading -- content still gets fetched
        // on demand during actual gameplay, and anything still missing then fails gracefully
        // via the prefab-load patches above instead of crashing. So one bad prefetch shouldn't
        // block the whole client. __0 is IntroUI's private nested enum (BundleDownloadFailed
        // etc.) boxed as object -- compared by name since the type itself is inaccessible here.
        private static bool SkipBundleDownloadFailedPrefix(object __instance, object __0, Exception __1)
        {
            try
            {
                if (__0 != null && __0.ToString() == "BundleDownloadFailed")
                {
                    Log.LogWarning($"[BD2CompatPatch] Ignoring BundleDownloadFailed (single dead CDN bundle, not fatal) instead of restarting the client: {__1?.Message}");
                    var onDownloadComplete = __instance.GetType().GetMethod("OnDownloadComplete", BindingFlags.Public | BindingFlags.Instance);
                    onDownloadComplete?.Invoke(__instance, null);
                    return false; // skip the original, which would show the fatal error popup
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] SkipBundleDownloadFailedPrefix itself failed: {e.Message}");
            }
            return true;
        }

        // Confirmed live via this same diagnostic: tutorial id 10057 (the home-screen first-time
        // message) replayed in full on every single return to the home screen -- IsClearTutorial
        // stayed false and, per the httpserver log, the client never once sent a TutorialClearRequest
        // for it. TutorialManager.Play only sends that clear request itself for FocusTutorialTable
        // rows whose Type is ONCE; 10057's own trigger condition (whatever the real client normally
        // uses to only show it once) isn't correctly resolving true on this server's account data,
        // so it re-qualifies to play every time instead of self-clearing. Rather than chase that
        // condition through the client's own compiled design tables, force the exact same
        // already-proven clear round-trip (confirmed working for ids 10015/10030) for ANY tutorial
        // that plays through once, regardless of its own Type -- so nothing can loop, ever, even if
        // more of these turn up later.
        private static readonly HashSet<int> _tutorialClearRequested = new HashSet<int>();

        private static bool TutorialPlayDiagnosticPrefix(object __instance, int __0)
        {
            try
            {
                var instanceType = __instance.GetType();
                var isClearMethod = instanceType.GetMethod("IsClearTutorial", BindingFlags.Public | BindingFlags.Instance);
                bool alreadyCleared = isClearMethod != null && (bool)isClearMethod.Invoke(__instance, new object[] { __0 });
                Log.LogInfo($"[BD2CompatPatch] TutorialManager.Play({__0}) called, IsClearTutorial={alreadyCleared}.");
                if (!alreadyCleared && _tutorialClearRequested.Add(__0))
                {
                    var netManagerType = AccessTools.TypeByName("ὣὡὧὡὦὣὣὬὨὪὫ");
                    var sendClearMethod = netManagerType?.GetMethod("ὫὪὮὥὣὬὥὤὢὦὪ", BindingFlags.Public | BindingFlags.Static);
                    if (sendClearMethod != null)
                    {
                        sendClearMethod.Invoke(null, new object[] { __0, null });
                        Log.LogInfo($"[BD2CompatPatch] Force-sent TutorialClearRequest({__0}) so this tutorial can't replay on the next home-screen visit.");
                    }
                    else
                    {
                        Log.LogWarning("[BD2CompatPatch] Could not find the tutorial-clear network method to force-clear a repeating tutorial.");
                    }
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] TutorialPlayDiagnosticPrefix itself failed: {e.Message}");
            }
            return true; // let this one showing play out normally; only future replays are prevented
        }

        private static bool SkipFocusCoroutinePrefix(ref IEnumerator __result)
        {
            Log.LogInfo("[BD2CompatPatch] Skipping TutorialFocusController.Focus entirely.");
            __result = EmptyEnumerable().GetEnumerator();
            return false;
        }

        private static bool BlockTutorialInputBlockerPrefix(GameObject __instance, bool value)
        {
            if (value && __instance != null && __instance.name == "Image - InputBlock")
            {
                Log.LogInfo("[BD2CompatPatch] Blocked activation of tutorial input blocker 'Image - InputBlock'.");
                return false;
            }
            return true;
        }

        private static FieldInfo _fieldCharSettingBusyFlagField;
        private static MethodInfo _isUiOpeningGetter;

        private static void FieldCharSettingOnClickDiagnosticPrefix(object __instance, GameObject ὩὨὫὩὨὪὩὡὥὫὪ)
        {
            try
            {
                bool? busy = (bool?)_fieldCharSettingBusyFlagField?.GetValue(__instance);
                bool? isUiOpening = (bool?)_isUiOpeningGetter?.Invoke(null, null);
                Log.LogInfo($"[BD2CompatPatch] FieldCharSettingPopupUI.OnClickUI: clicked '{(ὩὨὫὩὨὪὩὡὥὫὪ != null ? ὩὨὫὩὨὪὩὡὥὫὪ.name : "null")}', busyFlag={busy}, IsUIOpening={isUiOpening}");
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] FieldCharSettingPopupUI.OnClickUI diagnostic failed: {e.Message}");
            }
        }

        private static void AtlasGetSpriteDiagnosticPostfix(object __instance, string name, UnityEngine.Sprite __result)
        {
            if (__result == null)
            {
                string atlasName = "?";
                try
                {
                    var atlasField = __instance.GetType().GetProperty("Atlas", BindingFlags.Public | BindingFlags.Instance);
                    var atlas = atlasField?.GetValue(__instance) as UnityEngine.U2D.SpriteAtlas;
                    atlasName = atlas != null ? atlas.name : "(atlas is null)";
                }
                catch { /* best-effort */ }
                Log.LogWarning($"[BD2CompatPatch] AtlasContainer.GetSprite(\"{name}\") returned null from atlas '{atlasName}' -- this sprite will render blank.");
            }
        }

        // Generic Harmony finalizer: lets the patched method run completely normally, but if it
        // throws, swallows the exception right there instead of letting it unwind into the caller.
        // For a void method this makes the call site look like it returned normally, so a caller
        // that runs a long sequence of independent setup calls (MenuUI.Init(), etc.) keeps going
        // to its next statement instead of aborting everything after the one that failed.
        private static Exception SwallowExceptionFinalizer(Exception __exception, MethodBase __originalMethod)
        {
            if (__exception != null)
            {
                Log.LogWarning($"[BD2CompatPatch] {__originalMethod.DeclaringType?.Name}.{__originalMethod.Name} threw ({__exception.GetType().Name}: {__exception.Message}) -- swallowed so the caller keeps running.");
            }
            return null;
        }

        // Exact call order taken from the decompiled MenuUI.UpdateIssue() body -- replicated here
        // so every badge/issue sub-updater still runs (in the same order), just each wrapped in
        // its own try/catch instead of sharing one uncaught exception that kills every updater
        // after the first one that hits null (zero-progression/private-server data for a feature
        // like guild, dating, mini-game-hub, story timeline, etc. is exactly the kind of thing
        // that trips these -- same category as the talent-data skips above, just many of them at
        // once instead of one at a time).
        private static readonly string[] MenuUiIssueSubMethods = new[]
        {
            "UpdateInteractionIssue", "UpdateNewsIssue", "UpdateEventIssue", "UpdatePictorialIssue",
            "UpdateMissionPassIssue", "UpdateMailIssue", "UpdateCashShopIssue", "UpdatePackageShopIssue",
            "UpdateMenuMonthlySubIcon", "UpdateGachaIssue", "UpdateAchievementIssue", "UpdatePassIssue",
            "UpdateLobbySettingIssue", "UpdateEtcIssue", "UpdateEventHubIssue", "UpdateMiniEventHubIssue",
            "UpdateMyRoomIssue", "UpdateGuildIssue", "UpdateFriendIssue", "UpdateDatingIssue",
            "UpdateMiniGameHubIssue", "UpdateHuntDispatchButtonIssue", "UpdateStoryTimelineIssue",
            "UpdateFriendshipIssue", "ὥὥὬὡὥὪὢὯὦὥὭ", "ὧὨὪὡὫὤὥὭὩὠὬ", "ὡὥὮὦὧὩὠὠὫὭὮ",
            "UpdateCharacterVotingIssue",
        };

        private static bool SafeUpdateIssuePrefix(object __instance)
        {
            var type = __instance.GetType();
            foreach (var name in MenuUiIssueSubMethods)
            {
                try
                {
                    var m = type.GetMethod(name, BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Instance, null, Type.EmptyTypes, null);
                    if (m == null)
                    {
                        Log.LogWarning($"[BD2CompatPatch] MenuUI.UpdateIssue: sub-method {name} not found (client version mismatch?), skipping.");
                        continue;
                    }
                    m.Invoke(__instance, null);
                }
                catch (Exception e)
                {
                    var inner = e.InnerException ?? e;
                    Log.LogWarning($"[BD2CompatPatch] MenuUI.UpdateIssue: {name} threw ({inner.GetType().Name}: {inner.Message}) -- skipped so the rest of the menu bar still initializes.");
                }
            }
            try
            {
                var packIconField = AccessTools.Field(type, "_packIconBase");
                var packIconBase = packIconField?.GetValue(__instance);
                var setTotalPackIssue = packIconBase?.GetType().GetMethod("SetTotalPackIssue", BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Instance, null, Type.EmptyTypes, null);
                setTotalPackIssue?.Invoke(packIconBase, null);
            }
            catch (Exception e)
            {
                var inner = e.InnerException ?? e;
                Log.LogWarning($"[BD2CompatPatch] MenuUI.UpdateIssue: _packIconBase.SetTotalPackIssue threw ({inner.GetType().Name}: {inner.Message}), skipped.");
            }
            return false; // we just ran a safe equivalent of the original body above
        }

        // Wraps a coroutine's returned IEnumerator so every MoveNext is tracked -- logs entry,
        // periodic "still running" heartbeats (so a genuine hang is visible within seconds
        // instead of guessed at from silence), any exception thrown out of MoveNext (which
        // would otherwise vanish into Unity's coroutine runner with no trace back to us), and
        // exit. Purely a diagnostic: behavior is unchanged, only observed.
        private static IEnumerator LoggingCoroutineWrapper(string label, IEnumerator inner)
        {
            Log.LogInfo($"[BD2CompatPatch] ENTER {label}");
            float start = Time.realtimeSinceStartup;
            float lastFlip = -999f;
            int frame = 0;
            while (true)
            {
                bool moved;
                try
                {
                    moved = inner.MoveNext();
                }
                catch (Exception e)
                {
                    Log.LogWarning($"[BD2CompatPatch] EXCEPTION out of {label} after {frame} frames / {Time.realtimeSinceStartup - start:F1}s: {e}");
                    LogCrashLocationAndCapturedState(e, inner);
                    yield break;
                }
                if (!moved) break;
                frame++;
                if (frame % 30 == 0)
                {
                    var cur = inner.Current;
                    Log.LogInfo($"[BD2CompatPatch] {label} still running: frame {frame}, {Time.realtimeSinceStartup - start:F1}s elapsed, yielding={cur?.GetType().FullName ?? "null"}");
                }
                // Safety valve: several real bugs found this session are exactly this shape --
                // `while (!someFlagThatNeverBecomesTrue) yield return null;`, no timeout, gated on
                // a UI/asset registration that silently never fires. Rather than abandoning the
                // whole coroutine (which also skips whatever legitimate game code was supposed to
                // run right after that wait), reach into the compiler-generated closure(s) holding
                // its captured locals and flip every currently-false bool to true. The loop's own
                // `yield return null` re-checks its flag on the very next MoveNext and falls
                // through on its own -- everything after that point then runs as normal, correct
                // game code. Retried every ~3s in case a coroutine has more than one such gate in
                // sequence (confirmed to happen at least once this session).
                if (Time.realtimeSinceStartup - start > 3f && Time.realtimeSinceStartup - lastFlip > 3f)
                {
                    int flipped = TryForceStuckBoolFlags(inner);
                    if (flipped > 0)
                    {
                        Log.LogInfo($"[BD2CompatPatch] {label}: flipped {flipped} stuck bool flag(s) after {frame} frames -- letting it continue.");
                    }
                    lastFlip = Time.realtimeSinceStartup;
                }
                if (Time.realtimeSinceStartup - start > 20f)
                {
                    Log.LogWarning($"[BD2CompatPatch] {label} exceeded 20s ({frame} frames) with no sign of completing -- abandoning it so the game doesn't hang forever.");
                    // Whatever this coroutine hasn't finished, at minimum don't leave the player
                    // staring at a stuck loading screen -- force it closed as a last resort so the
                    // field is at least visible/interactable.
                    if (label.Contains("GameFieldManager.ὬὭὢὤὥὨὤὦὧὧὤ"))
                    {
                        TryRecoverBlackScreenAfterFieldLoad(closeLoadingUi: true);
                    }
                    yield break;
                }
                yield return inner.Current;
            }
            Log.LogInfo($"[BD2CompatPatch] EXIT {label} after {frame} frames / {Time.realtimeSinceStartup - start:F1}s");
            // CORRECTION (2026-09-29): the coroutine can also complete NORMALLY (well under the
            // 20s abandon threshold -- ~4s observed) while the screen is STILL solid black.
            // Originally gated this on GameCameraManager being null, but a retest showed
            // GameCameraManager is found fine (non-null) on normal completion while the screen is
            // STILL black -- meaning the real cause here isn't a missing camera manager at all,
            // it's the SEPARATE mechanism the recovery body's own comment already documents: the
            // out-of-field UI cameras (OutGameUICamera etc.) staying enabled with SolidColor clear
            // flags and a higher render depth, painting over an otherwise-healthy field camera
            // every frame. That cleanup (AppManager.SetActiveOutGameUICamera(false)) is normally
            // reached deep inside this same coroutine chain -- since we can't tell from here
            // whether THIS particular normal completion actually reached it, just always run the
            // recovery unconditionally on this coroutine's normal exit. Every step inside is
            // idempotent (SetActive/RestoreFieldOfView/etc. on an already-correct state is a
            // harmless no-op), so this is safe even when nothing was actually wrong.
            if (label.Contains("GameFieldManager.ὬὭὢὤὥὨὤὦὧὧὤ"))
            {
                Log.LogInfo($"[BD2CompatPatch] {label} completed normally -- running black-screen recovery unconditionally (idempotent) to cover the OutGameUICamera-overlay case.");
                TryRecoverBlackScreenAfterFieldLoad(closeLoadingUi: false);
            }
        }

        // Extracted from the 20s-abandon watchdog above (which no longer duplicates this body) so
        // the same recovery can also run after a coroutine that completed NORMALLY but still left
        // the screen black (see the correction note at its normal-exit call site).
        private static void TryRecoverBlackScreenAfterFieldLoad(bool closeLoadingUi)
        {
            if (closeLoadingUi)
            {
                try
                {
                    var loadingUi = UnityEngine.Object.FindObjectOfType<LoadingUI>();
                    if (loadingUi != null)
                    {
                        Log.LogInfo("[BD2CompatPatch] Force-closing LoadingUI after abandoning the stuck coroutine.");
                        loadingUi.CloseUIImmediately();
                    }
                }
                catch (Exception e)
                {
                    Log.LogWarning($"[BD2CompatPatch] Failed to force-close LoadingUI: {e.Message}");
                }
            }
            // The abandoned/incomplete coroutine never reached its own camera/character/HUD
            // activation calls (RestoreFieldOfView, SetActiveTrainPlayerCharacters, the
            // GameFieldDefaultUI/CurrencyManageUI reveal), which is why the screen stays solid
            // black even after the loading UI itself closes. Do the essential ones directly --
            // calling these ordinary public methods normally (not patching them) is unaffected by
            // the generic-sharing issue that ruled out patching UIManager's own lookup helper.
            // Uses the includeInactive overload: a disabled-but-present GameCameraManager would
            // otherwise be invisible to the default FindObjectOfType overload used everywhere else.
            try
            {
                var gcm = UnityEngine.Object.FindObjectOfType<GameCameraManager>(true);
                if (gcm != null)
                {
                    gcm.SetActive(true);
                    gcm.RestoreFieldOfView();
                }
                // The out-of-field UI cameras (OutGameUICamera etc.) stay enabled with
                // SolidColor clear flags and a higher render depth than the field
                // camera, so every frame they paint over it -- this is the actual cause
                // of the black screen even with the field camera correctly positioned
                // and enabled. Normally cleared up by AppManager.SetActiveOutGameUICamera(false)
                // deep inside the same abandoned coroutine chain; call it directly.
                Singleton<AppManager>.ὪὫὢὨὯὭὦὪὦὨὣ?.SetActiveOutGameUICamera(false);
                var gfm = UnityEngine.Object.FindObjectOfType<GameFieldManager>();
                gfm?.SetActiveTrainPlayerCharacters(true);
                var findUiOpen = typeof(ὩὭὨὪὨὨὮὣὪὣὥ)
                    .GetMethods(BindingFlags.Public | BindingFlags.Static)
                    .FirstOrDefault(m => m.Name == "ὤὨὪὥὩὦὫὨὩὫὥ" && m.IsGenericMethodDefinition && m.GetParameters().Length == 0);
                foreach (var uiType in new[] { typeof(GameFieldDefaultUI), typeof(CurrencyManageUI) })
                {
                    var ui = findUiOpen?.MakeGenericMethod(uiType).Invoke(null, null) as UIBase;
                    ui?.SetActive(true);
                }
                // SetActive(true) alone doesn't run GameFieldDefaultUI's own Init()
                // (currency display, quest counts, reputation icon, event banner,
                // and -- critically -- whatever wires up its ProgressInfo sub-object).
                // Normally called from within the same abandoned coroutine chain.
                // Without it, ProgressInfo.OnClick throws a NullReferenceException on
                // literally every click/keypress (it's one branch of OnClickUI's long
                // `||` dispatch chain), which aborts that whole chain before later
                // branches (menu/home navigation) ever get a chance to run -- this is
                // the actual cause of "can't go home" and several of the blank icons.
                var gameFieldDefaultUi = UnityEngine.Object.FindObjectOfType<GameFieldDefaultUI>();
                if (gameFieldDefaultUi != null)
                {
                    // These fields (chat, channel select, avatar customization, etc.)
                    // are lobby/social features our server doesn't implement -- they're
                    // genuinely absent for this pack, and the client's own Init()
                    // doesn't null-check before touching them, so it throws on the
                    // first one and aborts everything after (currency, quest counts,
                    // event banner, reputation icon, ProgressInfo wiring). Give each
                    // null Component-typed field a harmless stub (an inactive GameObject
                    // with the component attached) so Init() can run past them --
                    // these features stay non-functional either way since we don't
                    // back them server-side, but the REST of Init() gets to complete.
                    try
                    {
                        var stubRoot = new GameObject("BD2CompatPatch_Stubs");
                        UnityEngine.Object.DontDestroyOnLoad(stubRoot);
                        stubRoot.SetActive(false);
                        // GameFieldDefaultUI's own null fields are one layer -- but it
                        // also has non-null nested helper objects (ProgressInfo and
                        // similar) whose OWN internal fields (quest slot prefabs,
                        // ScrollRect, etc.) are separately null and not reachable by
                        // only scanning the top-level type. Recurse into fields whose
                        // type is declared nested inside GameFieldDefaultUI (keeps this
                        // from wandering into unrelated singletons/managers).
                        int stubbed = StubNullFieldsRecursive(gameFieldDefaultUi, typeof(GameFieldDefaultUI), stubRoot, new HashSet<object>(), 0);
                        Log.LogInfo($"[BD2CompatPatch] Stubbed {stubbed} null GameFieldDefaultUI field(s) (including nested) so Init() can complete.");
                    }
                    catch (Exception diagEx)
                    {
                        Log.LogWarning($"[BD2CompatPatch] Field stubbing failed: {diagEx.Message}");
                    }
                    try
                    {
                        gameFieldDefaultUi.Init();
                        Log.LogInfo("[BD2CompatPatch] GameFieldDefaultUI.Init() succeeded.");
                    }
                    catch (Exception initEx)
                    {
                        Log.LogWarning($"[BD2CompatPatch] GameFieldDefaultUI.Init() threw: {initEx}");
                    }
                }
                Log.LogInfo("[BD2CompatPatch] Directly activated camera/character/HUD (black-screen recovery).");
                foreach (var cam in UnityEngine.Camera.allCameras)
                {
                    Log.LogInfo($"[BD2CompatPatch] Camera '{cam.name}': enabled={cam.enabled} gameObjectActive={cam.gameObject.activeInHierarchy} depth={cam.depth} clearFlags={cam.clearFlags} cullingMask={cam.cullingMask} pos={cam.transform.position} targetTexture={cam.targetTexture}");
                }
                Log.LogInfo($"[BD2CompatPatch] Camera.main={(UnityEngine.Camera.main != null ? UnityEngine.Camera.main.name : "null")}, total allCameras={UnityEngine.Camera.allCamerasCount}");
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] Failed to directly activate camera/character/HUD: {e.Message}");
            }
        }

        // Ground-truth crash-site diagnostics, done entirely with the LIVE Mono runtime's own
        // reflection -- no ilspycmd, no external decompiler, no MetadataLoadContext. Tonight's
        // whole session found that every static tool disagrees with this assembly's actual
        // runtime member names at least once; the one thing that's stayed correct all night is
        // whatever the live game itself reports. Mono's own stack trace formatter already prints
        // the exact IL byte offset the exception came from (e.g. "[0x0007e]") -- parse that back
        // out, read the real IL bytes of the method that actually threw (via
        // MethodBase.GetMethodBody(), a normal reflection API, not any external tool), decode the
        // single instruction at that offset, and resolve its token through the SAME live module
        // that's already running. This answers "what field/method access was this" with total
        // certainty, sidestepping the entire naming-instability problem instead of fighting it.
        // Built once from System.Reflection.Emit.OpCodes' own static fields -- the real, complete
        // opcode table (keyed by Value: single-byte opcodes 0x00-0xFD as-is, two-byte 0xFE-prefixed
        // ones as 0xFE00|secondByte) -- rather than a hand-typed partial list prone to the same
        // "guessed and wrong" failure mode as every named-member guess earlier tonight.
        private static readonly Dictionary<int, OpCode> _ilOpCodes = typeof(OpCodes)
            .GetFields(BindingFlags.Public | BindingFlags.Static)
            .Where(f => f.FieldType == typeof(OpCode))
            .Select(f => (OpCode)f.GetValue(null))
            .ToDictionary(op => (int)(ushort)op.Value, op => op);

        private static void LogCrashLocationAndCapturedState(Exception e, object stateMachineInstance)
        {
            try
            {
                var throwingMethod = e.TargetSite;
                var match = System.Text.RegularExpressions.Regex.Match(e.StackTrace ?? "", @"\[0x([0-9A-Fa-f]+)\]");
                if (throwingMethod != null && match.Success)
                {
                    int reportedOffset = Convert.ToInt32(match.Groups[1].Value, 16);
                    var body = throwingMethod.GetMethodBody();
                    byte[] il = body?.GetILAsByteArray();
                    if (il != null)
                    {
                        // Proper instruction-by-instruction walk from offset 0 using the real
                        // opcode table (via System.Reflection.Emit.OpCodes, reflected once) rather
                        // than a hand-picked partial list -- a single blind byte-offset lookup at
                        // the reported offset previously landed mid-instruction on a harmless
                        // ldc.i4.2, meaning Mono's reported offset doesn't necessarily line up with
                        // where a naive index expects the *start* of the faulting instruction. Log
                        // every instruction in a window around the reported offset instead, so the
                        // real fault is visible by context even if the exact offset semantics are
                        // slightly different from a raw byte index.
                        Log.LogWarning($"[BD2CompatPatch] Crash site: {throwingMethod.DeclaringType?.FullName}.{throwingMethod.Name}, Mono reported IL offset 0x{reportedOffset:X} -- disassembly window:");
                        int pos = 0;
                        while (pos < il.Length)
                        {
                            int instrStart = pos;
                            int opByte = il[pos];
                            OpCode opcode;
                            if (opByte == 0xFE && pos + 1 < il.Length)
                            {
                                int twoByteVal = 0xFE00 | il[pos + 1];
                                if (!_ilOpCodes.TryGetValue(twoByteVal, out opcode)) break;
                                pos += 2;
                            }
                            else
                            {
                                if (!_ilOpCodes.TryGetValue(opByte, out opcode)) break;
                                pos += 1;
                            }
                            int operandLen = opcode.OperandType switch
                            {
                                OperandType.InlineNone => 0,
                                OperandType.ShortInlineBrTarget or OperandType.ShortInlineI or OperandType.ShortInlineVar => 1,
                                OperandType.InlineVar => 2,
                                OperandType.InlineSwitch => -1, // variable length, handled below
                                OperandType.InlineI8 or OperandType.InlineR => 8,
                                _ => 4, // InlineBrTarget/InlineField/InlineI/InlineMethod/InlineSig/InlineString/InlineTok/InlineType/ShortInlineR
                            };
                            if (operandLen == -1)
                            {
                                if (pos + 4 > il.Length) break;
                                int caseCount = BitConverter.ToInt32(il, pos);
                                operandLen = 4 + caseCount * 4;
                            }
                            if (pos + operandLen > il.Length) break;
                            bool inWindow = instrStart >= reportedOffset - 24 && instrStart <= reportedOffset + 16;
                            if (inWindow)
                            {
                                string marker = instrStart == reportedOffset ? " <== reported offset" : "";
                                string operandStr = "";
                                if (operandLen == 4 && (opcode.OperandType == OperandType.InlineField || opcode.OperandType == OperandType.InlineMethod
                                    || opcode.OperandType == OperandType.InlineTok || opcode.OperandType == OperandType.InlineType))
                                {
                                    int token = BitConverter.ToInt32(il, pos);
                                    try
                                    {
                                        var member = throwingMethod.Module.ResolveMember(token, throwingMethod.DeclaringType?.GetGenericArguments(), null);
                                        operandStr = $" -> {member.MemberType} {member.DeclaringType?.Name}.{member.Name}";
                                    }
                                    catch (Exception resolveEx) { operandStr = $" token=0x{token:X8} (resolve failed: {resolveEx.Message})"; }
                                }
                                Log.LogWarning($"[BD2CompatPatch]   +0x{instrStart:X4} {opcode.Name}{operandStr}{marker}");
                            }
                            pos += operandLen;
                        }
                    }
                }

                // Also dump the coroutine's own captured state (its fields hold whatever locals/
                // parameters the compiler promoted into the state machine -- the GateSpotData
                // parameter, the computed MapPositionData local, etc.) so a null field is visible
                // directly, independent of the IL decode above.
                if (stateMachineInstance != null)
                {
                    foreach (var f in stateMachineInstance.GetType().GetFields(BindingFlags.Instance | BindingFlags.Public | BindingFlags.NonPublic))
                    {
                        string valStr;
                        try { valStr = f.GetValue(stateMachineInstance)?.ToString() ?? "null"; }
                        catch (Exception fieldEx) { valStr = $"(read failed: {fieldEx.Message})"; }
                        Log.LogWarning($"[BD2CompatPatch] Crash site captured state: {f.FieldType.Name} {f.Name} = {valStr}");
                    }
                }
            }
            catch (Exception diagEx)
            {
                Log.LogWarning($"[BD2CompatPatch] LogCrashLocationAndCapturedState itself failed: {diagEx.Message}");
            }
        }

        // Non-generic, safe to patch normally (see the call site comment for why the generic
        // method itself can't be touched directly on this Mono runtime). __args[0] is the UI name
        // string being checked; only override the result for the two names confirmed stuck.
        private static readonly HashSet<string> _stuckUiTypeNames = new HashSet<string> { "NoticeUI", "OverheadManageUI" };

        private static void UnstickUiOpeningPostfix(object[] __args, ref bool __result)
        {
            if (__args.Length > 0 && __args[0] is string name && _stuckUiTypeNames.Contains(name) && __result)
            {
                Log.LogInfo($"[BD2CompatPatch] Unsticking IsUIOpening({name}) -> forcing false so the real load retries.");
                __result = false;
            }
        }

        // Blunt but effective: a local captured and assigned inside a lambda (e.g. `isLoadedUI`,
        // `isNext`, `isCheckComplete` -- this exact pattern recurs constantly in this codebase's
        // "wait for an async callback" idiom) gets lifted by the C# compiler into a heap-allocated
        // closure class, referenced from the iterator state machine's own fields rather than
        // stored directly on it. Walk the iterator's fields (and one level into any reference-type
        // field that looks like such a closure) and flip every bool currently false to true --
        // whichever specific flag was gating the stuck `while(!flag) yield return null;` gets
        // cleared, and the surrounding real game code resumes normally from there. Skips fields
        // that are clearly not local captures (Unity objects, strings, collections) to limit
        // blast radius.
        private static int TryForceStuckBoolFlags(object iteratorStateMachine)
        {
            int flipped = 0;
            try
            {
                flipped += FlipFalseBoolFields(iteratorStateMachine);
                var iteratorFields = iteratorStateMachine.GetType().GetFields(BindingFlags.Instance | BindingFlags.Public | BindingFlags.NonPublic);
                foreach (var f in iteratorFields)
                {
                    if (f.FieldType.IsPrimitive || f.FieldType == typeof(string) || typeof(UnityEngine.Object).IsAssignableFrom(f.FieldType))
                    {
                        continue;
                    }
                    object value;
                    try { value = f.GetValue(iteratorStateMachine); }
                    catch { continue; }
                    if (value == null) continue;
                    // Only descend into compiler-generated closures/state machines, not arbitrary
                    // game objects reachable from the coroutine (managers, tables, etc.) -- keeps
                    // this from reaching into and mutating unrelated live game state.
                    if (!value.GetType().Name.Contains("<") && !value.GetType().Name.Contains("DisplayClass"))
                    {
                        continue;
                    }
                    flipped += FlipFalseBoolFields(value);
                }
            }
            catch (Exception e)
            {
                Log.LogWarning($"[BD2CompatPatch] TryForceStuckBoolFlags reflection failed: {e.Message}");
            }
            return flipped;
        }

        private static int FlipFalseBoolFields(object obj)
        {
            int flipped = 0;
            foreach (var f in obj.GetType().GetFields(BindingFlags.Instance | BindingFlags.Public | BindingFlags.NonPublic))
            {
                if (f.FieldType != typeof(bool)) continue;
                bool current;
                try { current = (bool)f.GetValue(obj); }
                catch { continue; }
                if (!current)
                {
                    f.SetValue(obj, true);
                    flipped++;
                }
            }
            return flipped;
        }

        // Stubs every null Component/GameObject-typed field on `obj`, and recurses into non-null
        // reference-typed fields whose declared type is nested inside `rootType` (e.g.
        // GameFieldDefaultUI+ProgressInfo) so their OWN null fields get stubbed too, without
        // wandering into unrelated singletons/managers reachable from the same object graph.
        private static int StubNullFieldsRecursive(object obj, Type rootType, GameObject stubRoot, HashSet<object> visited, int depth)
        {
            if (obj == null || depth > 4 || !visited.Add(obj)) return 0;
            int stubbed = 0;
            foreach (var f in obj.GetType().GetFields(BindingFlags.Instance | BindingFlags.Public | BindingFlags.NonPublic))
            {
                if (f.FieldType.IsValueType || f.FieldType == typeof(string)) continue;
                object current;
                try { current = f.GetValue(obj); } catch { continue; }
                if (current == null)
                {
                    try
                    {
                        if (f.FieldType == typeof(GameObject))
                        {
                            var stubGo = new GameObject("Stub_" + f.Name);
                            stubGo.transform.SetParent(stubRoot.transform);
                            f.SetValue(obj, stubGo);
                            stubbed++;
                        }
                        else if (typeof(Component).IsAssignableFrom(f.FieldType))
                        {
                            var stubGo = new GameObject("Stub_" + f.Name);
                            stubGo.transform.SetParent(stubRoot.transform);
                            var comp = stubGo.AddComponent(f.FieldType);
                            // HUD.IsActive (and other HUD base-class members) read a
                            // [SerializeField] `root` GameObject that only ever gets populated by
                            // Unity's own prefab deserialization -- a bare AddComponent stub has
                            // no such data, so `root` stays null and HUD.IsActive throws on every
                            // single LateUpdate call project-wide (confirmed: this exact crash was
                            // spamming every frame). Point it at the stub's own GameObject so the
                            // property returns a real (if meaningless) bool instead of throwing.
                            if (typeof(HUD).IsAssignableFrom(f.FieldType))
                            {
                                var rootField = typeof(HUD).GetField("root", BindingFlags.Instance | BindingFlags.NonPublic);
                                rootField?.SetValue(comp, stubGo);
                            }
                            f.SetValue(obj, comp);
                            stubbed++;
                        }
                    }
                    catch (Exception stubEx)
                    {
                        Log.LogWarning($"[BD2CompatPatch] Could not stub {f.Name} ({f.FieldType.Name}): {stubEx.Message}");
                    }
                }
                else if (f.FieldType.IsClass && !typeof(UnityEngine.Object).IsAssignableFrom(f.FieldType)
                    && f.FieldType.DeclaringType == rootType)
                {
                    stubbed += StubNullFieldsRecursive(current, rootType, stubRoot, visited, depth + 1);
                }
            }
            return stubbed;
        }

        private static void WrapCoroutinePostfix(MethodBase __originalMethod, object[] __args, ref IEnumerator __result)
        {
            string argSummary = __args != null && __args.Length > 0 ? string.Join(",", __args.Select(a => a?.ToString() ?? "null")) : "";
            string label = $"{__originalMethod.DeclaringType?.Name}.{__originalMethod.Name}({argSummary})";
            __result = LoggingCoroutineWrapper(label, __result);
        }

        private static void AlwaysTruePostfix(ref bool __result)
        {
            __result = true;
        }

        private static void AlwaysFalsePostfix(ref bool __result)
        {
            __result = false;
        }

        private static void AssetLoadDiagnosticPostfix(object[] __args, AsyncOperationHandle<GameObject> __result)
        {
            string key = __args.Length > 0 ? __args[0] as string : "?";
            Log.LogInfo($"[BD2CompatPatch] GetPrefabAsset requested: {key}");
            var handle = __result;
            handle.Completed += (op) =>
            {
                Log.LogInfo($"[BD2CompatPatch] GetPrefabAsset completed: {key} status={op.Status} exception={op.OperationException?.Message ?? "none"}");
            };
        }

        private static void EnterPackDiagnostic([HarmonyArgument(0)] ref PackTable pack)
        {
            if (pack == null) return;
            Log.LogInfo($"[BD2CompatPatch] PackManager.EnterPack called with Id={pack.Id}, StartPositionPath={pack.StartPositionPath}");

            // The EnterPack(21) -> EnterPack(1) redirect that used to live here was a workaround
            // for the stale bundle_version bug (pack21's real map bundle -- pack21-bundlepack21_
            // assets_p21_map.bundle -- 404'd on the CDN, so we silently substituted pack1's data).
            // Now that bundle_version is current, pack21's own bundle resolves fine (verified with
            // a direct curl: HTTP 200), and the redirect actively causes bugs of its own: whatever
            // UI element tracks "current pack" as 21 while we'd quietly loaded pack1's content
            // underneath it -- the back-arrow trying to return to "the current pack" would hang,
            // and pack1's sprites/overworld models would render in a layout meant for pack21,
            // showing up as greyed-out/cut-off. Removed; only the diagnostic log above remains.
        }

        // See the registration comment above (near "the actual ... why does a fresh-ish account
        // enter pack21") for the full story.
        private static void ForceInitPackId1Postfix(ref int __result)
        {
            __result = 21; // TEMP: testing the new black-screen recovery fix on pack21 before deciding
        }

        // __args works regardless of the real (obfuscated) parameter names — index 1 is the
        // `ignore` bool that makes FinalWorking treat the exception as non-fatal.
        // Skip the original method entirely rather than trying to override its `ignore`
        // argument — modifying __args for a value-type (bool) parameter didn't propagate to
        // the original call in testing. The only externally-visible effect of FinalWorking is
        // buffer-clearing/logging plus the conditional Application.Quit(); skipping all of it
        // is safe and guarantees the quit never happens.
        private static bool NeverFatalPrefix()
        {
            return false;
        }

        // Skips the original (broken) mapper call entirely when its row argument is null,
        // returning a freshly-constructed default T instead (the method's own generic
        // constraint guarantees T has a public parameterless constructor).
        private static bool NullRowPrefix(object[] __args, MethodBase __originalMethod, ref object __result)
        {
            if (__args.Length > 0 && __args[0] == null)
            {
                var returnType = ((MethodInfo)__originalMethod).ReturnType;
                __result = Activator.CreateInstance(returnType);
                Log.LogInfo($"[BD2CompatPatch] Skipped null-row mapper call ({returnType.Name}) to avoid NullReferenceException.");
                return false;
            }
            return true;
        }

        // ref object works for any reference-type return value (List<T> is a reference type) —
        // Harmony converts to/from the real declared type automatically.
        private static void NullListPostfix(MethodBase __originalMethod, ref object __result)
        {
            if (__originalMethod.Name == "GetMercernaryScoutInfoByType")
            {
                var count = __result == null ? -1 : ((System.Collections.IList)__result).Count;
                Log.LogInfo($"[BD2CompatPatch] GetMercernaryScoutInfoByType postfix fired, result={(count < 0 ? "null" : count + " items")}");
            }

            if (__result == null)
            {
                var elementType = ((MethodInfo)__originalMethod).ReturnType.GetGenericArguments()[0];
                __result = Activator.CreateInstance(typeof(List<>).MakeGenericType(elementType));
                Log.LogInfo($"[BD2CompatPatch] {__originalMethod.Name} returned null -> replaced with empty list");
                return;
            }

            // Also seen: a non-null list that still contains individual null entries (a row
            // that failed to parse/deserialize from the real, current production master data).
            // List<T> always implements the non-generic IList, so this works without needing T.
            if (__result is System.Collections.IList list)
            {
                int removed = 0;
                for (int i = list.Count - 1; i >= 0; i--)
                {
                    if (list[i] == null) { list.RemoveAt(i); removed++; }
                }
                if (removed > 0)
                {
                    Log.LogInfo($"[BD2CompatPatch] {__originalMethod.Name} returned {removed} null element(s) -> stripped");
                }
            }
        }
    }
}
