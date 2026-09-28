use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::IbItemInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::ib::ib_item_info;
use sqlx::SqlitePool;

#[put("IbItemInfo")]
async fn ib_item_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<IbItemInfoRequest>("IbItemInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse IbItemInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = ib_item_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
