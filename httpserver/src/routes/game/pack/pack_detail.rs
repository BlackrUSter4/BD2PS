use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PackDetailRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pack::pack_detail;
use sqlx::SqlitePool;

#[put("PackDetail")]
async fn pack_detail_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PackDetailRequest>("PackDetail", &body).map_err(|e| {
        tracing::warn!("Failed to parse PackDetail: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = pack_detail::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
