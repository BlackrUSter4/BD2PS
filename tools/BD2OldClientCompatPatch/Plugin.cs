using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;
using BepInEx;
using BepInEx.Logging;
using HarmonyLib;
using Mono.Cecil;
using Mono.Cecil.Cil;
using Proto.Design.common;
using Proto.Net;
using UnityEngine;

namespace BD2OldClientCompatPatch
{
	[BepInPlugin("com.bd2ps.oldclientcompatpatch", "BD2OldClientCompatPatch", "0.1.0")]
	public class Plugin : BaseUnityPlugin
	{
		internal static ManualLogSource Log;

		private void Awake()
		{
			Log = Logger;
			Harmony harmony = new Harmony("com.bd2ps.oldclientcompatpatch");
			harmony.PatchAll();

			// Diagnostic: capture the real call stack for any "table not found"
			// exception. Already used once (2026-10-02) to find the real CharTable
			// caller when a name-based static-decompile guess turned out wrong.
			ConstructorInfo ctor = AccessTools.Constructor(typeof(DataNotFoundException), new[] { typeof(string), typeof(int), typeof(string) });
			if (ctor != null)
			{
				DataNotFoundExceptionDiagnostic.HarmonyInstance = harmony;
				harmony.Patch(ctor, prefix: new HarmonyMethod(typeof(DataNotFoundExceptionDiagnostic), nameof(DataNotFoundExceptionDiagnostic.Prefix)));
			}

			// Composite-key overload - confirmed live, 2026-10-02: CharLevelTable is
			// looked up by (CharGrowthId, Level), a totally different method from the
			// single-int GetValueObject<T> path already handled above, and that
			// composite lookup was still failing even after the simple-key one got
			// patched.
			ConstructorInfo ctorComposite = AccessTools.Constructor(typeof(DataNotFoundException), new[] { typeof(string), typeof(int), typeof(int), typeof(string) });
			if (ctorComposite != null)
			{
				CompositeKeyDataNotFoundDiagnostic.HarmonyInstance = harmony;
				harmony.Patch(ctorComposite, prefix: new HarmonyMethod(typeof(CompositeKeyDataNotFoundDiagnostic), nameof(CompositeKeyDataNotFoundDiagnostic.Prefix)));
				Log.LogInfo("[BD2OldClientCompatPatch] Composite-key self-heal armed.");
			}

			PatchGetValueObject(harmony);
			PatchAllTableLookupsOnAccessorClass(harmony);
			PatchGenericExceptionSelfHeal(harmony);
			PatchStatCalcNullGuard(harmony);

			PatchSkipFieldEntry(harmony);
			PatchRedirectBrokenFieldScene(harmony);

			StartCoroutine(DumpVisibleTextPeriodically());

			Log.LogInfo("[BD2OldClientCompatPatch] Loaded.");
		}

		// Reads whatever the disconnect notification's FULL text actually says -
		// screen captures of this window are cropped at the right edge and the
		// box visibly extends past it, so this is more reliable than a screenshot.
		// Reflection-based (no UnityEngine.UI/TMPro project reference needed): finds
		// any loaded type named exactly "Text" or "TextMeshProUGUI"/"TMP_Text" across
		// every loaded assembly, calls Object.FindObjectsOfType on each, and logs
		// every active, non-empty string found along with its GameObject's name.
		private static IEnumerator DumpVisibleTextPeriodically()
		{
			var textTypes = new List<Type>();
			foreach (Assembly asm in AppDomain.CurrentDomain.GetAssemblies())
			{
				Type[] types;
				try { types = asm.GetTypes(); }
				catch { continue; }
				foreach (Type t in types)
				{
					if ((t.Name == "Text" || t.Name == "TextMeshProUGUI" || t.Name == "TMP_Text")
						&& typeof(UnityEngine.Object).IsAssignableFrom(t))
					{
						textTypes.Add(t);
					}
				}
			}
			Log.LogInfo($"[BD2OldClientCompatPatch] [textdump] found {textTypes.Count} text component type(s): {string.Join(", ", textTypes.Select(t => t.FullName))}");

			int tick = 0;
			while (true)
			{
				yield return new WaitForSecondsRealtime(5f);
				tick++;
				Log.LogInfo($"[BD2OldClientCompatPatch] [textdump] tick {tick}");
				foreach (Type textType in textTypes)
				{
					UnityEngine.Object[] found;
					try
					{
						found = Resources.FindObjectsOfTypeAll(textType);
					}
					catch (Exception e)
					{
						Log.LogWarning($"[BD2OldClientCompatPatch] [textdump] FindObjectsOfTypeAll({textType.Name}) failed: {e.Message}");
						continue;
					}
					PropertyInfo textProp = textType.GetProperty("text");
					foreach (UnityEngine.Object obj in found)
					{
						if (obj is not Component comp || !comp.gameObject.activeInHierarchy)
						{
							continue;
						}
						string text = textProp?.GetValue(comp) as string;
						if (!string.IsNullOrWhiteSpace(text))
						{
							Log.LogInfo($"[BD2OldClientCompatPatch] [textdump] {comp.gameObject.name}: \"{text}\"");
						}
					}
				}
			}
		}

		// Generalized version of the same self-heal idea, for exceptions that
		// AREN'T DataNotFoundException. Confirmed live, 2026-10-02: a genuine
		// NullReferenceException deep in character-stat-refresh logic
		// (route "AllCharRefresh") happens in a DIFFERENT top-level handler than
		// the one already wrapped with a finalizer - manually chasing each new
		// obfuscated method name one at a time (via ilspycmd across SEPARATE
		// invocations) turned out fundamentally unreliable here: ilspycmd's
		// placeholder-name assignment for obfuscated members is not even
		// deterministic run-to-run, so a name read from one decompile and typed
		// into a later, separate ilspycmd invocation can silently refer to nothing.
		// Live runtime reflection (via a real exception's own StackTrace) does not
		// have this problem - it reads the assembly's actual metadata every time.
		// So: hook Unity's own stable, non-obfuscated Debug.LogException, and for
		// any exception whose stack touches Assembly-CSharp code, wrap every such
		// frame with the same exception-swallowing finalizer, reactively,
		// forever, the first time each one is actually seen live. Doesn't save the
		// CURRENT occurrence (already past the point of interception by the time
		// LogException runs) but self-heals every later retry, same as the
		// DataNotFoundException self-heal already does for that narrower case.
		// Real root cause of the "Disconnected from the server" / AllCharRefresh
		// disconnect loop, found live 2026-10-02 via the logMessageReceived
		// diagnostic net: a genuine NullReferenceException inside
		// ὣὭὤὮὤὢὧὭὮὡὭ.ὫὩὭὭὨὨὡὢὨὧὡ(enum, CharTable, CharLevelTable) - the per-character
		// stat-calc routine called once per character while building the
		// AllCharRefresh response. Both CharTable and CharLevelTable are pure
		// value-type protobuf messages (decompiled and confirmed - no nested
		// reference fields), so neither fallback object itself can be the null;
		// something else inside this method (a different table/cache lookup for
		// whichever specific character is being processed) is the real null.
		// Rather than keep chasing that one field through more obfuscated,
		// decompile-unstable IL, apply the same proven exception-swallowing
		// finalizer pattern already used elsewhere in this plugin directly to
		// this exact method - this obfuscated name has now shown up identically
		// across many separate live runs today, so (unlike ilspycmd's
		// non-deterministic placeholder names) it's safe to hardcode.
		// Swallowing here just means that one character's stats don't populate
		// for this response instead of tearing down the whole connection.
		// Confirmed live 2026-10-02: even with every downstream crash site in
		// the field-entry cascade guarded (async scene-load completion,
		// GameCameraManager.Awake, its nested sibling), the process still
		// dies with NO further log output shortly after they all fire - this
		// looks like a native-level crash from GameCameraManager being left
		// half-initialized by a finalizer that swallowed an exception
		// mid-Awake(), then used by Unity's own render loop on a later frame
		// (something a catchable C# exception guard fundamentally can't fix).
		// This client has NO field/map scene assets at all (confirmed against
		// the Addressables catalog - only Empty/MyRoom/ReGame/Splash are
		// bundled), so field entry can never succeed here regardless of which
		// map id is requested. Better fix: skip the whole attempt at its
		// entry point instead of letting it start and catching the fallout.
		// This entry point (ὤὤὢὯὭὠὤὧὠὫὯ.ὩὪὧὪὣὠὡὥὭὤὮ()) is a DIFFERENT
		// top-level handler than the one the working shop/pack-collection
		// flow goes through (IntroUI.ὢὩὫὩὦὬὭὫὭὧὦ()), so skipping it should
        // not affect the already-working path.
		private static void PatchSkipFieldEntry(Harmony harmony)
		{
			Type outerType = typeof(CharDBInfo).Assembly.GetTypes().FirstOrDefault(t => t.Name == "ὤὤὢὯὭὠὤὧὠὫὯ");
			MethodInfo target = outerType
				?.GetMethods(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Static | BindingFlags.Instance)
				.FirstOrDefault(m => m.Name == "ὩὪὧὪὣὠὡὥὭὤὮ" && m.GetParameters().Length == 0);
			if (target == null)
			{
				Log.LogWarning("[BD2OldClientCompatPatch] Field-entry skip target not found.");
				return;
			}
			try
			{
				harmony.Patch(target, prefix: new HarmonyMethod(typeof(SkipFieldEntryPatch), nameof(SkipFieldEntryPatch.Prefix)));
				Log.LogInfo($"[BD2OldClientCompatPatch] Field-entry skip armed on {outerType.Name}.{target.Name}().");
			}
			catch (Exception e)
			{
				Log.LogWarning($"[BD2OldClientCompatPatch] Could not arm field-entry skip: {e.Message}");
			}
		}

