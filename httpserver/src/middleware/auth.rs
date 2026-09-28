use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    middleware::Next,
};
use sqlx::SqlitePool;
use tracing::{debug, info, warn};

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let path = req.path();

    // Skip auth for public endpoints
    if path.contains("MaintenanceInfo")
        || path.contains("ServerInfo")
        || path.contains("ServerNowTime")
        || path.contains("NoticeInfo")
        || path.contains("StateCheckInfoJson")
        || path.contains("BalanceVersionCheck")
        || path.contains("JoinUser")
        || path.contains("LoginUser")
        || path.contains("/loki/api/v1/push")
        // Stands in for a real S3 URL (see spine_interaction::record_data_url) — the client
        // fetches this like a plain file host, without this game's session cookie.
        || path.contains("SpineInteractionRecordData")
        // Fetched the same plain-URL way (no cookie attached), and not a registered route at
        // all yet — without this exemption the client got a hard 401 here instead of the
        // normal default_service fallback, which made it disconnect with a "data not found"
        // error during the post-login sync burst instead of failing soft like every other
        // unimplemented route.
        || path.contains("sendmail")
    {
        info!("Skipping auth for public endpoint: {}", path);
        return next.call(req).await;
    }

    // Read the raw cookie header value
    let cookie_header = req
        .headers()
        .get("cookie")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            warn!("Missing cookie header for: {}", path);
            ErrorUnauthorized("Missing authentication")
        })?;

    debug!("Cookie header value: {}", cookie_header);

    // The cookie might be in format: "token|owner_index" or "name=token|owner_index"
    // Or multiple cookies: "cookie1=value1; cookie2=value2"
    // Try to find the auth token (format: something|number)
    let auth_value = cookie_header
        .split(';')
        .map(|s| s.trim())
        .find_map(|cookie| {
            // If it contains '=', extract the value part
            if let Some((_name, value)) = cookie.split_once('=') {
                // Check if value matches token|owner_index format
                if value.matches('|').count() == 1 {
                    return Some(value.to_string());
                }
            } else {
                // No '=', so the whole thing might be the token
                if cookie.matches('|').count() == 1 {
                    return Some(cookie.to_string());
                }
            }
            None
        })
        .ok_or_else(|| {
            warn!("No valid auth token found in cookies for: {}", path);
            ErrorUnauthorized("Missing authentication")
        })?;

    debug!("Extracted auth value: {}", auth_value);

    // Parse token|owner_index
    let owner_index = parse_and_validate_auth_cookie(&auth_value).map_err(|e| {
        warn!("Invalid auth token for {}: {}", path, e);
        ErrorUnauthorized("Invalid authentication")
    })?;

    // Get database pool and convert to UID
   // 1. Keep the pool retrieval intact so Actix doesn't crash
    let _pool = req
        .app_data::<actix_web::web::Data<SqlitePool>>()
        .ok_or_else(|| ErrorUnauthorized("Internal error"))?;

    // 2. COMMENT OUT THE REAL DATABASE LOOKUP
    /*
    let uid =
        gameserver::logic::game::account::get_uid_for_owner_index(pool.get_ref(), owner_index)
            .await
            .map_err(|e| {
                error!("Failed to get UID: {}", e);
                ErrorUnauthorized("Invalid authentication")
            })?;
    */

    // 3. HARDCODE A FAKE LOGICAL UID (e.g., User ID #1)
    let uid: i64 = 1; 

    info!(
        "BYPASSED AUTH: Hardcoded owner_index {} to (UID: {}) for {}",
        owner_index, uid, path
    );
    
    // 4. Attach the fake UID to the request extensions context exactly as expected
    req.extensions_mut().insert(uid);

    next.call(req).await
}
/// Parse and validate the auth cookie
/// Format: "access_token|owner_index"
/// Example: "LdIv57CNLTBsMLKy1JEgkBUr9A7ztIENBNYcT5d06lMYlRsA3srNPwuZmgy8jaym|15559314"
fn parse_and_validate_auth_cookie(cookie_value: &str) -> Result<i64, String> {
    let parts: Vec<&str> = cookie_value.split('|').collect();

    if parts.len() != 2 {
        return Err(format!(
            "Invalid cookie format: expected 2 parts, got {}",
            parts.len()
        ));
    }

    let access_token = parts[0];
    let owner_index_str = parts[1];

    // Validate access token is not empty and has reasonable length (at least 32 chars)
    if access_token.is_empty() || access_token.len() < 32 {
        return Err(format!(
            "Invalid access token length: {}",
            access_token.len()
        ));
    }

    // Validate access token contains only alphanumeric characters
    if !access_token.chars().all(|c| c.is_alphanumeric()) {
        return Err("Invalid access token characters".to_string());
    }

    // Parse owner_index
    let owner_index = owner_index_str
        .parse::<i64>()
        .map_err(|e| format!("Invalid owner_index: {}", e))?;

    // Validate owner_index is positive
    if owner_index <= 0 {
        return Err(format!(
            "Invalid owner_index: must be positive, got {}",
            owner_index
        ));
    }

    debug!(
        "Successfully validated auth cookie for owner_index: {}",
        owner_index
    );

    Ok(owner_index)
}
