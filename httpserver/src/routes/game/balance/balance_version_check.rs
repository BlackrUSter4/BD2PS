use actix_web::{HttpResponse, Result, put, web};
use bd2::proto::proto_net::BalanceVersionCheckRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::balance::balance_version_check;
use sqlx::SqlitePool;

#[put("BalanceVersionCheck")]
async fn balance_version_check_handler(
    _pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let _uid = *user_id;
    let req =
        parse_packet::<BalanceVersionCheckRequest>("BalanceVersionCheck", &body).map_err(|e| {
            tracing::warn!("Failed to parse BalanceVersionCheck: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = balance_version_check::handle(req).await;
    Ok(HttpResponse::Ok().json(response))
}
