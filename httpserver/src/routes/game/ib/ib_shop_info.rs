use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::IbShopInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::ib::ib_shop_info;
use sqlx::SqlitePool;

#[put("IbShopInfo")]
async fn ib_shop_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<IbShopInfoRequest>("IbShopInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse IbShopInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = ib_shop_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
