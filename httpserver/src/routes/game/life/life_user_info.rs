use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeUserInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_user_info;
use sqlx::SqlitePool;

#[put("LifeUserInfo")]
async fn life_user_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeUserInfoRequest>("LifeUserInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse LifeUserInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = life_user_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
