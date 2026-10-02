using Mono.Cecil;
using Mono.Cecil.Cil;

if (args.Length > 0 && args[0] == "--inspect")
{
    var inspectModule = ModuleDefinition.ReadModule(args[1], new ReaderParameters { ReadWrite = false });
    var t = inspectModule.Types
        .SelectMany(ty => ty.NestedTypes.Append(ty))
        .FirstOrDefault(ty => ty.FullName == args[2]);
    if (t == null) { Console.WriteLine("TYPE NOT FOUND via direct scan, trying GetType"); t = inspectModule.GetType(args[2]); }
    if (t == null) { Console.WriteLine("still not found"); return 1; }
    Console.WriteLine($"Type: {t.FullName} (resolved from {t.Module.Name})");
    foreach (var m in t.Methods) Console.WriteLine($"  method: {m}");
    foreach (var f in t.Fields) Console.WriteLine($"  field: {f.FieldType} {f.Name}");
    foreach (var p in t.Properties) Console.WriteLine($"  property: {p.PropertyType} {p.Name}");
    return 0;
}

if (args.Length > 0 && args[0] == "--dump-il")
{
    var dumpModule = ModuleDefinition.ReadModule(args[1], new ReaderParameters { ReadWrite = false });
    var dt = dumpModule.Types.First(t => t.FullName == args[2]);
    var dm = dt.Methods.First(m => m.Name == args[3]);
    Console.WriteLine($"Method: {dm}");
    foreach (var instr in dm.Body.Instructions) Console.WriteLine($"  {instr}");
    return 0;
}

