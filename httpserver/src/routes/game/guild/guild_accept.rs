use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildAcceptRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_accept;
use sqlx::SqlitePool;

#[put("GuildAccept")]
async fn guild_accept_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildAcceptRequest>("GuildAccept", &body).map_err(|e| {
        tracing::warn!("Failed to parse GuildAccept: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = guild_accept::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
