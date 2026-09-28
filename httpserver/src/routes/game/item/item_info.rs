use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ItemInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::item::item_info;
use sqlx::SqlitePool;

#[put("ItemInfo")]
async fn item_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ItemInfoRequest>("ItemInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse ItemInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = item_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