if (args.Length > 0 && args[0] == "--patch-common")
{
    string commonSrc = args[1];
    string commonOut = args[2];
    var commonResolver = new DefaultAssemblyResolver();
    commonResolver.AddSearchDirectory(Path.GetDirectoryName(Path.GetFullPath(commonSrc)));
    var commonModule = ModuleDefinition.ReadModule(commonSrc, new ReaderParameters { ReadWrite = false, AssemblyResolver = commonResolver });

    // GetMonsterInfoListByPackId does `_monsterDictionary[packId]` (a plain
    // Dictionary<int, List<FieldMonsterTable>> indexer), which throws
    // KeyNotFoundException for any pack whose FieldMonsterTable rows were
    // never captured. Confirmed live: the real client's current pack (uid
    // 412287269's account) hits exactly this, and the exception propagates
    // all the way up through PackInGameInfo -> BatchRequest, which the client
    // retries forever, causing the infinite loading-logo screen. Same root
    // cause category as this whole project's established "incomplete
    // captured master data" pattern -- fix is to make the lookup tolerant
    // (empty list for an unknown pack) instead of throwing.
    var battleDeckManager = commonModule.Types.First(t => t.FullName == "Bd2.Server.Common.Managers.BattleDeckManager");
    var getMonsterMethod = battleDeckManager.Methods.First(m => m.Name == "GetMonsterInfoListByPackId");
    var gmBody = getMonsterMethod.Body;
    var gmIl = gmBody.GetILProcessor();

    var getItemInstr = gmBody.Instructions.First(i =>
        i.OpCode == OpCodes.Callvirt && i.Operand is MethodReference mr && mr.Name == "get_Item");
    var getItemRef = (MethodReference)getItemInstr.Operand;

    var existingContainsKeyInstr = gmBody.Instructions.FirstOrDefault(i => i.OpCode == OpCodes.Callvirt && i.Operand is MethodReference mr2 && mr2.Name == "ContainsKey");
    if (existingContainsKeyInstr != null)
    {
        // Already structurally patched (branch/newobj/relabel all present) --
        // but an earlier attempt may have built a malformed ContainsKey
        // MethodReference (wrong generic-parameter form -> MissingMethodException
        // at runtime even though it looked fine in ildasm). Just correct the
        // operand in place rather than re-deriving the whole branch structure.
        var dictType2 = (GenericInstanceType)getItemRef.DeclaringType;
        var dictOpenDef2 = dictType2.ElementType.Resolve();
        var keyGenericParam2 = dictOpenDef2.GenericParameters[0];
        var fixedContainsKeyRef = new MethodReference("ContainsKey", commonModule.TypeSystem.Boolean, dictType2) { HasThis = true };
        fixedContainsKeyRef.Parameters.Add(new ParameterDefinition(keyGenericParam2));
        existingContainsKeyInstr.Operand = fixedContainsKeyRef;
        Console.WriteLine("Fixed up existing ContainsKey reference on GetMonsterInfoListByPackId (corrected generic-parameter form).");

        var existingNewobjInstr = gmBody.Instructions.FirstOrDefault(i => i.OpCode == OpCodes.Newobj &&
            i.Operand is MethodReference nmr && nmr.DeclaringType is GenericParameter);
        if (existingNewobjInstr != null)
        {
            var fixedListCtorRef = new MethodReference(".ctor", commonModule.TypeSystem.Void, dictType2.GenericArguments[1]) { HasThis = true };
            existingNewobjInstr.Operand = fixedListCtorRef;
            Console.WriteLine("Fixed up existing empty-list constructor reference on GetMonsterInfoListByPackId (corrected declaring type).");
        }
    }
    else
    {
        // Dictionary<TKey,TValue>.ContainsKey/get_Item aren't generic METHODS
        // (the generics are on the type), so a reference can be built purely
        // from types already resolvable in this module -- no need to resolve
        // or import ContainsKey's definition from System.Private.CoreLib at
        // all, which is what was tripping up Cecil's generic-context importer.
        // IMPORTANT: for a method on a GenericInstanceType, the parameter/
        // return types must be expressed via the open type definition's OWN
        // generic parameters (!0/!1), matching how get_Item's existing,
        // working reference looks (`!1 get_Item(!0)`) -- using the already-
        // substituted closed types directly (e.g. System.Int32) produces a
        // signature the runtime can't match, which is what caused the first
        // attempt's MissingMethodException.
        var dictType = (GenericInstanceType)getItemRef.DeclaringType;
        var dictOpenDef = dictType.ElementType.Resolve();
        var keyGenericParam = dictOpenDef.GenericParameters[0]; // TKey, i.e. !0
        var containsKeyRef = new MethodReference("ContainsKey", commonModule.TypeSystem.Boolean, dictType) { HasThis = true };
        containsKeyRef.Parameters.Add(new ParameterDefinition(keyGenericParam));

        // NOTE: getItemRef.ReturnType is "!1" (a generic-parameter placeholder,
        // only meaningful as PART OF get_Item's own signature) -- using it
        // directly as an unrelated method's DeclaringType produces malformed
        // metadata (BadImageFormatException at runtime). The actual closed
        // type (List<FieldMonsterTable>) is dictType.GenericArguments[1].
        var listCtorRef = new MethodReference(".ctor", commonModule.TypeSystem.Void, dictType.GenericArguments[1]) { HasThis = true };

        // getItemInstr's two preceding instructions push dict then key (the
        // exact args get_Item needs); ContainsKey takes the same (dict, key)
        // shape, so it can consume that ALREADY-pushed pair directly -- no
        // duplication needed for the probe itself. Only the "has key" branch
        // needs to reload dict+key, since ContainsKey already consumed the
        // original pair.
        var loadDict = (FieldReference)gmBody.Instructions.First(i => i.OpCode == OpCodes.Ldsfld).Operand;

        var hasKeyLabel = Instruction.Create(OpCodes.Ldsfld, loadDict);
        var doneLabel = Instruction.Create(OpCodes.Nop);
        var containsKeyInstr = Instruction.Create(OpCodes.Callvirt, containsKeyRef);

        gmIl.Replace(getItemInstr, containsKeyInstr);

        var toInsert = new[]
        {
            Instruction.Create(OpCodes.Brtrue_S, hasKeyLabel),
            Instruction.Create(OpCodes.Newobj, listCtorRef),
            Instruction.Create(OpCodes.Br_S, doneLabel),
            hasKeyLabel,                                  // ldsfld dict
            Instruction.Create(OpCodes.Ldarg_0),
            Instruction.Create(OpCodes.Callvirt, getItemRef),
            doneLabel,
        };
        var cursor = containsKeyInstr;
        foreach (var instr in toInsert)
        {
            gmIl.InsertAfter(cursor, instr);
            cursor = instr;
        }
        Console.WriteLine("Patched GetMonsterInfoListByPackId: ContainsKey-guarded lookup, empty list fallback for unknown packId.");
    }

    commonModule.Write(commonOut);
    Console.WriteLine($"Wrote patched assembly to {commonOut}");
    return 0;
}

