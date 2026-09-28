use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ReportUserRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::report::report_user;
use sqlx::SqlitePool;

#[put("ReportUser")]
async fn report_user_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ReportUserRequest>("ReportUser", &body).map_err(|e| {
        tracing::warn!("Failed to parse ReportUser: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = report_user::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
