use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeHelperReconnectRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_helper_reconnect;
use sqlx::SqlitePool;

#[put("LifeHelperReconnect")]
async fn life_helper_reconnect_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeHelperReconnectRequest>("LifeHelperReconnect", &body).map_err(|e| {
        tracing::warn!("Failed to parse LifeHelperReconnect: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = life_helper_reconnect::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
