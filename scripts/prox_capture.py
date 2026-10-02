import base64
from mitmproxy import http
from mitmproxy import ctx

# One-shot diagnostic capture: pure passthrough (no redirection, nothing
# modified) but logs full URLs and response bodies for the real BD2 backend
# hosts, so we can learn the real MaintenanceInfo/ServerInfo response fields
# (BundleVersion, CdnInfo) and the real catalog-fetch URL without guessing.

INTERESTING_HOSTS = ["mt.bd2.pmang.cloud", "dl.bd2.pmang.cloud", "cdn.bd2.pmang.cloud", "api-cf.bd2.pmang.cloud"]

def load(loader):
    ctx.options.ssl_insecure = True

def _is_interesting(host: str) -> bool:
    return any(h in host for h in INTERESTING_HOSTS)

def responseheaders(flow: http.HTTPFlow):
    if _is_interesting(flow.request.pretty_host):
        flow.response.stream = False
    else:
        flow.response.stream = True

def request(flow: http.HTTPFlow):
    if _is_interesting(flow.request.pretty_host):
        ctx.log.info(f"=== REQUEST: {flow.request.method} {flow.request.pretty_url} ===")

def response(flow: http.HTTPFlow):
    if not _is_interesting(flow.request.pretty_host):
        return
    body_text = ""
    try:
        body_text = flow.response.text or ""
    except Exception as e:
        body_text = f"<decode error: {e}>"
    ctx.log.info(f"=== RESPONSE: {flow.response.status_code} {flow.request.pretty_url} ===\nbody: {body_text[:2000]}")

    # MaintenanceInfo/ServerInfo wrap a base64-encoded protobuf in a "data" field
    # inside a JSON envelope -- decode it to raw bytes so we can grep for the
    # real BundleVersion/CdnInfo string values without a full proto schema.
    try:
        import json
        parsed = json.loads(body_text)
        if isinstance(parsed, dict) and "data" in parsed:
            raw = base64.b64decode(parsed["data"])
            ctx.log.info(f"=== DECODED PROTOBUF BYTES (lossy latin1) ===\n{raw.decode('latin1')}")
    except Exception:
        pass
