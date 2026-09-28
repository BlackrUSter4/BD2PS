use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeCitizenAvatarSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_citizen_avatar_save;
use sqlx::SqlitePool;

#[put("LifeCitizenAvatarSave")]
async fn life_citizen_avatar_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeCitizenAvatarSaveRequest>("LifeCitizenAvatarSave", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse LifeCitizenAvatarSave: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = life_citizen_avatar_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
