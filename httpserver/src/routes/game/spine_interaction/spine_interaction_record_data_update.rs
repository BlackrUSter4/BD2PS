use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SpineInteractionRecordDataUpdateRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::spine_interaction::spine_interaction_record_data_update;
use sqlx::SqlitePool;

#[put("SpineInteractionRecordDataUpdate")]
async fn spine_interaction_record_data_update_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<SpineInteractionRecordDataUpdateRequest>("SpineInteractionRecordDataUpdate", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse SpineInteractionRecordDataUpdate: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = spine_interaction_record_data_update::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
