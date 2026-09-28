use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EventRewardHistoryRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::event::event_reward_history;
use sqlx::SqlitePool;

#[put("EventRewardHistory")]
async fn event_reward_history_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req =
        parse_packet::<EventRewardHistoryRequest>("EventRewardHistory", &body).map_err(|e| {
            tracing::warn!("Failed to parse EventRewardHistory: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;

    let response = event_reward_history::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
