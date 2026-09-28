use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CashMailInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::cash::cash_mail_info;
use sqlx::SqlitePool;

#[put("CashMailInfo")]
async fn cash_mail_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CashMailInfoRequest>("CashMailInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse CashMailInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = cash_mail_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
