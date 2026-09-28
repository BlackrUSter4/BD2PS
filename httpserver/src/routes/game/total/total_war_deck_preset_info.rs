use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::TotalWarDeckPresetInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::total::total_war_deck_preset_info;
use sqlx::SqlitePool;

#[put("TotalWarDeckPresetInfo")]
async fn total_war_deck_preset_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<TotalWarDeckPresetInfoRequest>("TotalWarDeckPresetInfo", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse TotalWarDeckPresetInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = total_war_deck_preset_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