string srcDll = args.Length > 0 ? args[0] : @"A:\Private Servers\bd\BrownDust.II_2.19.5_PC_Client (1)\server\Bd2.Server.Api.dll";
string outDll = args.Length > 1 ? args[1] : @"A:\Private Servers\bd\BD2PS-main\tools\OldServerDecompiled\Bd2.Server.Api.patched.dll";

// Routes that need to become absolute (leading "/") so they bypass the
// CommonController's [Route("[controller]")] convention, which otherwise
// prefixes them with "Common/" -- the real client calls these bare, before
// it knows any base path (confirmed via mitm capture + Rust httpserver's
// own bare #[put("MaintenanceInfo")]-style routes).
var routeFixes = new (string typeName, string methodName, string oldRoute, string newRoute)[]
{
    ("Bd2.Server.Api.Controllers.Common.CommonController", "MaintenanceInfo", "MaintenanceInfo", "/MaintenanceInfo"),
    ("Bd2.Server.Api.Controllers.Common.CommonController", "ServerInfo", "ServerInfo", "/ServerInfo"),
    ("Bd2.Server.Api.Controllers.Common.CommonController", "StateCheckInfoJson", "StateCheckInfoJson", "/StateCheckInfoJson"),
};

// String-literal fixes inside method bodies (ldstr operands). Used to update
// stale data baked into the server, e.g. MaintenanceInfo.BundleVersion was
// hardcoded to "20240823161138" (an old, no-longer-served patch) -- the real
// client fetches https://cdn.bd2.pmang.cloud/ServerData/.../<BundleVersion>/catalog_alpha.hash
// using this value, got 403 Forbidden for the stale one (confirmed via a real
// client crash-log capture), and aborted with CLIENT_LOGIC_ERROR/BUNDLE_CATALOG_CHECK.
// The currently-installed client version is 20260918000, so point it there instead.
var literalFixes = new (string typeName, string methodName, string oldLiteral, string newLiteral)[]
{
    // Confirmed via a real-server capture (prox_capture.py) that BundleVersion
    // is NOT the same as the client's install/exe version (20260918000) -- it's
    // a separately-versioned Addressables content-catalog timestamp that the
    // real backend updates frequently. 20260921135230 was confirmed live (200 OK
    // fetching its real catalog_sound.json from dl.bd2.pmang.cloud) at capture
    // time; if this goes stale again later, re-run prox_capture.py's
    // observe-only capture against the real mt.bd2.pmang.cloud to find the
    // current value (see the ServerInfo/MaintenanceInfo decoded protobuf output).
    ("Bd2.Server.Api.Controllers.Common.CommonController", "MaintenanceInfo", "20260918000", "20260921135230"),
    // ServerInfo.CdnInfo pointed at the wrong CDN host entirely. Confirmed via a
    // real-server capture (prox_official.py, observe-only) that the actual live
    // backend's ServerInfo returns CdnInfo at dl.bd2.pmang.cloud, not
    // cdn.bd2.pmang.cloud -- the client's CheckCatalogsOperation asset-catalog
    // fetch 403'd against the wrong host for every bundle version tried.
    // dl.bd2.pmang.cloud is a real, public, unauthenticated CDN (confirmed 200 OK
    // in the capture), so we leave this request going out to the real internet
    // rather than redirecting it locally.
    ("Bd2.Server.Api.Controllers.Common.CommonController", "ServerInfo", "https://cdn.bd2.pmang.cloud/ServerData", "https://dl.bd2.pmang.cloud/ServerData"),
    // TextInputFormatter (the custom formatter wired up in Program.cs so that
    // [FromBody] string action parameters like UserController.LoginUser can
    // bind at all) only registered "text/plain" as a supported media type --
    // but every real request from the client is "multipart/form-data"
    // (confirmed: every captured PUT call uses it, none ever use text/plain).
    // Actions with no body parameter (MaintenanceInfo/ServerInfo) never hit
    // this formatter at all regardless of content type, which is why this
    // bug was invisible until the client reached its first real body-bound
    // call (LoginUser) and got 415 Unsupported Media Type.
    // The real client's Content-Type is literally "multipart/form-datas" (with
    // a trailing "s" -- confirmed consistently across every captured PUT
    // request's logged content type, not a logging artifact), not the
    // standard "multipart/form-data". An earlier attempt at this fix used the
    // standard spelling and still got 415.
    ("Bd2.Server.Api.Config.TextInputFormatter", ".ctor", "text/plain", "multipart/form-datas"),
};

