import json
from mitmproxy import http
from mitmproxy import ctx

def load(loader):
    ctx.options.ssl_insecure = True

def _needs_body_inspection(path: str) -> bool:
    path_no_query = path.split("?", 1)[0]
    return path_no_query.endswith(".js") or any(
        x in path for x in ["launcher_g.json", "game.json", "10000002.json"]
    )

def responseheaders(flow: http.HTTPFlow):
    if "akamaized" in flow.request.pretty_host and ".bundle" in flow.request.path:
        ctx.log.info(f"--- BUNDLE REQUEST: status={flow.response.status_code} url={flow.request.pretty_url} ---")
    # Defining response() forces mitmproxy to buffer every response body in memory before
    # calling it. That's fine for small JS/JSON config files, but it chokes on large game
    # asset bundles (multi-hundred-MB+ CDN downloads) — causing exactly the "download stalls
    # at a fixed percentage, retries forever" symptom. Stream anything we don't actually need
    # to inspect/modify straight through instead of buffering it.
    if not _needs_body_inspection(flow.request.path):
        flow.response.stream = True

def response(flow: http.HTTPFlow):
    path = flow.request.path

    # 1. GLOBAL JS INJECTION: Force local storage auth tokens into every webpage script loaded by the launcher
    # NOTE: must be a real .js file check, not a substring check — ".json" contains ".js" as a
    # prefix, so a naive "in" check here catches game.json/launcher_g.json/10000002.json too and
    # corrupts them by prepending JS to what must be valid JSON, breaking the client's config parse.
    path_no_query = path.split("?", 1)[0]
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
        # Prepend the payload so it executes the moment the browser initializes the script file
        flow.response.text = bypass_payload + flow.response.text
        return

    # 2. Route game configurations cleanly to point to your local development server
    if any(x in path for x in ["launcher_g.json", "game.json", "10000002.json"]):
        try:
            data = json.loads(flow.response.content.decode('utf-8'))
            data["status"] = "success"
            data["server_status"] = "NORMAL"
            data["install_status"] = "INSTALLED"
            data["api_gateway"] = "https://127.0.0.1:8443"
            
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

    # Allow third party dependencies to pass cleanly through
    if not any(domain in host for domain in ["pmang.cloud", "pmang.com", "localhost", "127.0.0.1", "browndust2.com"]):
        return

    # Serve placeholder responses for background analytics endpoints to prevent timeouts
    if "/message" in path and flow.request.method == "POST":
        mock_analytics = {"result_code": 0, "message": "Success", "status": 0, "data": {}}
        flow.response = http.Response.make(200, json.dumps(mock_analytics).encode('utf-8'), {"Content-Type": "application/json"})
        return

    # Route general game configuration traffic from live architecture directly to localhost
    if host.endswith("pmang.cloud") or host.endswith("pmang.com") or host == "pc.bd2.pmang.cloud":
        if any(x in path for x in ["launcher_g.json", "game.json", "10000002.json"]):
            return

        flow.request.host = "127.0.0.1"
        flow.request.port = 8443
        flow.request.scheme = "https"
