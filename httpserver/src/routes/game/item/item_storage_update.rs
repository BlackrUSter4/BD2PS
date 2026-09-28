use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ItemStorageUpdateRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::item::item_storage_update;
use sqlx::SqlitePool;

#[put("ItemStorageUpdate")]
async fn item_storage_update_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<ItemStorageUpdateRequest>("ItemStorageUpdate", &body).map_err(|e| {
            tracing::warn!("Failed to parse ItemStorageUpdate: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = item_storage_update::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