		// The real fix for the field-entry problem, found 2026-10-02 after
		// chasing individual crash sites in the cascade proved unreliable
		// (multiple independent code paths trigger field entry, and catching
		// the fallout after Addressables throws leaves things like
		// GameCameraManager half-initialized, which looks like it causes a
		// native-level crash no catchable C# guard can fix). Confirmed via
		// the client's own Addressables catalog
		// (com.unity.addressables/catalog_alpha.json) that field/map scenes
		// use keys shaped "<PackId>_Map/Scenes/<name>.unity", and NONE of
		// those exist in this build at all (zero matches for any "*.unity"
		// key under "*/Scenes/*") - only four flat keys exist: Scenes/Empty,
		// Scenes/MyRoom, Scenes/ReGame, Scenes/Splash. So instead of letting
		// a broken field key reach Addressables and fail, intercept
		// Addressables.LoadSceneAsync(object key, ...) itself (the one true
		// common chokepoint every field-entry path funnels through) and
		// REWRITE the key to "Scenes/MyRoom" whenever it matches the broken
		// shape - redirecting to the client's real, working home scene
		// instead of just suppressing a crash.
		private static void PatchRedirectBrokenFieldScene(Harmony harmony)
		{
			Type addressablesType = AccessTools.TypeByName("UnityEngine.AddressableAssets.Addressables");
			MethodInfo loadSceneAsync = addressablesType
				?.GetMethods(BindingFlags.Public | BindingFlags.Static)
				.FirstOrDefault(m => m.Name == "LoadSceneAsync"
					&& m.GetParameters().Length == 4
					&& m.GetParameters()[0].ParameterType == typeof(object)
					&& m.GetParameters()[1].ParameterType.Name == "LoadSceneMode");
			if (loadSceneAsync == null)
			{
				Log.LogWarning("[BD2OldClientCompatPatch] Addressables.LoadSceneAsync(object,...) not found for redirect patch.");
				return;
			}
			try
			{
				harmony.Patch(loadSceneAsync, prefix: new HarmonyMethod(typeof(RedirectBrokenFieldScenePatch), nameof(RedirectBrokenFieldScenePatch.Prefix)));
				Log.LogInfo("[BD2OldClientCompatPatch] Field-scene redirect armed on Addressables.LoadSceneAsync(object,...).");
			}
			catch (Exception e)
			{
				Log.LogWarning($"[BD2OldClientCompatPatch] Could not arm field-scene redirect: {e.Message}");
			}
		}

