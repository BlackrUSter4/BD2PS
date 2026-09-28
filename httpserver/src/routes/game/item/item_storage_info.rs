use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ItemStorageInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::item::item_storage_info;
use sqlx::SqlitePool;

#[put("ItemStorageInfo")]
async fn item_storage_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ItemStorageInfoRequest>("ItemStorageInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse ItemStorageInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = item_storage_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
