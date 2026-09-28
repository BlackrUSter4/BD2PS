use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::RoomChatReportAndBlockInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::room_chat::room_chat_report_and_block_info;
use sqlx::SqlitePool;

#[put("RoomChatReportAndBlockInfo")]
async fn room_chat_report_and_block_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<RoomChatReportAndBlockInfoRequest>("RoomChatReportAndBlockInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse RoomChatReportAndBlockInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = room_chat_report_and_block_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
