using System;
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

			PatchGetValueObject(harmony);
			PatchAllTableLookupsOnAccessorClass(harmony);

			Log.LogInfo("[BD2OldClientCompatPatch] Loaded.");
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
					Log.LogInfo($"[BD2OldClientCompatPatch] Processor: {candidate.DeclaringType.FullName}.{candidate.Name}");
					break;
				}
			}
			if (processorCecil == null)
			{
				Log.LogWarning("[BD2OldClientCompatPatch] Could not locate the real CharDBInfo processor.");
				return;
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
