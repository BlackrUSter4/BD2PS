use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ItemDumpRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::item::item_dump;
use sqlx::SqlitePool;

#[put("ItemDump")]
async fn item_dump_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ItemDumpRequest>("ItemDump", &body).map_err(|e| {
        tracing::warn!("Failed to parse ItemDump: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = item_dump::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
