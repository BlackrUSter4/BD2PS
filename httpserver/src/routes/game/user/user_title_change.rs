use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::UserTitleChangeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::user::user_title_change;
use sqlx::SqlitePool;

#[put("UserTitleChange")]
async fn user_title_change_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<UserTitleChangeRequest>("UserTitleChange", &body).map_err(|e| {
        tracing::warn!("Failed to parse UserTitleChange: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = user_title_change::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
