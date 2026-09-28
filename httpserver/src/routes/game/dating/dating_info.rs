use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::DatingInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::dating::dating_info;
use sqlx::SqlitePool;

#[put("DatingInfo")]
async fn dating_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<DatingInfoRequest>("DatingInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse DatingInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = dating_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
