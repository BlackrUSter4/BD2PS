use actix_web::{HttpResponse, Result, put, web};
use bd2::proto::proto_net::UseResourceItemRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::use2::use_resource_item;
use sqlx::SqlitePool;

#[put("UseResourceItem")]
async fn use_resource_item_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<UseResourceItemRequest>("UseResourceItem", &body).map_err(|e| {
        tracing::warn!("Failed to parse UseResourceItem: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = use_resource_item::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
