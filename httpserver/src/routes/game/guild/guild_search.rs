use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildSearchRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_search;
use sqlx::SqlitePool;

#[put("GuildSearch")]
async fn guild_search_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildSearchRequest>("GuildSearch", &body).map_err(|e| {
        tracing::warn!("Failed to parse GuildSearch: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = guild_search::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
