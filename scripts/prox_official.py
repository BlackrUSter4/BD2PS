import json
from mitmproxy import http
from mitmproxy import ctx

# Passthrough-only variant of prox.py for capturing real data from the
# OFFICIAL Brown Dust 2 servers. Unlike prox.py, this does NOT redirect
# pmang.cloud/pmang.com traffic to our local httpserver, and does NOT
# rewrite api_gateway in launcher_g.json/game.json/10000002.json -- it just
# observes and logs, exactly like a plain `mitmdump` with no script would,
# except it keeps the same streaming fix (large CDN bundle downloads would
# otherwise stall if buffered) and bundle-request logging as prox.py.

def load(loader):
    ctx.options.ssl_insecure = True

def _needs_body_inspection(path: str) -> bool:
    return False  # never buffer/inspect bodies in passthrough mode

def responseheaders(flow: http.HTTPFlow):
    if "akamaized" in flow.request.pretty_host and ".bundle" in flow.request.path:
        ctx.log.info(f"--- BUNDLE REQUEST: status={flow.response.status_code} url={flow.request.pretty_url} ---")
    if not _needs_body_inspection(flow.request.path):
        flow.response.stream = True

def response(flow: http.HTTPFlow):
    pass  # no rewriting at all -- real official responses pass through untouched

def request(flow: http.HTTPFlow):
    pass  # no redirection at all -- everything goes to its real destination
