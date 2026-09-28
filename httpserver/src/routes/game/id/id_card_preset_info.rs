use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::IdCardPresetInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::id::id_card_preset_info;
use sqlx::SqlitePool;

#[put("IdCardPresetInfo")]
async fn id_card_preset_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<IdCardPresetInfoRequest>("IdCardPresetInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse IdCardPresetInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = id_card_preset_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
