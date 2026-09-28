use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniEventHubInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_event_hub_info;
use sqlx::SqlitePool;

#[put("MiniEventHubInfo")]
async fn mini_event_hub_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniEventHubInfoRequest>("MiniEventHubInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse MiniEventHubInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = mini_event_hub_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
