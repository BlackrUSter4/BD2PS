use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::HuntingGroundEnterRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::hunting::hunting_ground_enter;
use sqlx::SqlitePool;

#[put("HuntingGroundEnter")]
async fn hunting_ground_enter_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<HuntingGroundEnterRequest>("HuntingGroundEnter", &body).map_err(|e| {
            tracing::warn!("Failed to parse HuntingGroundEnter: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = hunting_ground_enter::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
