use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::UserGreetingChangeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::user::user_greeting_change;
use sqlx::SqlitePool;

#[put("UserGreetingChange")]
async fn user_greeting_change_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<UserGreetingChangeRequest>("UserGreetingChange", &body).map_err(|e| {
            tracing::warn!("Failed to parse UserGreetingChange: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = user_greeting_change::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
