use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::AvatarInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::avatar::avatar_info;
use sqlx::SqlitePool;

#[put("AvatarInfo")]
async fn avatar_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<AvatarInfoRequest>("AvatarInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse AvatarInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = avatar_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
