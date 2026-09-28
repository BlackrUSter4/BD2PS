use actix_web::{HttpRequest, HttpResponse, web};
use crypto::network::GameResponse;
use tracing::info;

/// Catches any route the client calls that isn't explicitly registered.
///
/// The new client (post client-update) calls ~193 new request types this
/// server doesn't implement yet (see CLIENT_UPDATE.md step 5). Without this,
/// actix's default 404 makes those UI screens hang/error for the player.
/// Returning a well-formed, empty success envelope instead lets those
/// screens fail soft (show "nothing here" instead of freezing), while the
/// info! log line below doubles as a live inventory of exactly which routes
/// the client actually calls that still need real implementations.
pub async fn fallback_handler(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let path = req.path().trim_start_matches('/').to_string();
    info!("[fallback] Unhandled route hit, returning empty success: {}", path);
    // TEMP DIAGNOSTIC (live-testing session, 2026-09-27): log the raw request body for
    // whichever route is currently under investigation (/sendmail/1 caused a client-side
    // "data not found" logic error on an empty fallback response) — remove once resolved.
    if path.starts_with("sendmail") {
        info!(
            "[fallback][sendmail-diagnostic] body ({} bytes): {}",
            body.len(),
            String::from_utf8_lossy(&body)
        );
        for (name, value) in req.headers().iter() {
            info!("[fallback][sendmail-diagnostic] header: {}: {:?}", name, value);
        }
    }
    let response = GameResponse::success(&path, &[], 0);
    HttpResponse::Ok().json(response)
}
