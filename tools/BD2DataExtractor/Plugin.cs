using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Threading;
using BepInEx;
using BepInEx.Logging;
using HarmonyLib;
using Google.Protobuf;
using UnityEngine;

namespace BD2DataExtractor
{
    [BepInPlugin("bd2.dataextractor", "BD2 Data Extractor", "1.0.0")]
    public class Plugin : BaseUnityPlugin
    {
        internal static ManualLogSource Log;

        // table type name -> (dedupe key -> parsed row)
        private static readonly Dictionary<string, Dictionary<string, IMessage>> Captured =
            new Dictionary<string, Dictionary<string, IMessage>>();

        private static readonly object CaptureLock = new object();
        private static Harmony _harmony;
        private static string _outDir;
        private static Timer _autoDumpTimer;
        private static Timer _forceLoadTimer;
        private static bool _forceLoadDone;

        // All known "db" identifiers as of the 20260918000 client — enumerated from the
        // decompiled Proto.Design.* namespace list (see CLIENT_UPDATE.md). DB_PACK needs an
        // explicit id per call (GetDBName defaults to the *currently selected* pack otherwise).
        private static readonly int[] PackIds =
        {
            1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,
            1001,1002,1003,1004,1005,1006,1007,1008,1009,1010,
            2001,2002,2003,2004,2005,2006,2007,2008,2009,2010,
            3001,3002,3003,3004,3005,3006,3007,3008,3009,3010,3011,3012,
            10001,10004,10006,
            11001,11002,11003,11004,11005,11006,11007,11008,11009,11010,
            12001,12002,12003,12004,12005,12006,12007,12008,12009,12010,
            13007, 20000
        };

        private void Awake()
        {
            Log = Logger;
            _outDir = Path.Combine(Paths.GameRootPath, "ExtractedTables");
            Directory.CreateDirectory(_outDir);

            _harmony = new Harmony("bd2.dataextractor");
            int patched = PatchAllDesignTables();
            Log.LogInfo($"[BD2DataExtractor] Patched {patched} Proto.Design message types. " +
                        "Auto-dump runs every 30s (pure .NET timer, no Input/Update dependency) and once more on quit.");

            // Plain .NET timer instead of Unity's Update()/Input — avoids any dependency on
            // which Input System backend the project is configured to use.
            _autoDumpTimer = new Timer(_ => DumpAll("auto"), null, 30_000, 30_000);

            // Retries every 15s (RawDataManager may not exist until after login) until one
            // attempt goes through, then stops. Forces every known "db" to load via the
            // client's own RawDataManager.DBLoad, instead of relying on manual navigation.
            _forceLoadTimer = new Timer(_ => TryForceLoadAllDatabases(), null, 5_000, 15_000);
        }

        private static void TryForceLoadAllDatabases()
        {
            if (_forceLoadDone) return;
            try
            {
                var rdmType = AccessTools.TypeByName("RawDataManager");
                if (rdmType == null)
                {
                    Log.LogWarning("[BD2DataExtractor] RawDataManager type not found.");
                    _forceLoadDone = true;
                    return;
                }

                var getDbName = rdmType.GetMethod("GetDBName", BindingFlags.Public | BindingFlags.Instance);
                var dbLoad = rdmType.GetMethods(BindingFlags.Public | BindingFlags.Instance)
                    .FirstOrDefault(m => m.Name == "DBLoad" && m.GetParameters().Length == 2);
                if (getDbName == null || dbLoad == null)
                {
                    Log.LogWarning("[BD2DataExtractor] GetDBName/DBLoad method not found on RawDataManager.");
                    _forceLoadDone = true;
                    return;
                }

                // Locate the Singleton<RawDataManager> instance accessor by signature (static,
                // returns RawDataManager, no params) rather than by its obfuscated name.
                var singletonClosedType = typeof(gamfs.Singleton<>).MakeGenericType(rdmType);
                var instanceProp = singletonClosedType
                    .GetProperties(BindingFlags.Public | BindingFlags.Static)
                    .FirstOrDefault(p => p.PropertyType == rdmType && p.GetIndexParameters().Length == 0);
                var instance = instanceProp?.GetValue(null);
                if (instance == null)
                {
                    Log.LogInfo("[BD2DataExtractor] RawDataManager not ready yet, will retry.");
                    return;
                }

                var enumType = getDbName.GetParameters()[0].ParameterType;

                void LoadDb(string enumName, int index = 0)
                {
                    try
                    {
                        object enumVal = Enum.Parse(enumType, enumName);
                        object dbNameStruct = getDbName.Invoke(instance, new object[] { enumVal, index });
                        dbLoad.Invoke(instance, new object[] { dbNameStruct, null });
                    }
                    catch (Exception ex)
                    {
                        Log.LogWarning($"[BD2DataExtractor] LoadDb({enumName},{index}) failed: {ex.Message}");
                    }
                }

                LoadDb("DB_COMMON");
                LoadDb("DB_BLOCK");
                LoadDb("DB_FILED_OBJECT_SCENE"); // sic — typo preserved from the game's own enum
                LoadDb("DB_INTRO");
                foreach (var id in PackIds) LoadDb("DB_PACK", id);

                Log.LogInfo($"[BD2DataExtractor] Force-load requested for {4 + PackIds.Length} databases.");
                _forceLoadDone = true;
            }
            catch (Exception ex)
            {
                Log.LogWarning($"[BD2DataExtractor] TryForceLoadAllDatabases failed: {ex}");
            }
        }

