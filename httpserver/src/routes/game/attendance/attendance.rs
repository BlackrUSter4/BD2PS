use actix_web::{HttpResponse, Result, put, web};
use bd2::proto::proto_net::AttendanceRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::attendance;
use sqlx::SqlitePool;

#[put("Attendance")]
async fn attendance_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = parse_packet::<AttendanceRequest>("Attendance", &body).map_err(|e| {
        tracing::warn!("Failed to parse Attendance: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = attendance::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
