use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GachaInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::gacha::gacha_info;
use sqlx::SqlitePool;

#[put("GachaInfo")]
async fn gacha_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GachaInfoRequest>("GachaInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse GachaInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = gacha_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
