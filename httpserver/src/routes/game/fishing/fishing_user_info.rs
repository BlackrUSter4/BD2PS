use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingUserInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_user_info;
use sqlx::SqlitePool;

#[put("FishingUserInfo")]
async fn fishing_user_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingUserInfoRequest>("FishingUserInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingUserInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_user_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
