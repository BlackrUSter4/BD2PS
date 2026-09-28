use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharVoteFavoriteAddRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char_vote::char_vote_favorite_add;
use sqlx::SqlitePool;

#[put("CharVoteFavoriteAdd")]
async fn char_vote_favorite_add_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CharVoteFavoriteAddRequest>("CharVoteFavoriteAdd", &body).map_err(|e| {
        tracing::warn!("Failed to parse CharVoteFavoriteAdd: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = char_vote_favorite_add::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
