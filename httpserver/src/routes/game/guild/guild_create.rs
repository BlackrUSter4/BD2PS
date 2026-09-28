use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildCreateRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_create;
use sqlx::SqlitePool;

#[put("GuildCreate")]
async fn guild_create_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildCreateRequest>("GuildCreate", &body).map_err(|e| {
        tracing::warn!("Failed to parse GuildCreate: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = guild_create::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
