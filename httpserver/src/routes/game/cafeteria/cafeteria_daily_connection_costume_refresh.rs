use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CafeteriaDailyConnectionCostumeRefreshRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::cafeteria::cafeteria_daily_connection_costume_refresh;
use sqlx::SqlitePool;

#[put("CafeteriaDailyConnectionCostumeRefresh")]
async fn cafeteria_daily_connection_costume_refresh_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CafeteriaDailyConnectionCostumeRefreshRequest>(
        "CafeteriaDailyConnectionCostumeRefresh",
        &body,
    )
    .map_err(|e| {
        tracing::warn!(
            "Failed to parse CafeteriaDailyConnectionCostumeRefresh: {}",
            e
        );
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = cafeteria_daily_connection_costume_refresh::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
