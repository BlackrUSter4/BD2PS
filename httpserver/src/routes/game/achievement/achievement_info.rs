use actix_web::{HttpResponse, Result, put, web};
use bd2::proto::proto_net::AchievementInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::achievement::achievement_info;
use sqlx::SqlitePool;

#[put("AchievementInfo")]
async fn achievement_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<AchievementInfoRequest>("AchievementInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse AchievementInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = achievement_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
