use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::TutorialClearRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::tutorial::tutorial_clear;
use sqlx::SqlitePool;

#[put("TutorialClear")]
async fn tutorial_clear_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = parse_packet::<TutorialClearRequest>("TutorialClear", &body).map_err(|e| {
        tracing::warn!("Failed to parse TutorialClear: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = tutorial_clear::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
