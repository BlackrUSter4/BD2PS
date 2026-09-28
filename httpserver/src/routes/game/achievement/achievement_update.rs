use actix_web::{HttpResponse, Result, put, web};
use bd2::proto::proto_net::AchievementUpdateRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::achievement::achievement_update;
use sqlx::SqlitePool;

#[put("AchievementUpdate")]
async fn achievement_update_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<AchievementUpdateRequest>("AchievementUpdate", &body).map_err(|e| {
            tracing::warn!("Failed to parse AchievementUpdate: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = achievement_update::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
