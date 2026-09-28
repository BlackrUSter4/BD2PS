use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MyRoomSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::my::my_room_save;
use sqlx::SqlitePool;

#[put("MyRoomSave")]
async fn my_room_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MyRoomSaveRequest>("MyRoomSave", &body).map_err(|e| {
        tracing::warn!("Failed to parse MyRoomSave: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = my_room_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
