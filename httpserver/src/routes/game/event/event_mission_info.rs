use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EventMissionInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::event::event_mission_info;
use sqlx::SqlitePool;

#[put("EventMissionInfo")]
async fn event_mission_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = parse_packet::<EventMissionInfoRequest>("EventMissionInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse EventMissionInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = event_mission_info::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
