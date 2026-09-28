use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildJoinSendInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_join_send_info;
use sqlx::SqlitePool;

#[put("GuildJoinSendInfo")]
async fn guild_join_send_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<GuildJoinSendInfoRequest>("GuildJoinSendInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse GuildJoinSendInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = guild_join_send_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
