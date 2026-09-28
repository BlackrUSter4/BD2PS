use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MailHistoryInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mail::mail_history_info;
use sqlx::SqlitePool;

#[put("MailHistoryInfo")]
async fn mail_history_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MailHistoryInfoRequest>("MailHistoryInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse MailHistoryInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = mail_history_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