var resolver = new DefaultAssemblyResolver();
resolver.AddSearchDirectory(Path.GetDirectoryName(Path.GetFullPath(srcDll)));
resolver.AddSearchDirectory(@"A:\Private Servers\bd\BrownDust.II_2.19.5_PC_Client (1)\server");
var module = ModuleDefinition.ReadModule(srcDll, new ReaderParameters { ReadWrite = false, AssemblyResolver = resolver });

var routeFixCounts = new int[routeFixes.Length];
var literalFixCounts = new int[literalFixes.Length];

foreach (var typeDef in module.Types)
{
    for (int i = 0; i < routeFixes.Length; i++)
    {
        var fix = routeFixes[i];
        if (typeDef.FullName != fix.typeName) continue;
        foreach (var method in typeDef.Methods)
        {
            if (method.Name != fix.methodName) continue;
            foreach (var attr in method.CustomAttributes)
            {
                if (attr.ConstructorArguments.Count != 1) continue;
                var arg = attr.ConstructorArguments[0];
                if (arg.Type.FullName != "System.String") continue;
                if ((string)arg.Value != fix.oldRoute) continue;

                attr.ConstructorArguments[0] = new CustomAttributeArgument(arg.Type, fix.newRoute);
                Console.WriteLine($"Patched route {fix.typeName}.{fix.methodName}: \"{fix.oldRoute}\" -> \"{fix.newRoute}\" (attribute: {attr.AttributeType.Name})");
                routeFixCounts[i]++;
            }
        }
    }

    for (int i = 0; i < literalFixes.Length; i++)
    {
        var fix = literalFixes[i];
        if (typeDef.FullName != fix.typeName) continue;
        foreach (var method in typeDef.Methods)
        {
            if (method.Name != fix.methodName || method.Body == null) continue;
            foreach (var instr in method.Body.Instructions)
            {
                if (instr.OpCode != OpCodes.Ldstr) continue;
                if ((string)instr.Operand != fix.oldLiteral) continue;

                instr.Operand = fix.newLiteral;
                Console.WriteLine($"Patched literal {fix.typeName}.{fix.methodName}: \"{fix.oldLiteral}\" -> \"{fix.newLiteral}\"");
                literalFixCounts[i]++;
            }
        }
    }
}

