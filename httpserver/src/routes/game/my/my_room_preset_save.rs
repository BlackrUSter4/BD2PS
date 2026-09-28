use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MyRoomPresetSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::my::my_room_preset_save;
use sqlx::SqlitePool;

#[put("MyRoomPresetSave")]
async fn my_room_preset_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MyRoomPresetSaveRequest>("MyRoomPresetSave", &body).map_err(|e| {
        tracing::warn!("Failed to parse MyRoomPresetSave: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = my_room_preset_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
