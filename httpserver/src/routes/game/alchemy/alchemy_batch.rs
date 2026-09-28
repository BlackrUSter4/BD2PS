use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::AlchemyBatchRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::alchemy::alchemy_batch;
use sqlx::SqlitePool;

#[put("AlchemyBatch")]
async fn alchemy_batch_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<AlchemyBatchRequest>("AlchemyBatch", &body).map_err(|e| {
        tracing::warn!("Failed to parse AlchemyBatch: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = alchemy_batch::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
