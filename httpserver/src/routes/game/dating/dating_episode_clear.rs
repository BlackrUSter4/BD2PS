use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::DatingEpisodeClearRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::dating::dating_episode_clear;
use sqlx::SqlitePool;

#[put("DatingEpisodeClear")]
async fn dating_episode_clear_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<DatingEpisodeClearRequest>("DatingEpisodeClear", &body).map_err(|e| {
            tracing::warn!("Failed to parse DatingEpisodeClear: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = dating_episode_clear::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
