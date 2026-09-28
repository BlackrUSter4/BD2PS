use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::TotalWarInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::total::total_war_info;
use sqlx::SqlitePool;

#[put("TotalWarInfo")]
async fn total_war_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<TotalWarInfoRequest>("TotalWarInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse TotalWarInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = total_war_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
