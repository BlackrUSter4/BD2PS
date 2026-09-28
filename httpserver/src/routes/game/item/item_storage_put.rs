use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ItemStoragePutRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::item::item_storage_put;
use sqlx::SqlitePool;

#[put("ItemStoragePut")]
async fn item_storage_put_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ItemStoragePutRequest>("ItemStoragePut", &body).map_err(|e| {
        tracing::warn!("Failed to parse ItemStoragePut: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = item_storage_put::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
