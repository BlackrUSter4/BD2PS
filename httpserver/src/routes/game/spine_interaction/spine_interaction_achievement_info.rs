use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SpineInteractionAchievementInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::spine_interaction::spine_interaction_achievement_info;
use sqlx::SqlitePool;

#[put("SpineInteractionAchievementInfo")]
async fn spine_interaction_achievement_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<SpineInteractionAchievementInfoRequest>("SpineInteractionAchievementInfo", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse SpineInteractionAchievementInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = spine_interaction_achievement_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
