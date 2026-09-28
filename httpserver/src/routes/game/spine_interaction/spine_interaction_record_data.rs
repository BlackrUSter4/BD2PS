use actix_web::{get, web, HttpResponse, Result};
use database::db::spine_interaction::spine_interaction_record;
use sqlx::SqlitePool;

/// Stands in for the real backend's S3-hosted record blob — see
/// gameserver::logic::game::spine_interaction::record_data_url. Deliberately unauthenticated
/// (allowlisted in the auth middleware) since a real S3 GET wouldn't carry this game's session
/// cookie either.
#[get("SpineInteractionRecordData/{inven_index}")]
async fn spine_interaction_record_data_handler(
    pool: web::Data<SqlitePool>,
    inven_index: web::Path<i64>,
) -> Result<HttpResponse> {
    let record = spine_interaction_record::get_by_index_only(&pool, inven_index.into_inner())
        .await
        .ok()
        .flatten();

    match record.and_then(|r| r.record_data) {
        Some(data) => Ok(HttpResponse::Ok().content_type("application/octet-stream").body(data)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