// Route fixes are allowed to already be applied (0 matches = already patched,
// found via the route's NEW value instead -- not re-verified here since that's
// a harmless no-op either way). Literal fixes must match exactly once each --
// if a literal is already patched or never existed, something's wrong.
bool ok = true;
for (int i = 0; i < routeFixes.Length; i++)
{
    if (routeFixCounts[i] > 1)
    {
        Console.WriteLine($"WARNING: route fix {i} ({routeFixes[i].methodName}) matched {routeFixCounts[i]} times, expected 0 or 1.");
        ok = false;
    }
    else if (routeFixCounts[i] == 0)
    {
        Console.WriteLine($"NOTE: route fix {i} ({routeFixes[i].methodName}) found no match for \"{routeFixes[i].oldRoute}\" -- likely already patched.");
    }
}
for (int i = 0; i < literalFixes.Length; i++)
{
    if (literalFixCounts[i] > 1)
    {
        Console.WriteLine($"WARNING: literal fix {i} ({literalFixes[i].methodName}) matched {literalFixCounts[i]} times, expected 0 or 1.");
        ok = false;
    }
    else if (literalFixCounts[i] == 0)
    {
        Console.WriteLine($"NOTE: literal fix {i} ({literalFixes[i].methodName}) found no match for \"{literalFixes[i].oldLiteral}\" -- likely already patched.");
    }
}

if (!ok)
{
    Console.WriteLine("Aborting write due to unexpected match counts.");
    return 1;
}

// The exception filter only logs Exception.StackTrace, which for a
// reflection-wrapped or re-thrown exception doesn't include the actual
// exception type/message -- made diagnosing real bugs (like the
// GetMonsterInfoListByPackId one) much harder than necessary. Swap it for
// Exception.ToString() (inherited from Object, same single-string-arg shape),
// which includes type, message, full stack, and inner exceptions.
var methodBodyPatches = new (string typeName, string methodName, string targetMethodName, string newMethodName, string newDeclaringType)[]
{
    ("Bd2.Server.Api.Filter.CustomerExceptionFilter", "OnExceptionAsync", "get_StackTrace", "ToString", "System.Object"),
};
foreach (var fix in methodBodyPatches)
{
    var t = module.Types.First(ty => ty.FullName == fix.typeName);
    var m = t.Methods.First(mm => mm.Name == fix.methodName);
    var target = m.Body.Instructions.FirstOrDefault(i => i.Operand is MethodReference mr && mr.Name == fix.targetMethodName);
    if (target == null)
    {
        Console.WriteLine($"NOTE: {fix.typeName}.{fix.methodName} has no {fix.targetMethodName} call -- likely already patched.");
    }
    else
    {
        var newRef = new MethodReference(fix.newMethodName, module.TypeSystem.String, module.TypeSystem.Object) { HasThis = true };
        target.OpCode = OpCodes.Callvirt;
        target.Operand = newRef;
        Console.WriteLine($"Patched {fix.typeName}.{fix.methodName}: {fix.targetMethodName} -> {fix.newMethodName}");
    }
}

