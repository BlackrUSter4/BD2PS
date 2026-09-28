use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MailOpenRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mail::mail_open;
use sqlx::SqlitePool;

#[put("MailOpen")]
async fn mail_open_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MailOpenRequest>("MailOpen", &body).map_err(|e| {
        tracing::warn!("Failed to parse MailOpen: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = mail_open::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
