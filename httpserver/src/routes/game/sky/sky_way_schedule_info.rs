use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SkyWayScheduleInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::sky::sky_way_schedule_info;
use sqlx::SqlitePool;

#[put("SkyWayScheduleInfo")]
async fn sky_way_schedule_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<SkyWayScheduleInfoRequest>("SkyWayScheduleInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse SkyWayScheduleInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = sky_way_schedule_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
