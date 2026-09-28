use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MyRoomSearchGuildRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::my::my_room_search_guild;
use sqlx::SqlitePool;

#[put("MyRoomSearchGuild")]
async fn my_room_search_guild_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<MyRoomSearchGuildRequest>("MyRoomSearchGuild", &body).map_err(|e| {
            tracing::warn!("Failed to parse MyRoomSearchGuild: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = my_room_search_guild::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
