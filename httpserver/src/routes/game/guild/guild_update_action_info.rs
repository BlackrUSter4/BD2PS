use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildUpdateActionInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_update_action_info;
use sqlx::SqlitePool;

#[put("GuildUpdateActionInfo")]
async fn guild_update_action_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildUpdateActionInfoRequest>("GuildUpdateActionInfo", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse GuildUpdateActionInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = guild_update_action_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
