use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SpineInteractionRecordDeleteRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::spine_interaction::spine_interaction_record_delete;
use sqlx::SqlitePool;

#[put("SpineInteractionRecordDelete")]
async fn spine_interaction_record_delete_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<SpineInteractionRecordDeleteRequest>("SpineInteractionRecordDelete", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse SpineInteractionRecordDelete: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = spine_interaction_record_delete::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
