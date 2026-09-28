use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::AvatarSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::avatar::avatar_save;
use sqlx::SqlitePool;

#[put("AvatarSave")]
async fn avatar_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<AvatarSaveRequest>("AvatarSave", &body).map_err(|e| {
        tracing::warn!("Failed to parse AvatarSave: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = avatar_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
