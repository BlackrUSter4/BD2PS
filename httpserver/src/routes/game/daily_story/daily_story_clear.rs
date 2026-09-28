use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::DailyStoryClearRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::daily_story::daily_story_clear;
use sqlx::SqlitePool;

#[put("DailyStoryClear")]
async fn daily_story_clear_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<DailyStoryClearRequest>("DailyStoryClear", &body).map_err(|e| {
        tracing::warn!("Failed to parse DailyStoryClear: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = daily_story_clear::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
