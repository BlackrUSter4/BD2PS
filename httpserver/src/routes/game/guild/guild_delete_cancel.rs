use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildDeleteCancelRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_delete_cancel;
use sqlx::SqlitePool;

#[put("GuildDeleteCancel")]
async fn guild_delete_cancel_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<GuildDeleteCancelRequest>("GuildDeleteCancel", &body).map_err(|e| {
            tracing::warn!("Failed to parse GuildDeleteCancel: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = guild_delete_cancel::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
