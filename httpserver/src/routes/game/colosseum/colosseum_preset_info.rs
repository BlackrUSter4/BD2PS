use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumPresetInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_preset_info;
use sqlx::SqlitePool;

#[put("ColosseumPresetInfo")]
async fn colosseum_preset_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumPresetInfoRequest>("ColosseumPresetInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumPresetInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_preset_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
