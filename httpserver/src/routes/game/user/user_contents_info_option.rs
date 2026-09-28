use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::UserContentsInfoOptionRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::user::user_contents_info_option;
use sqlx::SqlitePool;

#[put("UserContentsInfoOption")]
async fn user_contents_info_option_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<UserContentsInfoOptionRequest>("UserContentsInfoOption", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse UserContentsInfoOption: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = user_contents_info_option::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
