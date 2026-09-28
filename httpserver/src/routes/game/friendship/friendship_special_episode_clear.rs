use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FriendshipSpecialEpisodeClearRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::friendship::friendship_special_episode_clear;
use sqlx::SqlitePool;

#[put("FriendshipSpecialEpisodeClear")]
async fn friendship_special_episode_clear_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FriendshipSpecialEpisodeClearRequest>("FriendshipSpecialEpisodeClear", &body).map_err(|e| {
        tracing::warn!("Failed to parse FriendshipSpecialEpisodeClear: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = friendship_special_episode_clear::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
