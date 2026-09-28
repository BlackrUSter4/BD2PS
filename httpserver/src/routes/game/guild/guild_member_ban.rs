use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildMemberBanRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_member_ban;
use sqlx::SqlitePool;

#[put("GuildMemberBan")]
async fn guild_member_ban_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildMemberBanRequest>("GuildMemberBan", &body).map_err(|e| {
        tracing::warn!("Failed to parse GuildMemberBan: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = guild_member_ban::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
