use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PrestigeSkinInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::prestige::prestige_skin_info;
use sqlx::SqlitePool;

#[put("PrestigeSkinInfo")]
async fn prestige_skin_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PrestigeSkinInfoRequest>("PrestigeSkinInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse PrestigeSkinInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = prestige_skin_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
