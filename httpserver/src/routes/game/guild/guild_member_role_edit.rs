use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildMemberRoleEditRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_member_role_edit;
use sqlx::SqlitePool;

#[put("GuildMemberRoleEdit")]
async fn guild_member_role_edit_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<GuildMemberRoleEditRequest>("GuildMemberRoleEdit", &body).map_err(|e| {
            tracing::warn!("Failed to parse GuildMemberRoleEdit: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = guild_member_role_edit::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
