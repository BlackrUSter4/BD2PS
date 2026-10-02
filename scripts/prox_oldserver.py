import json
from mitmproxy import http
from mitmproxy import ctx

# Variant of prox.py that routes the client at the pre-built C# "old" BD2
# server (Bd2.Server.Api.exe, http://127.0.0.1:5000) instead of this repo's
# Rust httpserver (https://127.0.0.1:8443). Everything else (JS auth-token
# injection, launcher_g.json/game.json/10000002.json rewriting, analytics
# mock, streaming fix for large bundles) is identical to prox.py.

def load(loader):
    ctx.options.ssl_insecure = True

def _needs_body_inspection(path: str) -> bool:
    path_no_query = path.split("?", 1)[0]
    return path_no_query.endswith(".js") or any(
        x in path for x in ["launcher_g.json", "game.json", "10000002.json", "browndust2starter"]
    )

def responseheaders(flow: http.HTTPFlow):
    if "akamaized" in flow.request.pretty_host and ".bundle" in flow.request.path:
        ctx.log.info(f"--- BUNDLE REQUEST: status={flow.response.status_code} url={flow.request.pretty_url} ---")
    if not _needs_body_inspection(flow.request.path):
        flow.response.stream = True

def response(flow: http.HTTPFlow):
    path = flow.request.path
    path_no_query = path.split("?", 1)[0]
    host = flow.request.pretty_host

    if any(d in host for d in ["pmang.cloud", "pmang.com", "browndust2.com"]) or flow.request.port == 5000:
        body_preview = ""
        try:
            body_preview = flow.response.text[:500]
        except Exception:
            pass
        ctx.log.info(f"--- FULL RESPONSE: {flow.response.status_code} {flow.request.pretty_url} body={body_preview!r} ---")

    if path_no_query.endswith(".js") and flow.response and flow.response.text:
        ctx.log.info(f"--- Injecting Auth State into: {path} ---")
        bypass_payload = (
            ";(function(){"
            "try {"
            "  localStorage.setItem('mysession', 'LdIv57CNLTBsMLKy1JEgkBUr9A7ztIENBNYcT5d06lMYlRsA3srNPwuZmgy8jaym|15559314');"
            "  localStorage.setItem('uid', '1');"
            "  localStorage.setItem('email', 'developer@local.mesh');"
            "  localStorage.setItem('isLoggedIn', 'true');"
            "  localStorage.setItem('status', 'success');"
            "} catch(e) { console.log(e); }"
            "})();"
        )
        flow.response.text = bypass_payload + flow.response.text
        return

    if any(x in path for x in ["launcher_g.json", "game.json", "10000002.json"]):
        try:
            data = json.loads(flow.response.content.decode('utf-8'))
            data["status"] = "success"
            data["server_status"] = "NORMAL"
            data["install_status"] = "INSTALLED"
            data["api_gateway"] = "http://127.0.0.1:5000"

            if "patch" in data:
                data["patch"]["version"] = "1.0.0"
                data["patch"]["patch_url"] = "https://pc.bd2.pmang.cloud/browndust2starter/games/10000002/patch/"

            flow.response.content = json.dumps(data).encode('utf-8')
            flow.response.headers["Content-Type"] = "application/json"
        except Exception:
            pass

def request(flow: http.HTTPFlow):
    host = flow.request.pretty_host
    path = flow.request.path

    if not any(domain in host for domain in ["pmang.cloud", "pmang.com", "localhost", "127.0.0.1", "browndust2.com"]):
        return

    ctx.log.info(f"--- FULL REQUEST: {flow.request.method} {flow.request.pretty_url} ---")

    if "/message" in path and flow.request.method == "POST":
        mock_analytics = {"result_code": 0, "message": "Success", "status": 0, "data": {}}
        flow.response = http.Response.make(200, json.dumps(mock_analytics).encode('utf-8'), {"Content-Type": "application/json"})
        return

    if host.endswith("pmang.cloud") or host.endswith("pmang.com") or host == "pc.bd2.pmang.cloud":
        if any(x in path for x in ["launcher_g.json", "game.json", "10000002.json"]):
            return

        # Real game/patch asset downloads (multi-hundred-MB CDN files) must hit the
        # REAL Neowiz CDN, not our local server -- it doesn't have these files and
        # 404s, which the client reports as a network timeout. Only redirect actual
        # small JSON/protobuf API calls.
        path_no_query = path.split("?", 1)[0]
        if any(path_no_query.endswith(ext) for ext in (".dat", ".zip", ".7z", ".bundle", ".pak")):
            return

        flow.request.host = "127.0.0.1"
        flow.request.port = 5000
        flow.request.scheme = "http"