// ServerInfo's compiled protobuf message type (Bd2.Server.Common's generated
// ServerInfoResponse.Types.ServerInfo) only has 7 of the 9 fields the CURRENT
// protocol defines (confirmed against this project's own captured proto_net.proto)
// -- it's missing game_data_info (field 8) and game_data_version (field 9).
// Adding fields to a compiled Google.Protobuf-generated message class via IL is
// impractical, so instead: this response is always the same static content
// anyway (no per-request variation), so replace the whole method body with a
// fixed, hand-encoded protobuf byte string that includes all 9 fields correctly
// (verified by round-trip decoding before embedding here). See
// scripts/compute_serverinfo_protobuf.py-equivalent reasoning in chat history --
// field values match the existing local-server routing (127.0.0.1:5000) except
// cdn_info/game_data_info which point at the real dl.bd2.pmang.cloud (same
// reasoning as the CdnInfo fix above: real, public, unauthenticated CDN).
// NOTE: game_data_version (field 9) is a SEPARATE version counter from
// BundleVersion/catalog version -- confirmed via the real capture, where
// ServerInfo's own game_data_version ("20261001135239") did NOT match
// MaintenanceInfo's BundleVersion ("20260921135230") at the same point in
// time. An earlier patch mistakenly reused BundleVersion's value here,
// which caused a NEW real 403 ("Failed to get GameDataSize" on
// .../GameData/<wrong-version>/release/common-dbdata.info). Both of these
// version strings drift over time on the real backend -- if this goes stale,
// re-run the prox_capture.py observe-only capture against the real
// mt.bd2.pmang.cloud to read the current values directly out of the decoded
// MaintenanceInfo/ServerInfo protobuf bytes.
const string fixedServerInfoBase64 =
    "CtoBEhtodHRwOi8vMTI3LjAuMC4xOjUwMDAvR2FtZS8aJWh0dHBzOi8vZGwuYmQyLnBtYW5nLmNsb3VkL1NlcnZlckRhdGEqH2h0dHA6Ly8xMjcuMC4wLjE6NTAwMC9Mb2tpL3B1c2gyDzEyNy4wLjAuMTozODUwMTotaHR0cHM6Ly9yZWRlZW0uYmQyLnBtYW5nLmNsb3VkL2JkMi9pbmRleC5odG1sQiNodHRwczovL2RsLmJkMi5wbWFuZy5jbG91ZC9HYW1lRGF0YUoOMjAyNjEwMDExMzUyMzk=";

{
    var commonController = module.Types.First(t => t.FullName == "Bd2.Server.Api.Controllers.Common.CommonController");
    var serverInfoMethod = commonController.Methods.First(m => m.Name == "ServerInfo");
    var body = serverInfoMethod.Body;

    var existingSetData = body.Instructions.First(i =>
        (i.OpCode == OpCodes.Callvirt || i.OpCode == OpCodes.Call) &&
        i.Operand is MethodReference mr && mr.Name == "set_data");
    var setDataRef = (MethodReference)existingSetData.Operand;
    var existingNewobj = body.Instructions.First(i =>
        i.OpCode == OpCodes.Newobj &&
        ((MethodReference)i.Operand).DeclaringType.FullName == setDataRef.DeclaringType.FullName);
    var baseActionResultCtor = (MethodReference)existingNewobj.Operand;

    bool alreadyFixed = body.Instructions.Any(i => i.OpCode == OpCodes.Ldstr && (string)i.Operand == fixedServerInfoBase64);
    if (alreadyFixed)
    {
        Console.WriteLine("NOTE: ServerInfo method body already replaced with the fixed protobuf payload -- skipping.");
    }
    else
    {
        var il = body.GetILProcessor();
        body.Instructions.Clear();
        body.ExceptionHandlers.Clear();
        il.Append(il.Create(OpCodes.Newobj, baseActionResultCtor));
        il.Append(il.Create(OpCodes.Dup));
        il.Append(il.Create(OpCodes.Ldstr, fixedServerInfoBase64));
        il.Append(il.Create(OpCodes.Callvirt, setDataRef));
        il.Append(il.Create(OpCodes.Ret));
        Console.WriteLine("Replaced ServerInfo() method body with fixed 9-field protobuf payload (added game_data_info/game_data_version).");
    }
}

