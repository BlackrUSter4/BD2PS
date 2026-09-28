use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MyRoomShareOptionUpdateRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::my::my_room_share_option_update;
use sqlx::SqlitePool;

#[put("MyRoomShareOptionUpdate")]
async fn my_room_share_option_update_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MyRoomShareOptionUpdateRequest>("MyRoomShareOptionUpdate", &body).map_err(|e| {
        tracing::warn!("Failed to parse MyRoomShareOptionUpdate: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = my_room_share_option_update::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
