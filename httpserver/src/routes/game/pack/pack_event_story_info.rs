use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PackEventStoryInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pack::pack_event_story_info;
use sqlx::SqlitePool;

#[put("PackEventStoryInfo")]
async fn pack_event_story_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req =
        parse_packet::<PackEventStoryInfoRequest>("PackEventStoryInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse PackEventStoryInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;

    let response = pack_event_story_info::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
