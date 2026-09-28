use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SkyWayInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::sky::sky_way_info;
use sqlx::SqlitePool;

#[put("SkyWayInfo")]
async fn sky_way_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<SkyWayInfoRequest>("SkyWayInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse SkyWayInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = sky_way_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
