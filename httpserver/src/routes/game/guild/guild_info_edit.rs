use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildInfoEditRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_info_edit;
use sqlx::SqlitePool;

#[put("GuildInfoEdit")]
async fn guild_info_edit_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildInfoEditRequest>("GuildInfoEdit", &body).map_err(|e| {
        tracing::warn!("Failed to parse GuildInfoEdit: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = guild_info_edit::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