        private int PatchAllDesignTables()
        {
            var postfix = new HarmonyMethod(typeof(Plugin).GetMethod(nameof(GenericPostfix),
                BindingFlags.NonPublic | BindingFlags.Static));

            int patched = 0;
            foreach (var asm in AppDomain.CurrentDomain.GetAssemblies())
            {
                Type[] types;
                try { types = asm.GetTypes(); }
                catch (ReflectionTypeLoadException ex) { types = ex.Types.Where(t => t != null).ToArray(); }
                catch { continue; }

                foreach (var t in types)
                {
                    if (t?.Namespace == null) continue;
                    if (!t.Namespace.StartsWith("Proto.Design", StringComparison.Ordinal)) continue;
                    if (!typeof(IMessage).IsAssignableFrom(t)) continue;
                    if (t.IsAbstract || t.ContainsGenericParameters) continue;

                    MethodInfo mergeMethod;
                    try
                    {
                        mergeMethod = t.GetMethod("MergeFrom", BindingFlags.Public | BindingFlags.Instance,
                            null, new[] { typeof(CodedInputStream) }, null);
                    }
                    catch { continue; }
                    if (mergeMethod == null) continue;

                    try
                    {
                        _harmony.Patch(mergeMethod, postfix: postfix);
                        patched++;
                    }
                    catch (Exception ex)
                    {
                        Log.LogWarning($"[BD2DataExtractor] Failed to patch {t.FullName}: {ex.Message}");
                    }
                }
            }
            return patched;
        }

        // Harmony postfix — runs after every table row finishes parsing off the wire.
        private static void GenericPostfix(object __instance)
        {
            try
            {
                if (!(__instance is IMessage msg)) return;

                string typeName = msg.Descriptor.Name;
                string key = TryGetDedupeKey(msg);

                lock (CaptureLock)
                {
                    if (!Captured.TryGetValue(typeName, out var dict))
                    {
                        dict = new Dictionary<string, IMessage>();
                        Captured[typeName] = dict;
                    }
                    dict[key] = msg;
                }
            }
            catch
            {
                // Never let instrumentation break the game.
            }
        }

        private static string TryGetDedupeKey(IMessage msg)
        {
            // Exact-content key, NOT just the "Id" field: several tables (e.g. AchievementTable)
            // have multiple distinct rows sharing the same Id, scoped by a separate field like
            // GroupId. Keying on Id alone silently drops rows when they collide. Keying on the
            // canonical serialized bytes only collapses genuinely identical re-parses of the same
            // row (e.g. the client re-syncing the same table twice), never distinct rows.
            return Convert.ToBase64String(msg.ToByteArray());
        }

        private void OnApplicationQuit()
        {
            DumpAll("quit");
        }

        private static void DumpAll(string reason)
        {
            var formatter = new JsonFormatter(JsonFormatter.Settings.Default);
            int tableCount = 0;
            int rowCount = 0;

            Dictionary<string, Dictionary<string, IMessage>> snapshot;
            lock (CaptureLock)
            {
                snapshot = Captured.ToDictionary(
                    kv => kv.Key,
                    kv => new Dictionary<string, IMessage>(kv.Value));
            }

            foreach (var kv in snapshot)
            {
                try
                {
                    var lines = kv.Value.Values.Select(m => formatter.Format(m));
                    var json = "[\n  " + string.Join(",\n  ", lines) + "\n]\n";
                    File.WriteAllText(Path.Combine(_outDir, kv.Key + ".json"), json);
                    tableCount++;
                    rowCount += kv.Value.Count;
                }
                catch (Exception ex)
                {
                    Log.LogWarning($"[BD2DataExtractor] Failed to dump {kv.Key}: {ex.Message}");
                }
            }

            Log.LogInfo($"[BD2DataExtractor] ({reason}) dumped {tableCount} tables / {rowCount} rows to {_outDir}");
        }
    }
}
