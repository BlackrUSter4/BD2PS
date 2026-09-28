use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SupporterDetailRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::supporter::supporter_detail;
use sqlx::SqlitePool;

#[put("SupporterDetail")]
async fn supporter_detail_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<SupporterDetailRequest>("SupporterDetail", &body).map_err(|e| {
        tracing::warn!("Failed to parse SupporterDetail: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = supporter_detail::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