		private static void PatchStatCalcNullGuard(Harmony harmony)
		{
			Type declaringType = typeof(CharDBInfo).Assembly.GetTypes().FirstOrDefault(t => t.Name == "ὣὭὤὮὤὢὧὭὮὡὭ");
			if (declaringType == null)
			{
				Log.LogWarning("[BD2OldClientCompatPatch] StatCalcNullGuard: declaring type not found.");
				return;
			}
			MethodInfo target = declaringType
				.GetMethods(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Static | BindingFlags.Instance)
				.FirstOrDefault(m => m.Name == "ὫὩὭὭὨὨὡὢὨὧὡ"
					&& m.GetParameters().Length == 3
					&& m.GetParameters().Any(p => p.ParameterType == typeof(CharLevelTable))
					&& m.GetParameters().Any(p => p.ParameterType == typeof(CharTable)));
			if (target == null)
			{
				Log.LogWarning("[BD2OldClientCompatPatch] StatCalcNullGuard: target method not found.");
				return;
			}
			try
			{
				harmony.Patch(target, finalizer: new HarmonyMethod(typeof(SwallowExceptionPatch), nameof(SwallowExceptionPatch.Finalizer)));
				Log.LogInfo($"[BD2OldClientCompatPatch] StatCalcNullGuard: wrapped {declaringType.FullName}.{target.Name}(enum,CharTable,CharLevelTable).");
			}
			catch (Exception e)
			{
				Log.LogWarning($"[BD2OldClientCompatPatch] StatCalcNullGuard failed: {e.Message}");
			}

			// Confirmed live 2026-10-02: swallowing the inner stat-calc NRE above
			// is NOT enough - its caller still unconditionally uses the (now
			// missing) result and throws its OWN exception, which still
			// propagates all the way up to the top-level "AllCharRefresh" packet
			// handler and still disconnects. Wrap THAT top-level handler too, as
			// the outer safety net - its exact name has shown up identically in
			// every single run's stack trace today (parameterless, declared on
			// "ὤὤὢὯὭὠὤὧὠὫὯ"), so hardcoding it is safe the same way the inner one
			// was. This is the method the project's own notes already identified
			// as "the real site" for the AllCharRefresh disconnect, previously
			// left unwrapped because a different (wrong) top-level handler got
			// wrapped instead.
			// A SECOND, unrelated, and apparently FATAL gap - confirmed live,
			// 2026-10-02, reproducible on every single launch after the
			// AllCharRefresh fix above cleared the way to it: once the client
			// gets past character refresh, IntroUI calls into
			// PackagePopupUI.GetEntranceUIDataList() (a real, non-obfuscated
			// class/method name - no fragile lookup needed) to build the
			// shop/IAP entrance popup from CashPackageTable rows, and throws a
			// NullReferenceException there that kills the whole process outright
			// - not just a disconnect-and-retry this time. Because it's fatal on
			// FIRST occurrence, the reactive self-heal (which only wraps a
			// method AFTER seeing it throw once, via Debug.LogException) is too
			// late to save it - this one needs a PROACTIVE wrap in place before
			// it's ever called at all.
			Type packagePopupType = AccessTools.TypeByName("PackagePopupUI");
			MethodInfo entranceListMethod = packagePopupType != null
				? AccessTools.Method(packagePopupType, "GetEntranceUIDataList")
				: null;
			if (entranceListMethod != null)
			{
				try
				{
					// This one returns List<T> - a bare swallow leaves __result at
					// null (the method threw before reaching its own "return
					// list;"), and the caller iterates the result unconditionally,
					// which would just trade this crash for an immediate new one.
					// Force an empty instance of the real return type instead.
					SafeEmptyResultPatch.ResultTypes[entranceListMethod] = entranceListMethod.ReturnType;
					harmony.Patch(entranceListMethod, finalizer: new HarmonyMethod(typeof(SafeEmptyResultPatch), nameof(SafeEmptyResultPatch.Finalizer)));
					Log.LogInfo("[BD2OldClientCompatPatch] StatCalcNullGuard: proactively wrapped PackagePopupUI.GetEntranceUIDataList() with a safe-empty-result guard.");
				}
				catch (Exception e)
				{
					Log.LogWarning($"[BD2OldClientCompatPatch] Could not proactively wrap PackagePopupUI.GetEntranceUIDataList: {e.Message}");
				}
			}

			// A THIRD gap in the same shop/package UI family, confirmed live
			// 2026-10-02 right after the two above: this one is reached via an
			// ASYNC Addressables load-completion callback
			// (PackInfoUI prefab instantiation finishing), not synchronously from
			// IntroUI's own call chain - so wrapping an earlier method in that
			// chain would never have caught it. Confirmed this run: client
			// actually reached the lobby (UID shown, settings/power icons
			// visible on screen) for the first time all session, then crashed
			// here shortly after. ArgumentNullException from
			// Dictionary.ContainsKey(null) inside PackInfoUI+FrontPage, called
			// from PackInfoUI.SetNewPackInfoState - both real, stable class
			// names. Proactively guard the public entry point.
			Type packInfoType = AccessTools.TypeByName("PackInfoUI");
			MethodInfo setNewPackInfoState = packInfoType != null
				? AccessTools.Method(packInfoType, "SetNewPackInfoState")
				: null;
			if (setNewPackInfoState != null)
			{
				try
				{
					harmony.Patch(setNewPackInfoState, finalizer: new HarmonyMethod(typeof(SwallowExceptionPatch), nameof(SwallowExceptionPatch.Finalizer)));
					Log.LogInfo("[BD2OldClientCompatPatch] StatCalcNullGuard: proactively wrapped PackInfoUI.SetNewPackInfoState().");
				}
				catch (Exception e)
				{
					Log.LogWarning($"[BD2OldClientCompatPatch] Could not proactively wrap PackInfoUI.SetNewPackInfoState: {e.Message}");
				}
			}

			// A FOURTH gap, confirmed live 2026-10-02 right after the three
			// above cleared: client progressed past the lobby/shop popups into
			// loading an actual field/map scene, and hit a missing
			// FieldMonsterRegenDTO (field monster spawn data) entry, fatal again
			// on first occurrence because it's reached via an async batch
			// network response handler (BDNetwork.NetworkManager.Success -> a
			// UniTask continuation), same shape as the earlier async
			// PackInfoUI case. Proactively wrap the exact nested-class throw
			// site from the live stack trace.
			Type fieldMonsterOuterType = typeof(CharDBInfo).Assembly.GetTypes().FirstOrDefault(t => t.Name == "ὥὣὤὣὧὯὢὤὥὬὨ");
			MethodInfo fieldMonsterMethod = fieldMonsterOuterType
				?.GetMethods(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Static | BindingFlags.Instance)
				.FirstOrDefault(m => m.Name == "ὥὥὦὦὤὩὣὮὯὩὢ" && m.GetParameters().Length == 1 && m.GetParameters()[0].ParameterType == typeof(int));
			if (fieldMonsterMethod != null)
			{
				try
				{
					if (fieldMonsterMethod.ReturnType != typeof(void) && !fieldMonsterMethod.ReturnType.IsValueType)
					{
						SafeEmptyResultPatch.ResultTypes[fieldMonsterMethod] = fieldMonsterMethod.ReturnType;
						harmony.Patch(fieldMonsterMethod, finalizer: new HarmonyMethod(typeof(SafeEmptyResultPatch), nameof(SafeEmptyResultPatch.Finalizer)));
					}
					else
					{
						harmony.Patch(fieldMonsterMethod, finalizer: new HarmonyMethod(typeof(SwallowExceptionPatch), nameof(SwallowExceptionPatch.Finalizer)));
					}
					Log.LogInfo($"[BD2OldClientCompatPatch] StatCalcNullGuard: proactively wrapped {fieldMonsterOuterType.FullName}.{fieldMonsterMethod.Name}(int).");
				}
				catch (Exception e)
				{
					Log.LogWarning($"[BD2OldClientCompatPatch] Could not proactively wrap field monster method: {e.Message}");
				}
			}
			else
			{
				Log.LogWarning("[BD2OldClientCompatPatch] Field monster throw-site method not found for proactive wrap.");
			}

			// A FIFTH gap, confirmed live 2026-10-02 right after the four above
			// cleared: client progressed into actually loading a field/map
			// scene and crashed inside the LOADING SCREEN's own coroutine
			// (LoadingUI -> SoundManager.PlayFadeInBackgroundRunningSounds ->
			// PlayBackgroundSoundDependingOnMapInfo), a NullReferenceException
			// formatting a sound path from a MapTable field that's null for
			// this map (another old-client data gap, same family as
			// CharTable/CostumeTable). A coroutine exception reached directly
			// (not via the UniTask/async machinery that catches the sibling
			// deck-building exception seen in the same burst) is a much more
			// likely candidate for what actually killed the process, so fix
			// this one proactively first. SoundManager is a real, stable class
			// name - no fragile lookup needed.
			Type soundManagerType = AccessTools.TypeByName("SoundManager");
			MethodInfo playBgSound = soundManagerType != null
				? AccessTools.Method(soundManagerType, "PlayBackgroundSoundDependingOnMapInfo")
				: null;
			if (playBgSound != null)
			{
				try
				{
					harmony.Patch(playBgSound, finalizer: new HarmonyMethod(typeof(SwallowExceptionPatch), nameof(SwallowExceptionPatch.Finalizer)));
					Log.LogInfo("[BD2OldClientCompatPatch] StatCalcNullGuard: proactively wrapped SoundManager.PlayBackgroundSoundDependingOnMapInfo().");
				}
				catch (Exception e)
				{
					Log.LogWarning($"[BD2OldClientCompatPatch] Could not proactively wrap SoundManager.PlayBackgroundSoundDependingOnMapInfo: {e.Message}");
				}
			}
			else
			{
				Log.LogWarning("[BD2OldClientCompatPatch] SoundManager.PlayBackgroundSoundDependingOnMapInfo not found for proactive wrap.");
			}

			// A SIXTH gap, confirmed live 2026-10-02: a THIRD, previously
			// unseen overload pair on the SAME CostumeTable accessor class
			// already known from PatchAllTableLookupsOnAccessorClass (that one
			// patched ὢὣὭὢὥὭὭὤὯὬὬ for CostumeTable and ὠὦὠὠὡὦὭὯὢὦὦ for
			// CostumeNodeGroupTable) - this one, named ὧὫὬὦὩὥὠὣὩὨὦ, has an
			// (int) and a (CharDBInfo) overload, reached via the async
			// deck-building chain during field-related batch processing.
			// Proactively wrap both overloads with a safe-empty-result guard
			// (both presumably return CostumeTable, used unconditionally by
			// callers).
			Type costumeAccessorType = typeof(CharDBInfo).Assembly.GetTypes().FirstOrDefault(t => t.Name == "ὥὨὫὬὣὨὮὮὡὬὣ");
			if (costumeAccessorType != null)
			{
				foreach (MethodInfo overload in costumeAccessorType
					.GetMethods(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Static | BindingFlags.Instance)
					.Where(m => m.Name == "ὧὫὬὦὩὥὠὣὩὨὦ" && m.GetParameters().Length == 1))
				{
					try
					{
						if (overload.ReturnType != typeof(void) && !overload.ReturnType.IsValueType)
						{
							SafeEmptyResultPatch.ResultTypes[overload] = overload.ReturnType;
							harmony.Patch(overload, finalizer: new HarmonyMethod(typeof(SafeEmptyResultPatch), nameof(SafeEmptyResultPatch.Finalizer)));
						}
						else
						{
							harmony.Patch(overload, finalizer: new HarmonyMethod(typeof(SwallowExceptionPatch), nameof(SwallowExceptionPatch.Finalizer)));
						}
						Log.LogInfo($"[BD2OldClientCompatPatch] StatCalcNullGuard: proactively wrapped {costumeAccessorType.Name}.ὧὫὬὦὩὥὠὣὩὨὦ({overload.GetParameters()[0].ParameterType.Name}).");
					}
					catch (Exception e)
					{
						Log.LogWarning($"[BD2OldClientCompatPatch] Could not proactively wrap costume accessor overload: {e.Message}");
					}
				}
			}
			else
			{
				Log.LogWarning("[BD2OldClientCompatPatch] Costume accessor type not found for third-overload proactive wrap.");
			}

			// A SEVENTH gap, confirmed live 2026-10-02: clearing the account's
			// saved field position (PositionInfo/MapActiveInfo/WayPointInfo)
			// stops login from trying to RESUME a field on its own, but the
			// shop/pack-collection flow independently tries to enter a field
			// afterward regardless - hitting the exact same missing-asset
			// problem (no field/map scene bundled in this client at all, see
			// the P0_Map InvalidKeyException notes elsewhere in this file).
			// The reactive self-heal DID catch this (confirmed: "Generic
			// self-heal: wrapped ...ὯὭὧὡὥὭὦὤὩὩὧ.ὯὯὭὭὥὡὭὠὯὪὨ after seeing it
			// throw live" appears in the log), but only after the first,
			// still-fatal occurrence already killed the run. Proactively wrap
			// it from the start using the now-confirmed-stable name (resolved
			// successfully via reflection multiple times this session) so the
			// very first occurrence is also safe.
			Type asyncOpHandlerType = typeof(CharDBInfo).Assembly.GetTypes().FirstOrDefault(t => t.Name == "ὯὭὧὡὥὭὦὤὩὩὧ");
			MethodInfo asyncOpHandlerMethod = asyncOpHandlerType
				?.GetMethods(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Static | BindingFlags.Instance)
				.FirstOrDefault(m => m.Name == "ὯὯὭὭὥὡὭὠὯὪὨ" && !m.IsGenericMethodDefinition && !m.ContainsGenericParameters);
			if (asyncOpHandlerMethod != null)
			{
				try
				{
					harmony.Patch(asyncOpHandlerMethod, finalizer: new HarmonyMethod(typeof(SwallowExceptionPatch), nameof(SwallowExceptionPatch.Finalizer)));
					Log.LogInfo($"[BD2OldClientCompatPatch] StatCalcNullGuard: proactively wrapped {asyncOpHandlerType.Name}.ὯὯὭὭὥὡὭὠὯὪὨ() (field-load async completion).");
				}
				catch (Exception e)
				{
					Log.LogWarning($"[BD2OldClientCompatPatch] Could not proactively wrap field-load async completion: {e.Message}");
				}
			}
			else
			{
				Log.LogWarning("[BD2OldClientCompatPatch] Field-load async completion method not found for proactive wrap.");
			}

			// An EIGHTH gap, confirmed live 2026-10-02, cascading directly from
			// the seventh above: even with the field-load completion handler
			// itself guarded, GameCameraManager.Awake() (a real, stable class
			// name - the camera setup that runs when the game tries to move
			// into the now-failed field scene) still throws its own NRE
			// downstream, because it assumes field data exists unconditionally.
			// Proactively guard it directly.
			Type gameCameraManagerType = AccessTools.TypeByName("GameCameraManager");
			MethodInfo gameCameraAwake = gameCameraManagerType != null
				? AccessTools.Method(gameCameraManagerType, "Awake")
				: null;
			if (gameCameraAwake != null)
			{
				try
				{
					harmony.Patch(gameCameraAwake, finalizer: new HarmonyMethod(typeof(SwallowExceptionPatch), nameof(SwallowExceptionPatch.Finalizer)));
					Log.LogInfo("[BD2OldClientCompatPatch] StatCalcNullGuard: proactively wrapped GameCameraManager.Awake().");
				}
				catch (Exception e)
				{
					Log.LogWarning($"[BD2OldClientCompatPatch] Could not proactively wrap GameCameraManager.Awake: {e.Message}");
				}
			}
			else
			{
				Log.LogWarning("[BD2OldClientCompatPatch] GameCameraManager.Awake not found for proactive wrap.");
			}

			// Same cascade, a second confirmed sibling call site: a nested
			// class on the same ὠὤὩὣὧὬὦὮὩὡὨ (field manager) type, seen
			// reactively self-healing this same run right after the above -
			// wrap it proactively too now that its name is confirmed stable.
			Type fieldManagerOuterType = typeof(CharDBInfo).Assembly.GetTypes().FirstOrDefault(t => t.Name == "ὠὤὩὣὧὬὦὮὩὡὨ");
			Type fieldManagerNestedType = typeof(CharDBInfo).Assembly.GetTypes().FirstOrDefault(t => t.Name == "ὣὯὠὭὪὤὪὢὣὡὥ");
			MethodInfo fieldManagerNestedMethod = fieldManagerNestedType
				?.GetMethods(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Static | BindingFlags.Instance)
				.FirstOrDefault(m => m.Name == "ὣὪὥὨὢὡὤὭὦὤὧ" && !m.IsGenericMethodDefinition && !m.ContainsGenericParameters);
			if (fieldManagerNestedMethod != null)
			{
				try
				{
					harmony.Patch(fieldManagerNestedMethod, finalizer: new HarmonyMethod(typeof(SwallowExceptionPatch), nameof(SwallowExceptionPatch.Finalizer)));
					Log.LogInfo($"[BD2OldClientCompatPatch] StatCalcNullGuard: proactively wrapped {fieldManagerOuterType?.Name}+{fieldManagerNestedType.Name}.ὣὪὥὨὢὡὤὭὦὤὧ().");
				}
				catch (Exception e)
				{
					Log.LogWarning($"[BD2OldClientCompatPatch] Could not proactively wrap field manager nested method: {e.Message}");
				}
			}

			Type outerType = typeof(CharDBInfo).Assembly.GetTypes().FirstOrDefault(t => t.Name == "ὤὤὢὯὭὠὤὧὠὫὯ");
			if (outerType == null)
			{
				Log.LogWarning("[BD2OldClientCompatPatch] StatCalcNullGuard: outer AllCharRefresh type not found.");
				return;
			}
			MethodInfo outerTarget = outerType
				.GetMethods(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Static | BindingFlags.Instance)
				.FirstOrDefault(m => m.Name == "ὨὥὩὫὭὩὫὩὭὯὠ" && m.GetParameters().Length == 0);
			if (outerTarget == null)
			{
				Log.LogWarning("[BD2OldClientCompatPatch] StatCalcNullGuard: outer AllCharRefresh method not found.");
				return;
			}
			try
			{
				harmony.Patch(outerTarget, finalizer: new HarmonyMethod(typeof(SwallowExceptionPatch), nameof(SwallowExceptionPatch.Finalizer)));
				Log.LogInfo($"[BD2OldClientCompatPatch] StatCalcNullGuard: wrapped outer handler {outerType.FullName}.{outerTarget.Name}().");
			}
			catch (Exception e)
			{
				Log.LogWarning($"[BD2OldClientCompatPatch] StatCalcNullGuard outer wrap failed: {e.Message}");
			}
		}

