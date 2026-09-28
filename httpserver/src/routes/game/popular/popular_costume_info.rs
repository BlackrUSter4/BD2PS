use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PopularCostumeInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::popular::popular_costume_info;
use sqlx::SqlitePool;

#[put("PopularCostumeInfo")]
async fn popular_costume_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<PopularCostumeInfoRequest>("PopularCostumeInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse PopularCostumeInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = popular_costume_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
