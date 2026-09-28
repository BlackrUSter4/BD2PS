use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::DailyStoryInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::daily_story::daily_story_info;
use sqlx::SqlitePool;

#[put("DailyStoryInfo")]
async fn daily_story_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<DailyStoryInfoRequest>("DailyStoryInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse DailyStoryInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = daily_story_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
