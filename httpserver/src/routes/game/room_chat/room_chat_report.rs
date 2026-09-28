use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::RoomChatReportRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::room_chat::room_chat_report;
use sqlx::SqlitePool;

#[put("RoomChatReport")]
async fn room_chat_report_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<RoomChatReportRequest>("RoomChatReport", &body).map_err(|e| {
        tracing::warn!("Failed to parse RoomChatReport: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = room_chat_report::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
