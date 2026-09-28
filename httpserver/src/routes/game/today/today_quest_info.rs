use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::TodayQuestInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::today::today_quest_info;
use sqlx::SqlitePool;

#[put("TodayQuestInfo")]
async fn today_quest_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = parse_packet::<TodayQuestInfoRequest>("TodayQuestInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse TodayQuestInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = today_quest_info::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
