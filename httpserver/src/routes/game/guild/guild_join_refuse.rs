use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildJoinRefuseRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_join_refuse;
use sqlx::SqlitePool;

#[put("GuildJoinRefuse")]
async fn guild_join_refuse_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildJoinRefuseRequest>("GuildJoinRefuse", &body).map_err(|e| {
        tracing::warn!("Failed to parse GuildJoinRefuse: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = guild_join_refuse::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
