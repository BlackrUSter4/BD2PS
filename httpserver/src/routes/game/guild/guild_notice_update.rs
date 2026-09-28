use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildNoticeUpdateRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_notice_update;
use sqlx::SqlitePool;

#[put("GuildNoticeUpdate")]
async fn guild_notice_update_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<GuildNoticeUpdateRequest>("GuildNoticeUpdate", &body).map_err(|e| {
            tracing::warn!("Failed to parse GuildNoticeUpdate: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = guild_notice_update::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
