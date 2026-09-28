use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildSupporterAddRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_supporter_add;
use sqlx::SqlitePool;

#[put("GuildSupporterAdd")]
async fn guild_supporter_add_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<GuildSupporterAddRequest>("GuildSupporterAdd", &body).map_err(|e| {
            tracing::warn!("Failed to parse GuildSupporterAdd: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = guild_supporter_add::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
