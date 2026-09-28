use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::UserRelayInfoUpdateRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::user::user_relay_info_update;
use sqlx::SqlitePool;

#[put("UserRelayInfoUpdate")]
async fn user_relay_info_update_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<UserRelayInfoUpdateRequest>("UserRelayInfoUpdate", &body).map_err(|e| {
        tracing::warn!("Failed to parse UserRelayInfoUpdate: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = user_relay_info_update::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
