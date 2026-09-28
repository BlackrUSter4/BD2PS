use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharVoteInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char_vote::char_vote_info;
use sqlx::SqlitePool;

#[put("CharVoteInfo")]
async fn char_vote_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CharVoteInfoRequest>("CharVoteInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse CharVoteInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = char_vote_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
