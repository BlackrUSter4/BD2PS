use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::HuntingGroundInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::hunting::hunting_ground_info;
use sqlx::SqlitePool;

#[put("HuntingGroundInfo")]
async fn hunting_ground_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req =
        parse_packet::<HuntingGroundInfoRequest>("HuntingGroundInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse HuntingGroundInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;

    let response = hunting_ground_info::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
