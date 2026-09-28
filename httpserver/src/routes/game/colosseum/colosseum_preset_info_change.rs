use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumPresetInfoChangeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_preset_info_change;
use sqlx::SqlitePool;

#[put("ColosseumPresetInfoChange")]
async fn colosseum_preset_info_change_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumPresetInfoChangeRequest>("ColosseumPresetInfoChange", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumPresetInfoChange: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_preset_info_change::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
