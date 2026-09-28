use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PackInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pack::pack_info;
use sqlx::SqlitePool;

#[put("PackInfo")]
async fn pack_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PackInfoRequest>("PackInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse PackInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = pack_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
