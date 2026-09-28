use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LoginEventRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::login::login_event;
use sqlx::SqlitePool;

#[put("LoginEvent")]
async fn login_event_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = parse_packet::<LoginEventRequest>("LoginEvent", &body).map_err(|e| {
        tracing::warn!("Failed to parse LoginEvent: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = login_event::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
