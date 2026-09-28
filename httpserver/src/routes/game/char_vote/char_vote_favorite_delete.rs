use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharVoteFavoriteDeleteRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char_vote::char_vote_favorite_delete;
use sqlx::SqlitePool;

#[put("CharVoteFavoriteDelete")]
async fn char_vote_favorite_delete_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CharVoteFavoriteDeleteRequest>("CharVoteFavoriteDelete", &body).map_err(|e| {
        tracing::warn!("Failed to parse CharVoteFavoriteDelete: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = char_vote_favorite_delete::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