		private static void PatchGenericExceptionSelfHeal(Harmony harmony)
		{
			GenericExceptionSelfHeal.HarmonyInstance = harmony;
			HarmonyMethod prefix = new HarmonyMethod(typeof(GenericExceptionSelfHeal), nameof(GenericExceptionSelfHeal.Prefix));
			int armed = 0;

			// Two separate overloads exist (Exception) and (Exception, Object) - the
			// game's internal top-level catch handler might call either one. Missing
			// the one actually used here meant this self-heal silently never fired at
			// all despite looking armed (confirmed live, 2026-10-02: zero "wrapped"
			// log lines ever appeared across several runs even though the client kept
			// disconnecting/retrying the whole time - the 2-arg-only hook was the gap).
			MethodInfo logException2 = AccessTools.Method(typeof(UnityEngine.Debug), "LogException", new[] { typeof(Exception), typeof(UnityEngine.Object) });
			if (logException2 != null)
			{
				harmony.Patch(logException2, prefix: prefix);
				armed++;
			}
			MethodInfo logException1 = AccessTools.Method(typeof(UnityEngine.Debug), "LogException", new[] { typeof(Exception) });
			if (logException1 != null)
			{
				harmony.Patch(logException1, prefix: prefix);
				armed++;
			}

			// Belt-and-suspenders: Application.logMessageReceived fires for EVERY
			// Debug.Log* call made through ANY path, including internal engine
			// exception reporting that might not even go through the public
			// Debug.LogException API at all. This gives us the exception's message
			// and a STRING stack trace (no live StackFrame objects), so it can't
			// drive the Harmony-patch-by-MethodInfo self-heal directly - it's used
			// purely as a diagnostic net to confirm whether an exception happened at
			// all when the two hooks above stay silent.
			UnityEngine.Application.logMessageReceived += (string condition, string stackTrace, UnityEngine.LogType type) =>
			{
				if (type == UnityEngine.LogType.Exception || type == UnityEngine.LogType.Error)
				{
					Log.LogInfo($"[BD2OldClientCompatPatch] [logMessageReceived] {type}: {condition}\n{stackTrace}");
				}
			};

			Log.LogInfo($"[BD2OldClientCompatPatch] Generic exception self-heal armed on {armed} Debug.LogException overload(s) + logMessageReceived diagnostic.");
		}

		// The real fix, found 2026-10-02 after the per-accessor-class approach below
		// kept missing paths that went through generic caching layers Cecil can't see
		// through (a cache's factory lambda's own concrete type argument isn't visible
		// from the generic cache method's own signature). Traced CostumeTable's two
		// different failing call paths back to their source and found BOTH ultimately
		// call this exact same method - the real, shared, non-obfuscated, universal
		// single-row-by-id data primitive this whole client funnels through:
		// `RawDataManager.GetValueObject<T>(DatabaseType, string tableName, int key)
		//     where T : IMessage<T>, new()`.
		// Patching this ONE generic method (Harmony supports patching an open generic
		// method definition; it applies to every closed instantiation) fixes every
		// current AND future single-key table gap in one place, instead of chasing
		// each table's own wrapper/cache/overload maze individually.
		private static MethodInfo _getValueObjectOpenGeneric;
		private static readonly HashSet<Type> _getValueObjectPatchedTypes = new();

		private static MethodInfo GetValueObjectOpenGeneric()
		{
			return _getValueObjectOpenGeneric ??= typeof(RawDataManager)
				.GetMethods(BindingFlags.Public | BindingFlags.Instance)
				.FirstOrDefault(m => m.Name == "GetValueObject" && m.IsGenericMethodDefinition && m.GetParameters().Length == 3);
		}

		// Resolves a table NAME (e.g. "CharLevelTable", as DataNotFoundException gives
		// it) to its real Type and patches that one closed GetValueObject<T>
		// instantiation, entirely independent of any fallback DATA being registered
		// for it - used reactively by the self-heal diagnostic for the universal id-0
		// sentinel case, which needs no per-table data, just a type to construct.
		internal static bool TryPatchGetValueObjectFor(string tableTypeName, Harmony harmony)
		{
			MethodInfo openGeneric = GetValueObjectOpenGeneric();
			if (openGeneric == null || harmony == null)
			{
				return false;
			}
			Type targetType = typeof(CharDBInfo).Assembly.GetTypes().FirstOrDefault(t => t.Name == tableTypeName);
			if (targetType == null || !_getValueObjectPatchedTypes.Add(targetType))
			{
				return _getValueObjectPatchedTypes.Contains(targetType); // already patched = still a "yes, handled"
			}
			return PatchOneClosedGetValueObject(openGeneric, targetType, harmony);
		}