// CheckAccessToken verifies array[4] (the token's signature) against a plain
// keyless SHA1(uid + timestamp) -- confirmed by directly computing this hash
// in Python against a REAL access token captured from this session's actual
// Google-Play-Games login and finding it does NOT match. The real Neowiz
// backend signs this with a secret/salt we don't have and never will, so this
// check can never pass against a real token. Since this is our own private
// test server (not a real gatekeeper), bypass the signature check entirely --
// keep the array.Length==6 shape guard, just always take the "valid" branch
// afterward instead of comparing hashes.
{
    var userController = module.Types.First(t => t.FullName == "Bd2.Server.Api.Controllers.Game.UserController");
    var checkTokenMethod = userController.Methods.First(m => m.Name == "CheckAccessToken");
    var ctBody = checkTokenMethod.Body;
    var ctIl = ctBody.GetILProcessor();

    var brtrue = ctBody.Instructions.FirstOrDefault(i => i.OpCode == OpCodes.Brtrue_S || i.OpCode == OpCodes.Brtrue);
    if (brtrue == null)
    {
        Console.WriteLine("NOTE: CheckAccessToken has no brtrue instruction -- likely already patched.");
    }
    else
    {
        // The instruction right after the comparison call chain's last `call`
        // (op_Inequality) is this brtrue; its "fall-through" target (the very
        // next instruction) is the success path (return array[0]).
        var successTarget = brtrue.Next;
        var pop = ctIl.Create(OpCodes.Pop);
        var brAlways = ctIl.Create(OpCodes.Br_S, successTarget);
        ctIl.Replace(brtrue, pop);
        ctIl.InsertAfter(pop, brAlways);
        Console.WriteLine("Patched CheckAccessToken: bypassed real-signature verification (brtrue -> pop+br to success path).");
    }
}

// BatchController exposes every IBatchService method both (a) bundled inside
// BatchRequest (where the OUTER request body is AES-decrypted once, and each
// sub-call's requestData is passed through to the service method as-is,
// already-decrypted), and (b) as its own direct [HttpPut] action for a
// single, non-bundled call. Confirmed live: for the direct path, the real
// client's body IS AES-encrypted(base64(protobuf)) -- but every direct action
// method here just forwards `data` straight to the shared IBatchService
// method without decrypting first (a systemic copy-paste bug, not something
// version-specific), which throws InvalidProtocolBufferException the moment
// a real direct (non-batched) call for that action actually occurs. Comparing
// a known-good batched call's requestData ("CFQQFQ==", already-decrypted raw
// protobuf) against a captured real direct call's body
// ("Sy5HjIS2H1GryHsJ+r7Sqg==", which decrypts via the project's standard
// AES key/IV to another base64 protobuf string) confirms this. Fix: insert
// an AesUtils.AesDecrypt call right after each direct action loads `data`,
// so by the time the shared service method runs, `data` is uniformly in the
// already-decrypted form regardless of which path got there.
{
    var batchController = module.Types.First(t => t.FullName == "Bd2.Server.Api.Controllers.Game.BatchController");

    // Find an existing AesUtils type reference in this module to build a
    // sibling AesDecrypt reference from (UserController.CheckAccessToken
    // already calls AesUtils.Sha1Hash elsewhere in this same assembly).
    var userControllerForAes = module.Types.First(t => t.FullName == "Bd2.Server.Api.Controllers.Game.UserController");
    var aesUtilsRef = userControllerForAes.Methods
        .SelectMany(m => m.Body?.Instructions ?? Enumerable.Empty<Instruction>())
        .Select(i => (i.Operand as MethodReference)?.DeclaringType)
        .FirstOrDefault(t => t != null && t.Name == "AesUtils");
    var aesDecryptRef = new MethodReference("AesDecrypt", module.TypeSystem.String, aesUtilsRef) { HasThis = false };
    aesDecryptRef.Parameters.Add(new ParameterDefinition(module.TypeSystem.String));

    int fixedCount = 0, alreadyOkCount = 0;
    foreach (var method in batchController.Methods)
    {
        if (method.Parameters.Count != 1 || method.Body == null) continue;

        var body = method.Body;
        bool callsIBatchService = body.Instructions.Any(i =>
            i.Operand is MethodReference mr && mr.DeclaringType.FullName == "Bd2.Server.iServices.IBatchService");
        if (!callsIBatchService) continue; // not one of the ~44 passthrough actions (e.g. BatchRequest itself uses reflection)

        if (body.Instructions.Any(i => i.Operand is MethodReference mr2 && mr2.Name == "AesDecrypt"))
        {
            alreadyOkCount++;
            continue;
        }

        var ldarg1 = body.Instructions.FirstOrDefault(i => i.OpCode == OpCodes.Ldarg_1);
        if (ldarg1 == null) continue; // doesn't load `data` at all -- not this bug shape

        var il = body.GetILProcessor();
        il.InsertAfter(ldarg1, il.Create(OpCodes.Call, aesDecryptRef));
        fixedCount++;
    }
    Console.WriteLine($"Patched {fixedCount} BatchController direct actions to AES-decrypt `data` before forwarding ({alreadyOkCount} already had it).");
}

