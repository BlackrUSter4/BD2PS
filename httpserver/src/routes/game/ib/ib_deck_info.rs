use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::IbDeckInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::ib::ib_deck_info;
use sqlx::SqlitePool;

#[put("IbDeckInfo")]
async fn ib_deck_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<IbDeckInfoRequest>("IbDeckInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse IbDeckInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = ib_deck_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
