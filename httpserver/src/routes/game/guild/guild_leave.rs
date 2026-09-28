use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildLeaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_leave;
use sqlx::SqlitePool;

#[put("GuildLeave")]
async fn guild_leave_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildLeaveRequest>("GuildLeave", &body).map_err(|e| {
        tracing::warn!("Failed to parse GuildLeave: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = guild_leave::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
