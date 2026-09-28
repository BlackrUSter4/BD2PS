use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ItemStorageOutRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::item::item_storage_out;
use sqlx::SqlitePool;

#[put("ItemStorageOut")]
async fn item_storage_out_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ItemStorageOutRequest>("ItemStorageOut", &body).map_err(|e| {
        tracing::warn!("Failed to parse ItemStorageOut: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = item_storage_out::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
