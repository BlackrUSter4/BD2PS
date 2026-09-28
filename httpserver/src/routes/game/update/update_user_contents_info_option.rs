use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::UpdateUserContentsInfoOptionRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::update::update_user_contents_info_option;
use sqlx::SqlitePool;

#[put("UpdateUserContentsInfoOption")]
async fn update_user_contents_info_option_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<UpdateUserContentsInfoOptionRequest>("UpdateUserContentsInfoOption", &body)
            .map_err(|e| {
                tracing::warn!("Failed to parse UpdateUserContentsInfoOption: {}", e);
                actix_web::error::ErrorBadRequest("Invalid packet")
            })?;
    let response = update_user_contents_info_option::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
