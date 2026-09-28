use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SupporterRegisterRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::supporter::supporter_register;
use sqlx::SqlitePool;

#[put("SupporterRegister")]
async fn supporter_register_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<SupporterRegisterRequest>("SupporterRegister", &body).map_err(|e| {
            tracing::warn!("Failed to parse SupporterRegister: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = supporter_register::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
