use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SpineInteractionRecordSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::spine_interaction::spine_interaction_record_save;
use sqlx::SqlitePool;

#[put("SpineInteractionRecordSave")]
async fn spine_interaction_record_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<SpineInteractionRecordSaveRequest>("SpineInteractionRecordSave", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse SpineInteractionRecordSave: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = spine_interaction_record_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
