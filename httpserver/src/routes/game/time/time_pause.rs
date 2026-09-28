use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::TimePauseRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::time::time_pause;
use sqlx::SqlitePool;

#[put("TimePause")]
async fn time_pause_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<TimePauseRequest>("TimePause", &body).map_err(|e| {
        tracing::warn!("Failed to parse TimePause: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = time_pause::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