		private static bool PatchOneClosedGetValueObject(MethodInfo openGeneric, Type targetType, Harmony harmony)
		{
			MethodInfo closed;
			try
			{
				closed = openGeneric.MakeGenericMethod(targetType);
			}
			catch (Exception e)
			{
				Log.LogWarning($"[BD2OldClientCompatPatch] Could not close GetValueObject<{targetType.Name}>: {e.Message}");
				return false;
			}
			try
			{
				harmony.Patch(closed, postfix: new HarmonyMethod(typeof(GetValueObjectPatch), nameof(GetValueObjectPatch.Postfix)));
				Log.LogInfo($"[BD2OldClientCompatPatch] Patched GetValueObject<{targetType.Name}> - closed generic instantiation.");
				return true;
			}
			catch (Exception e)
			{
				Log.LogWarning($"[BD2OldClientCompatPatch] Patching GetValueObject<{targetType.Name}> failed, skipping: {e.Message}");
				return false;
			}
		}

		private static void PatchGetValueObject(Harmony harmony)
		{
			MethodInfo openGeneric = GetValueObjectOpenGeneric();
			if (openGeneric == null)
			{
				Log.LogWarning("[BD2OldClientCompatPatch] Could not find RawDataManager.GetValueObject<T>(_, string, int).");
				return;
			}

			// Patching the OPEN generic definition directly throws (confirmed live,
			// 2026-10-02): MonoMod's reflection importer can't import an unresolved
			// generic parameter ("Specified method is not supported" from
			// MMReflectionImporter.ImportGenericParameter) - a real MonoMod limitation,
			// not something fixable from this side. But a CLOSED generic instantiation
			// (T already resolved to a concrete type) has a fully concrete signature,
			// so patch one closed instantiation per table we have fallback data for,
			// instead of trying to patch the shared open definition once.
			foreach (Type targetType in TableFallbacks.AllTypes())
			{
				if (_getValueObjectPatchedTypes.Add(targetType))
				{
					PatchOneClosedGetValueObject(openGeneric, targetType, harmony);
				}
			}
		}

		// The real per-character processor method is confirmed (live stack trace,
		// 2026-10-02) to be a static (CharDBInfo, <enum>) method - several methods
		// share that outer shape, so it's disambiguated by checking whose IL actually
		// reaches a CharTable-by-id call. From that SAME verified processor, every
		// OTHER missing table's lookup is also reachable, just at varying call depth
		// (CostumeNodeGroupTable was a direct call; a flat "scan the whole assembly for
		// a matching shape" approach found a same-shaped DECOY method for CostumeTable
		// that wasn't actually on the real call path and never fires - confirmed live
		// when its fallback never triggered). So: search recursively through the
		// processor's own call graph (same-module calls only) for each target type,
		// rather than a flat whole-assembly shape match - anchoring every search in
		// code that's actually proven to run, not just structurally similar.
		private static void PatchAllTableLookupsOnAccessorClass(Harmony harmony)
		{
			using ModuleDefinition cecilModule = ModuleDefinition.ReadModule(typeof(CharDBInfo).Assembly.Location);

			MethodDefinition processorCecil = null;
			MethodInfo processorRuntime = null;
			foreach (MethodInfo candidate in FindCharDbInfoProcessorCandidates())
			{
				MethodDefinition candidateCecil = ResolveCecil(candidate, cecilModule);
				if (candidateCecil == null)
				{
					continue;
				}
				MethodReference probe = FindTableByIdCallByTypeName(candidateCecil, "CharTable", new HashSet<string>(), 5);
				if (probe != null)
				{
					processorCecil = candidateCecil;
					processorRuntime = candidate;
					Log.LogInfo($"[BD2OldClientCompatPatch] Processor: {candidate.DeclaringType.FullName}.{candidate.Name}");
					break;
				}
			}
			if (processorCecil == null)
			{
				Log.LogWarning("[BD2OldClientCompatPatch] Could not locate the real CharDBInfo processor.");
				return;
			}

			// Real bug found live, 2026-10-02: even with CharTable/CostumeTable/
			// CharLevelTable all correctly resolved (not null), this SAME processor
			// still throws a genuine NullReferenceException deeper inside its own
			// stat-calculation logic (route "/Game/AllCharRefresh", confirmed via
			// Player.log's own CrashReporter capture - NOT a DataNotFoundException,
			// a different bug entirely, something else in that call chain is null).
			// Same proven fix already used throughout this project's OTHER compat
			// patch (BD2CompatPatch, for the current client): wrap the whole
			// processor with a Harmony FINALIZER that swallows any exception, so a
			// bug deeper in this stat-calc logic degrades to "this one char's
			// refresh silently did nothing" instead of stalling the entire
			// AllCharRefresh flow (and the login sequence depending on it) forever.
			try
			{
				harmony.Patch(processorRuntime, finalizer: new HarmonyMethod(typeof(SwallowExceptionPatch), nameof(SwallowExceptionPatch.Finalizer)));
				Log.LogInfo($"[BD2OldClientCompatPatch] Wrapped processor {processorRuntime.DeclaringType.FullName}.{processorRuntime.Name} with an exception-swallowing finalizer.");
			}
			catch (Exception e)
			{
				Log.LogWarning($"[BD2OldClientCompatPatch] Could not attach finalizer to processor: {e.Message}");
			}

			foreach (Type targetType in TableFallbacks.AllTypes())
			{
				MethodReference foundRef = FindTableByIdCallByTypeName(processorCecil, targetType.Name, new HashSet<string>(), 5);
				if (foundRef == null)
				{
					Log.LogWarning($"[BD2OldClientCompatPatch] No (int)->{targetType.Name} call reachable from the processor.");
					continue;
				}
				MethodInfo anchor;
				try
				{
					anchor = typeof(CharDBInfo).Assembly.ManifestModule.ResolveMethod(foundRef.MetadataToken.ToInt32()) as MethodInfo;
				}
				catch (Exception e)
				{
					Log.LogWarning($"[BD2OldClientCompatPatch] Failed to resolve {targetType.Name} lookup: {e.Message}");
					continue;
				}
				if (anchor == null)
				{
					Log.LogWarning($"[BD2OldClientCompatPatch] Resolved token for {targetType.Name} wasn't a MethodInfo.");
					continue;
				}

				// A table can have more than one lookup method on its accessor class
				// (confirmed live, 2026-10-02: CostumeTable has at least 2, reached from
				// two entirely separate caller chains - patching only the one call site
				// the recursive search happened to find left the other one still
				// crashing). Patch every (int)->targetType method on the SAME class the
				// recursive search landed on, not just the one specific call site.
				int patchedForType = 0;
				foreach (MethodInfo m in anchor.DeclaringType.GetMethods(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Static | BindingFlags.DeclaredOnly))
				{
					ParameterInfo[] p = m.GetParameters();
					if (m.ReturnType != targetType || p.Length != 1 || p[0].ParameterType != typeof(int))
					{
						continue;
					}
					try
					{
						harmony.Patch(m, postfix: new HarmonyMethod(typeof(GenericTableLookupPatch), nameof(GenericTableLookupPatch.Postfix)));
						patchedForType++;
						Log.LogInfo($"[BD2OldClientCompatPatch] Patched {m.DeclaringType.FullName}.{m.Name} ({targetType.Name}).");
					}
					catch (Exception e)
					{
						Log.LogWarning($"[BD2OldClientCompatPatch] Failed to patch {m.Name}: {e.Message}");
					}
				}
				Log.LogInfo($"[BD2OldClientCompatPatch] {patchedForType} lookup method(s) patched for {targetType.Name}.");
			}
		}

		// Matches by name AND each parameter's actual type name, not just parameter
		// COUNT - a count-only match is ambiguous for overloaded methods (confirmed
		// live, 2026-10-02: a 3-overload method with (long)/(CharDBInfo)/(int) all
		// have exactly 1 parameter, so a count-only match could silently resolve to
		// the wrong overload's body and search it instead of the one actually on the
		// live call path).
		internal static MethodDefinition ResolveCecil(MethodInfo runtime, ModuleDefinition module)
		{
			TypeDefinition type = module.GetType(runtime.DeclaringType.FullName);
			if (type == null)
			{
				return null;
			}
			ParameterInfo[] runtimeParams = runtime.GetParameters();
			return type.Methods.FirstOrDefault(m =>
				m.Name == runtime.Name
				&& m.Parameters.Count == runtimeParams.Length
				&& m.Parameters.Select(p => p.ParameterType.Name).SequenceEqual(runtimeParams.Select(p => p.ParameterType.Name)));
		}

