use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::IbDeckSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::ib::ib_deck_save;
use sqlx::SqlitePool;

#[put("IbDeckSave")]
async fn ib_deck_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<IbDeckSaveRequest>("IbDeckSave", &body).map_err(|e| {
        tracing::warn!("Failed to parse IbDeckSave: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = ib_deck_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
