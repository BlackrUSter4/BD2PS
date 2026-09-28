use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharScoutInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char::char_scout_info;
use sqlx::SqlitePool;

#[put("CharScoutInfo")]
async fn char_scout_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CharScoutInfoRequest>("CharScoutInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse CharScoutInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = char_scout_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
