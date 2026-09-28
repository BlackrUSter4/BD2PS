use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_info;
use sqlx::SqlitePool;

#[put("GuildInfo")]
async fn guild_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildInfoRequest>("GuildInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse GuildInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = guild_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