		// Recursively searches a method's IL (and, up to `depth` hops, same-module
		// methods it calls) for a call/callvirt resolving to a method matching
		// "(int) -> a type named targetReturnTypeName". Matches by short Name (not
		// FullName/namespace) so this is directly usable both at startup (where the
		// full Type is known) and from the live DataNotFoundException diagnostic
		// (where only the bare table-name string, e.g. "CostumeTable", is available).
		// `visited` prevents revisiting a method already on the current search path
		// (cycle guard). Uses Mono.Cecil (shipped in BepInEx/core for exactly this
		// kind of plugin-side IL inspection, same library this repo's
		// tools/RoutePatcher uses) for a correct instruction walk, rather than
		// hand-rolling an IL opcode/operand-size table - a one-byte mistake there
		// would silently misalign every instruction after it.
		internal static bool VerboseLog = false;

		internal static MethodReference FindTableByIdCallByTypeName(MethodDefinition method, string targetReturnTypeName, HashSet<string> visited, int depth)
		{
			if (method == null || !method.HasBody || depth < 0)
			{
				return null;
			}
			string key = method.FullName;
			if (!visited.Add(key))
			{
				return null;
			}

			if (VerboseLog)
			{
				Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] [diag] searching {method.FullName} (depth {depth})");
			}
			foreach (Instruction instr in method.Body.Instructions)
			{
				if (instr.OpCode != OpCodes.Call && instr.OpCode != OpCodes.Callvirt)
				{
					continue;
				}
				if (instr.Operand is not MethodReference methodRef)
				{
					continue;
				}
				if (VerboseLog)
				{
					Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] [diag]   call -> {methodRef.FullName} (returns {methodRef.ReturnType.Name})");
				}
				if (methodRef.ReturnType.Name == targetReturnTypeName
					&& methodRef.Parameters.Count == 1 && methodRef.Parameters[0].ParameterType.FullName == "System.Int32")
				{
					return methodRef;
				}

