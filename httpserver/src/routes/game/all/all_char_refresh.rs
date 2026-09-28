use actix_web::{HttpResponse, Result, put, web};
use bd2::proto::proto_net::AllCharRefreshRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::all::all_char_refresh;
use sqlx::SqlitePool;

#[put("AllCharRefresh")]
async fn all_char_refresh_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = parse_packet::<AllCharRefreshRequest>("AllCharRefresh", &body).map_err(|e| {
        tracing::warn!("Failed to parse AllCharRefresh: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = all_char_refresh::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
