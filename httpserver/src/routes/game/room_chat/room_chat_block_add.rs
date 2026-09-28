use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::RoomChatBlockAddRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::room_chat::room_chat_block_add;
use sqlx::SqlitePool;

#[put("RoomChatBlockAdd")]
async fn room_chat_block_add_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<RoomChatBlockAddRequest>("RoomChatBlockAdd", &body).map_err(|e| {
        tracing::warn!("Failed to parse RoomChatBlockAdd: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = room_chat_block_add::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