				if (depth == 0)
				{
					continue;
				}
				MethodDefinition resolved;
				try
				{
					resolved = methodRef.Resolve();
					if (VerboseLog && resolved == null)
					{
						Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] [diag]   resolve returned null for {methodRef.FullName}");
					}
				}
				catch (Exception e)
				{
					resolved = null;
					if (VerboseLog)
					{
						Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] [diag]   resolve threw for {methodRef.FullName}: {e.Message}");
					}
				}
				if (resolved != null && resolved.Module != method.Module && VerboseLog)
				{
					Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] [diag]   skipping {methodRef.FullName}: different module ({resolved.Module.Name} vs {method.Module.Name})");
				}
				if (resolved != null && resolved.Module == method.Module)
				{
					MethodReference found = FindTableByIdCallByTypeName(resolved, targetReturnTypeName, visited, depth - 1);
					if (found != null)
					{
						return found;
					}
				}
			}
			return null;
		}

		// Several methods share the outer (CharDBInfo, enum) shape (confirmed live,
		// 2026-10-02 - the first match by that shape alone was the wrong method).
		// Return every candidate; the caller disambiguates by checking which one's
		// own IL actually calls the CharTable-by-id lookup.
		private static IEnumerable<MethodInfo> FindCharDbInfoProcessorCandidates()
		{
			foreach (Type type in typeof(CharDBInfo).Assembly.GetTypes())
			{
				MethodInfo[] methods;
				try
				{
					methods = type.GetMethods(BindingFlags.Static | BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.DeclaredOnly);
				}
				catch
				{
					continue;
				}
				foreach (MethodInfo m in methods)
				{
					ParameterInfo[] p = m.GetParameters();
					if (p.Length == 2 && p[0].ParameterType == typeof(CharDBInfo) && p[1].ParameterType.IsEnum)
					{
						yield return m;
					}
				}
			}
		}

		// Uses Mono.Cecil (already shipped in BepInEx/core for exactly this kind of
		// plugin-side IL inspection, same library this repo's tools/RoutePatcher uses)
		// for a correct instruction walk, rather than hand-rolling an IL opcode/operand
	}

	// Self-healing: tracing each new table gap by hand (find the real caller, type its
	// obfuscated name correctly in a separate tool call, decompile, patch, redeploy,
	// retest) kept hitting friction from exactly the kind of Unicode-identifier
	// round-tripping this whole plugin otherwise avoids - typing a freshly-seen
	// obfuscated name into a shell command is a different path than referencing it in
	// already-decompiled source, and re-typing it has repeatedly corrupted the
	// identifier. So: whenever this fires for a table we have TableFallbacks data for,
	// walk the LIVE stack frames directly (no identifier typing at all) and patch
	// every (int)->ThatTable frame found, idempotently. The exception that triggered
	// this has already returned null for the current attempt, but the old client
	// auto-retries its whole login/batch flow every few minutes (confirmed live,
	// 2026-10-02 - that's what its "Disconnected from server, Restarting..." loop
	// actually is) - so patching here lets the NEXT retry self-heal without a redeploy.
	// Harmony finalizer: if __exception is non-null, log it (so it's still visible
	// for diagnosis) and return null to swallow it instead of letting it propagate.
	// Matches the exact technique BD2CompatPatch (this project's current-client
	// compat patch) already uses successfully for this same class of bug.
	// Companion to DataNotFoundExceptionDiagnostic, for the (string, int, int,
	// string) constructor overload - composite-key lookups like CharLevelTable's
	// (CharGrowthId, Level). No Cecil IL-walk needed here: a direct whole-assembly
	// scan for "(int,int) -> a type named __0" is cheap enough and more reliable
	// than re-deriving a stack-frame anchor for every new composite-key table.
	internal static class CompositeKeyDataNotFoundDiagnostic
	{
		internal static Harmony HarmonyInstance;
		private static readonly HashSet<MethodInfo> AlreadyPatched = new();

		internal static void Prefix(string __0, int __1, int __2)
		{
			Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] Composite DataNotFoundException({__0}, {__1}, {__2})");
			if (HarmonyInstance == null)
			{
				return;
			}
			foreach (Type type in typeof(CharDBInfo).Assembly.GetTypes())
			{
				MethodInfo[] methods;
				try
				{
					methods = type.GetMethods(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Static | BindingFlags.DeclaredOnly);
				}
				catch
				{
					continue;
				}
				foreach (MethodInfo m in methods)
				{
					if (m.ReturnType.Name != __0)
					{
						continue;
					}
					ParameterInfo[] p = m.GetParameters();
					if (p.Length != 2 || p[0].ParameterType != typeof(int) || p[1].ParameterType != typeof(int))
					{
						continue;
					}
					if (!AlreadyPatched.Add(m))
					{
						continue;
					}
					try
					{
						HarmonyInstance.Patch(m, postfix: new HarmonyMethod(typeof(CompositeKeyLookupPatch), nameof(CompositeKeyLookupPatch.Postfix)));
						Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] Composite-key self-heal: patched {m.DeclaringType.FullName}.{m.Name} ({__0}).");
					}
					catch (Exception e)
					{
						Plugin.Log.LogWarning($"[BD2OldClientCompatPatch] Composite-key patch failed for {m.Name}: {e.Message}");
					}
				}
			}
		}
	}

	// Default-constructs an empty instance whenever the composite-key lookup
	// comes back null - same "legitimate empty row, not fabricated data"
	// reasoning as the id-0 sentinel case, just for a 2-int key instead of 1.
	internal static class CompositeKeyLookupPatch
	{
		internal static void Postfix(MethodBase __originalMethod, ref object __result)
		{
			if (__result != null || __originalMethod is not MethodInfo mi)
			{
				return;
			}
			try
			{
				__result = Activator.CreateInstance(mi.ReturnType);
				Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] {mi.ReturnType.Name}: composite-key miss, returned a default-constructed instance.");
			}
			catch (Exception e)
			{
				Plugin.Log.LogWarning($"[BD2OldClientCompatPatch] Could not default-construct {mi.ReturnType.Name}: {e.Message}");
			}
		}
	}

	internal static class GenericExceptionSelfHeal
	{
		internal static Harmony HarmonyInstance;
		private static readonly HashSet<MethodBase> AlreadyWrapped = new();

		internal static void Prefix(Exception exception)
		{
			if (exception == null || HarmonyInstance == null)
			{
				return;
			}
			System.Diagnostics.StackTrace trace;
			try
			{
				trace = new System.Diagnostics.StackTrace(exception, true);
			}
			catch
			{
				return;
			}
			System.Diagnostics.StackFrame[] frames = trace.GetFrames();
			if (frames == null)
			{
				return;
			}
			foreach (System.Diagnostics.StackFrame frame in frames)
			{
				if (frame.GetMethod() is not MethodInfo mi)
				{
					continue;
				}
				// Only wrap our own game's code (Assembly-CSharp), never BCL/Unity
				// internals - those aren't safe or meaningful to finalizer-wrap, and
				// most won't even be patchable.
				if (mi.Module.Assembly != typeof(CharDBInfo).Assembly)
				{
					continue;
				}
				if (mi.IsGenericMethodDefinition || mi.ContainsGenericParameters)
				{
					continue; // same MonoMod limitation as the open GetValueObject<T> case
				}
				if (!AlreadyWrapped.Add(mi))
				{
					continue;
				}
				try
				{
					HarmonyInstance.Patch(mi, finalizer: new HarmonyMethod(typeof(SwallowExceptionPatch), nameof(SwallowExceptionPatch.Finalizer)));
					Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] Generic self-heal: wrapped {mi.DeclaringType?.FullName}.{mi.Name} after seeing it throw live.");
				}
				catch (Exception e)
				{
					Plugin.Log.LogWarning($"[BD2OldClientCompatPatch] Generic self-heal could not wrap {mi.Name}: {e.Message}");
				}
			}
		}
	}

	internal static class SwallowExceptionPatch
	{
		internal static Exception Finalizer(Exception __exception)
		{
			if (__exception != null)
			{
				Plugin.Log.LogWarning($"[BD2OldClientCompatPatch] Swallowed exception in processor: {__exception}");
			}
			return null;
		}
	}

	// Skips the field-entry entry point entirely rather than letting it run
	// and catching the fallout - see PatchSkipFieldEntry for why.
	internal static class SkipFieldEntryPatch
	{
		internal static bool Prefix()
		{
			Plugin.Log.LogInfo("[BD2OldClientCompatPatch] Skipped field-entry attempt (no field scenes exist in this client build).");
			return false;
		}
	}

	// Rewrites any Addressables scene key shaped like a (nonexistent) field
	// map - "<PackId>_Map/Scenes/<name>.unity" - to the real, bundled home
	// scene key "Scenes/MyRoom" before the original load runs. See
	// PatchRedirectBrokenFieldScene for the full reasoning. Harmony passes
	// the original "object key" parameter by name here, as a ref, so it can
	// be rewritten before the real method sees it.
	internal static class RedirectBrokenFieldScenePatch
	{
		internal static void Prefix(ref object key)
		{
			if (key is string s && s.Contains("/Scenes/") && s.EndsWith(".unity"))
			{
				Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] Redirecting broken field scene key \"{s}\" -> \"Scenes/MyRoom\".");
				key = "Scenes/MyRoom";
			}
		}
	}

	// Same exception-swallowing idea as SwallowExceptionPatch, but for a method
	// whose return value callers use unconditionally (e.g. iterate without a
	// null check) - a bare swallow would leave __result at null and just trade
	// one crash for an immediate new one. Forces a real empty instance of the
	// method's own declared return type instead (works for List<T> and any
	// other type with a parameterless constructor).
	internal static class SafeEmptyResultPatch
	{
		internal static readonly Dictionary<MethodBase, Type> ResultTypes = new();

		internal static Exception Finalizer(MethodBase __originalMethod, Exception __exception, ref object __result)
		{
			if (__exception == null)
			{
				return null;
			}
			Plugin.Log.LogWarning($"[BD2OldClientCompatPatch] Swallowed exception (safe-empty-result): {__exception}");
			if (ResultTypes.TryGetValue(__originalMethod, out Type returnType))
			{
				try
				{
					__result = Activator.CreateInstance(returnType);
				}
				catch
				{
					// Leave __result as-is if the return type has no parameterless ctor.
				}
			}
			return null;
		}
	}

	internal static class DataNotFoundExceptionDiagnostic
	{
		internal static Harmony HarmonyInstance;
		private static readonly HashSet<MethodBase> AutoPatched = new();

		internal static void Prefix(string __0, int __1)
		{
			string trace = new System.Diagnostics.StackTrace(true).ToString();
			Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] DataNotFoundException({__0}, id:{__1}) real call stack:\n{trace}");

			// Proceed either when we have specific fallback DATA for this table (the
			// 3-character case), or when the id is the universal "nothing selected"
			// sentinel (confirmed live, 2026-10-02, now seen for CharTable, CostumeTable,
			// and CharLevelTable - a generic client bug pattern, not table-specific) -
			// that case needs no per-table data, GenericTableLookupPatch's postfix
			// default-constructs an empty instance for id 0 regardless of type.
			if ((!TableFallbacks.HasAnyForName(__0) && __1 != 0) || HarmonyInstance == null)
			{
				return;
			}

			// Try the simple, robust path FIRST: GetValueObject<T> is the universal,
			// shared primitive (confirmed live, 2026-10-02 - patching its closed
			// generic instantiation fixed both of CostumeTable's separate failing call
			// paths at once, something the frame-by-frame IL search below could never
			// find since one of those paths went through generic cache indirection
			// Cecil can't see concrete types through). We already know the table NAME
			// (__0) here - resolve it to a real Type directly and patch that one closed
			// generic, with no stack-walking or IL search needed at all.
			if (Plugin.TryPatchGetValueObjectFor(__0, HarmonyInstance))
			{
				return;
			}

			// The failing lookup itself has already returned null and popped off the
			// stack by the time this constructor runs (confirmed live, 2026-10-02 -
			// only its CALLER is still visible here). So: for each live frame, treat it
			// as a candidate ANCHOR and run the same Cecil-based recursive IL search
			// used at startup (works on static IL, not live stack depth, so it finds
			// the real inner call regardless of whether it's still on the stack) for a
			// call matching (int)->[a type named __0]. This also covers the case where
			// the live frame itself IS the direct match (depth-0 search catches that).
			using ModuleDefinition cecilModule = ModuleDefinition.ReadModule(typeof(CharDBInfo).Assembly.Location);
			System.Diagnostics.StackFrame[] frames = new System.Diagnostics.StackTrace(true).GetFrames();
			if (frames == null)
			{
				return;
			}
			int frameNum = 0;
			foreach (System.Diagnostics.StackFrame frame in frames)
			{
				frameNum++;
				if (frame.GetMethod() is not MethodInfo anchorMi)
				{
					continue;
				}
				MethodDefinition anchorCecil = Plugin.ResolveCecil(anchorMi, cecilModule);
				Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] [diag] frame {frameNum}: {anchorMi.DeclaringType.FullName}.{anchorMi.Name} ; cecilMethod={(anchorCecil != null)}");
				if (anchorCecil == null)
				{
					continue;
				}
				Plugin.VerboseLog = true;
				MethodReference foundRef = Plugin.FindTableByIdCallByTypeName(anchorCecil, __0, new HashSet<string>(), 4);
				Plugin.VerboseLog = false;
				Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] [diag] frame {frameNum} search result: {(foundRef != null ? foundRef.FullName : "null")}");
				if (foundRef == null)
				{
					continue;
				}
				MethodBase resolved;
				try
				{
					resolved = typeof(CharDBInfo).Assembly.ManifestModule.ResolveMethod(foundRef.MetadataToken.ToInt32());
				}
				catch
				{
					continue;
				}
				if (resolved is not MethodInfo mi || !AutoPatched.Add(mi))
				{
					continue;
				}
				try
				{
					HarmonyInstance.Patch(mi, postfix: new HarmonyMethod(typeof(GenericTableLookupPatch), nameof(GenericTableLookupPatch.Postfix)));
					Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] Self-healed: auto-patched {mi.DeclaringType.FullName}.{mi.Name} ({__0}), reached from live frame {anchorMi.DeclaringType.FullName}.{anchorMi.Name}.");

					// Also patch every sibling (int)->__0 method on the same class,
					// same reasoning as the startup pass: a table can have more than
					// one lookup overload (confirmed live for CostumeTable itself).
					foreach (MethodInfo sibling in mi.DeclaringType.GetMethods(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Static | BindingFlags.DeclaredOnly))
					{
						ParameterInfo[] sp = sibling.GetParameters();
						if (sibling.ReturnType.Name == __0 && sp.Length == 1 && sp[0].ParameterType == typeof(int) && AutoPatched.Add(sibling))
						{
							HarmonyInstance.Patch(sibling, postfix: new HarmonyMethod(typeof(GenericTableLookupPatch), nameof(GenericTableLookupPatch.Postfix)));
							Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] Self-healed sibling: {sibling.DeclaringType.FullName}.{sibling.Name} ({__0}).");
						}
					}
				}
				catch (Exception e)
				{
					Plugin.Log.LogWarning($"[BD2OldClientCompatPatch] Self-heal patch failed for {mi.Name}: {e.Message}");
				}
				return; // found and patched via this anchor, no need to keep walking frames
			}
		}
	}

	// Shared generic postfix for every patched (int)->T lookup method. __result is
	// typed object here (not T) - Harmony supports this for reference-type returns,
	// and it's how one postfix can serve many different patched methods/return types.
	internal static class GenericTableLookupPatch
	{
		internal static void Postfix(MethodBase __originalMethod, int __0, ref object __result)
		{
			if (__result != null || __originalMethod is not MethodInfo mi)
			{
				return;
			}
			object fallback = TableFallbacks.Get(mi.ReturnType, __0);
			if (fallback != null)
			{
				__result = fallback;
				Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] Injected fallback {mi.ReturnType.Name} row for id={__0}");
				return;
			}
			FallbackHelpers.TryDefaultForZero(mi.ReturnType, __0, ref __result);
		}
	}

	// Shared between GenericTableLookupPatch and GetValueObjectPatch - both need the
	// same "id 0 is a sentinel, not a real missing row" handling.
	internal static class FallbackHelpers
	{
		internal static void TryDefaultForZero(Type returnType, int id, ref object result)
		{
			if (id != 0)
			{
				return;
			}
			try
			{
				result = Activator.CreateInstance(returnType);
				Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] {returnType.Name}: id 0 sentinel, returned a default-constructed instance.");
			}
			catch (Exception e)
			{
				Plugin.Log.LogWarning($"[BD2OldClientCompatPatch] Could not default-construct {returnType.Name}: {e.Message}");
			}
		}
	}

	// Postfix for the universal RawDataManager.GetValueObject<T>(DatabaseType,
	// string tableName, int key) primitive. __1 is tableName, __2 is key (dbType is
	// __0). __originalMethod reflects the actual CLOSED generic instantiation being
	// invoked (confirmed live, 2026-10-02 - Harmony resolves T per call site even
	// though the patch is attached to the open generic method definition), so
	// __originalMethod.ReturnType is the real concrete type (e.g. CostumeTable), not
	// the unresolved "T" placeholder.
	internal static class GetValueObjectPatch
	{
		internal static void Postfix(MethodBase __originalMethod, string __1, int __2, ref object __result)
		{
			if (__result != null || __originalMethod is not MethodInfo mi)
			{
				return;
			}
			object fallback = TableFallbacks.Get(mi.ReturnType, __2);
			if (fallback != null)
			{
				__result = fallback;
				Plugin.Log.LogInfo($"[BD2OldClientCompatPatch] GetValueObject<{mi.ReturnType.Name}>: injected fallback for table={__1} id={__2}");
				return;
			}

			// id 0 is a "nothing selected" sentinel in this codebase, not a real row
			// (confirmed live, 2026-10-02: hit for CharTable id:0 from an unrelated UI
			// element, not any of the 3 characters this plugin adds) - an unconditional
			// lookup without a zero-guard is the same "log-then-dereference-anyway" bug
			// pattern already documented for the current client's own compat patch.
			FallbackHelpers.TryDefaultForZero(mi.ReturnType, __2, ref __result);
		}
	}

	// Extensible fallback-data registry. Add one entry here per (table type, id) the
	// old client turns out to be missing - no new patch code needed, as long as the
	// lookup method lives on the same accessor class already being patched.
	// Field values are sourced from BD2PS-main's own current-client capture
	// (httpserver/data/tables/*.json) - the same source already used server-side for
	// these same 3 characters (202/203/204). Only growthgrade 1 is modeled for
	// CharTable - these characters' only captured growth stage.
	internal static class TableFallbacks
	{
		private static readonly Dictionary<(Type, int), object> Data = new();

		static TableFallbacks()
		{
			Add(2020, new CharTable
			{
				CharGrowthId = 101, CharNameTextId = 102021, CriticalChanceValue = 0.1,
				CriticalDamageRateValue = 0.5, DefaultCostumeId = 20201, Element = 2,
				ElementDefenseValue = 0.5, ElementPowerValue = 0.5, Grade = 5, Growthgrade = 1,
				HealthValue = 107.0, Id = 2020, NextCharId = 2021, PhysicalPowerValue = 31.0,
				UniqueCharId = 202,
			});
			Add(2030, new CharTable
			{
				CharGrowthId = 101, CharNameTextId = 102031, CriticalChanceValue = 0.05,
				CriticalDamageRateValue = 0.5, DefaultCostumeId = 20301, Element = 2,
				ElementDefenseValue = 0.5, ElementPowerValue = 0.5, Grade = 5, Growthgrade = 1,
				HealthValue = 88.0, Id = 2030, MagicPowerValue = 34.0, NextCharId = 2031,
				PhysicalDefenseValue = 0.1, UniqueCharId = 203,
			});
			Add(2040, new CharTable
			{
				CharGrowthId = 101, CharNameTextId = 102041, CriticalChanceValue = 0.1,
				CriticalDamageRateValue = 1.0, DefaultCostumeId = 20401, ElementDefenseValue = 0.5,
				ElementPowerValue = 0.5, Grade = 5, Growthgrade = 1, HealthValue = 96.0, Id = 2040,
				MagicDefenseValue = 0.15, MagicPowerValue = 30.0, NextCharId = 2041,
				UniqueCharId = 204,
			});

			// httpserver/data/tables/CostumeNodeGroupTable.json: real rows, charUniqueId
			// matches the char each belongs to.
			Add(20201, new CostumeNodeGroupTable { Id = 20201, IsActive = 1, NodeUIPrefab = "NodeType1", CharUniqueId = 202 });
			Add(20301, new CostumeNodeGroupTable { Id = 20301, IsActive = 1, NodeUIPrefab = "NodeType3", CharUniqueId = 203 });
			Add(20401, new CostumeNodeGroupTable { Id = 20401, IsActive = 1, NodeUIPrefab = "NodeType3", CharUniqueId = 204 });

			// httpserver/data/tables/CostumeTable.json: real rows, same source already
			// added to the old server's own server/data/CostumeTable.txt earlier today.
			Add(20201, new CostumeTable
			{
				AttackRangeCount = 1, AttackType = 1, CostumeDescNameTextId = 100000, CostumeDesignId = 20201,
				CostumeDialog = 29001, CostumeNameTextId = 202021, GrowthGroupId = 20201, Id = 20201,
				JapaneseCharacterVoiceActorNameTextId = 21049, KoreanCharacterVoiceActorNameTextId = 20047,
				MaxLevel = 5, NorSubAttackBuffId = 2005, NorSubAttackDescSkillTextId = 2005,
				NorSubAttackNameSkillTextId = 102, NormalAttackDescSkillTextId = 201, NormalAttackNameSkillTextId = 101,
				NotTrash = 1, SkillGroupId = 20201, SpAttackAddCount = 1, UseRoguelike = 2, UseUniqueCharId = 202,
			});
			Add(20301, new CostumeTable
			{
				AttackRangeCount = 1, AttackType = 1, CostumeDescNameTextId = 100000, CostumeDesignId = 20301,
				CostumeDialog = 29001, CostumeNameTextId = 202031, GrowthGroupId = 20301, Id = 20301,
				JapaneseCharacterVoiceActorNameTextId = 21050, KoreanCharacterVoiceActorNameTextId = 20048,
				MaxLevel = 5, NorSubAttackBuffId = 2101, NorSubAttackDescSkillTextId = 2101,
				NorSubAttackNameSkillTextId = 102, NormalAttackDescSkillTextId = 202, NormalAttackNameSkillTextId = 101,
				NotTrash = 1, SkillGroupId = 20301, SpAttackAddCount = 1, UseRoguelike = 2, UseUniqueCharId = 203,
			});
			Add(20401, new CostumeTable
			{
				AttackMoveType = 1, AttackRangeCount = 1, AttackType = 1, CostumeDescNameTextId = 100000,
				CostumeDesignId = 20401, CostumeDialog = 29001, CostumeNameTextId = 202041, GrowthGroupId = 20401,
				Id = 20401, JapaneseCharacterVoiceActorNameTextId = 21051, KoreanCharacterVoiceActorNameTextId = 20049,
				MaxLevel = 5, NorSubAttackBuffId = 2106, NorSubAttackDescSkillTextId = 2106,
				NorSubAttackNameSkillTextId = 102, NormalAttackDescSkillTextId = 202, NormalAttackNameSkillTextId = 101,
				NotTrash = 1, SkillGroupId = 20401, SpAttackAddCount = 1, UseRoguelike = 2, UseUniqueCharId = 204,
			});
		}

		private static void Add(int id, object row)
		{
			Data[(row.GetType(), id)] = row;
		}

		internal static bool HasAnyFor(Type type) => Data.Keys.Any(k => k.Item1 == type);

		internal static bool HasAnyForName(string typeName) => Data.Keys.Any(k => k.Item1.Name == typeName);

		internal static IEnumerable<Type> AllTypes() => Data.Keys.Select(k => k.Item1).Distinct();

		internal static object Get(Type type, int id) => Data.TryGetValue((type, id), out object v) ? v : null;
	}
}
