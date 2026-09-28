use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SpineInteractionRecordInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::spine_interaction::spine_interaction_record_info;
use sqlx::SqlitePool;

#[put("SpineInteractionRecordInfo")]
async fn spine_interaction_record_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<SpineInteractionRecordInfoRequest>("SpineInteractionRecordInfo", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse SpineInteractionRecordInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = spine_interaction_record_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
