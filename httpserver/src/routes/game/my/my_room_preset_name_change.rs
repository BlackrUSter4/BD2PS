use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MyRoomPresetNameChangeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::my::my_room_preset_name_change;
use sqlx::SqlitePool;

#[put("MyRoomPresetNameChange")]
async fn my_room_preset_name_change_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MyRoomPresetNameChangeRequest>("MyRoomPresetNameChange", &body).map_err(|e| {
        tracing::warn!("Failed to parse MyRoomPresetNameChange: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = my_room_preset_name_change::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
