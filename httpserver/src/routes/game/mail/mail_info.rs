use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MailInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mail::mail_info;
use sqlx::SqlitePool;

#[put("MailInfo")]
async fn mail_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MailInfoRequest>("MailInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse MailInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = mail_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
