use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ScheduleInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::schedule::schedule_info;
use sqlx::SqlitePool;

#[put("ScheduleInfo")]
async fn schedule_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = parse_packet::<ScheduleInfoRequest>("ScheduleInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse ScheduleInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = schedule_info::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
