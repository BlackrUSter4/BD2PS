use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildNameChangeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_name_change;
use sqlx::SqlitePool;

#[put("GuildNameChange")]
async fn guild_name_change_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildNameChangeRequest>("GuildNameChange", &body).map_err(|e| {
        tracing::warn!("Failed to parse GuildNameChange: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = guild_name_change::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
