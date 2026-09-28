use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildJoinRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_join;
use sqlx::SqlitePool;

#[put("GuildJoin")]
async fn guild_join_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildJoinRequest>("GuildJoin", &body).map_err(|e| {
        tracing::warn!("Failed to parse GuildJoin: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = guild_join::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
