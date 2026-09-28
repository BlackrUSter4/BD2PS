use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SupporterRemoveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::supporter::supporter_remove;
use sqlx::SqlitePool;

#[put("SupporterRemove")]
async fn supporter_remove_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<SupporterRemoveRequest>("SupporterRemove", &body).map_err(|e| {
        tracing::warn!("Failed to parse SupporterRemove: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = supporter_remove::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
