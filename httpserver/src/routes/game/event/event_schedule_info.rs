use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EventScheduleInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::event::event_schedule_info;
use sqlx::SqlitePool;

#[put("EventScheduleInfo")]
async fn event_schedule_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<EventScheduleInfoRequest>("EventScheduleInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse EventScheduleInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = event_schedule_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
