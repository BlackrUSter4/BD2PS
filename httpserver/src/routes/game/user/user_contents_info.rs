use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::UserContentsInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::user::user_contents_info;
use sqlx::SqlitePool;

#[put("UserContentsInfo")]
async fn user_contents_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<UserContentsInfoRequest>("UserContentsInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse UserContentsInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = user_contents_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
