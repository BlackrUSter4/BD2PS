use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SaveUserPositionRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::save::save_user_position;
use sqlx::SqlitePool;

#[put("SaveUserPosition")]
async fn save_user_position_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = parse_packet::<SaveUserPositionRequest>("SaveUserPosition", &body).map_err(|e| {
        tracing::warn!("Failed to parse SaveUserPosition: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = save_user_position::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