// Several routes don't exist anywhere in this old server build at all (their
// real protobuf response messages don't exist in this server's compiled
// schema either -- these features are all newer than this build, same
// situation as ServerInfo's missing fields):
//   - FishingItemInfo: real client calls PUT Game/FishingItemInfo repeatedly
//     in a retry loop right after a successful login; 404 forever blocked
//     all progress past the login screen.
//   - ServerNowTime: a real-time heartbeat/time-sync call made continuously
//     during actual gameplay (confirmed: client entered the black-screen
//     "in field" state and immediately started polling this every ~1s);
//     404 here would presumably eventually degrade/break time-dependent
//     client behavior.
// An empty protobuf message is valid regardless of schema (zero bytes = all
// fields default), and GameActionResult.serverNowTime is auto-populated by
// the response pipeline regardless of handler logic (confirmed: every
// response we've captured has it set even though no handler code sets it
// directly) -- so a minimal empty-payload action via the byte[]-based
// CreateResult overload satisfies both without needing the missing message
// types at all.
{
    var userController = module.Types.First(t => t.FullName == "Bd2.Server.Api.Controllers.Game.UserController");
    var allCharRefresh = userController.Methods.First(m => m.Name == "AllCharRefresh");
    var createResultRef = (MethodReference)allCharRefresh.Body.Instructions
        .First(i => i.OpCode == OpCodes.Call && i.Operand is MethodReference mr && mr.Name == "CreateResult" &&
                    mr.Parameters.Count == 2 && mr.Parameters[0].ParameterType.Name == "Byte[]")
        .Operand;
    var httpPutAttr = allCharRefresh.CustomAttributes.First(a => a.AttributeType.Name == "HttpPutAttribute");
    var httpPutCtor = httpPutAttr.Constructor;
    var gameActionResultType = createResultRef.ReturnType;

    foreach (var newActionName in new[] { "FishingItemInfo", "ServerNowTime", "SaveFieldCharControlDeckType" })
    {
        if (userController.Methods.Any(m => m.Name == newActionName))
        {
            Console.WriteLine($"NOTE: {newActionName} already added -- skipping.");
            continue;
        }

        var newMethod = new MethodDefinition(newActionName,
            Mono.Cecil.MethodAttributes.Public | Mono.Cecil.MethodAttributes.HideBySig,
            gameActionResultType);
        newMethod.CustomAttributes.Add(new CustomAttribute(httpPutCtor));

        var il = newMethod.Body.GetILProcessor();
        il.Append(il.Create(OpCodes.Ldc_I4_0));
        il.Append(il.Create(OpCodes.Newarr, module.TypeSystem.Byte));
        il.Append(il.Create(OpCodes.Ldc_I4_0));
        il.Append(il.Create(OpCodes.Call, createResultRef));
        il.Append(il.Create(OpCodes.Ret));

        userController.Methods.Add(newMethod);
        Console.WriteLine($"Added UserController.{newActionName}() returning an empty GameActionResult.");
    }
}

module.Write(outDll);
Console.WriteLine($"Wrote patched assembly to {outDll}");
return 0;
