use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildSupporterDeleteRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_supporter_delete;
use sqlx::SqlitePool;

#[put("GuildSupporterDelete")]
async fn guild_supporter_delete_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildSupporterDeleteRequest>("GuildSupporterDelete", &body).map_err(
        |e| {
            tracing::warn!("Failed to parse GuildSupporterDelete: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        },
    )?;
    let response = guild_supporter_delete::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
