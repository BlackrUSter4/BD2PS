use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::IdCardSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::id::id_card_save;
use sqlx::SqlitePool;

#[put("IdCardSave")]
async fn id_card_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<IdCardSaveRequest>("IdCardSave", &body).map_err(|e| {
        tracing::warn!("Failed to parse IdCardSave: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = id_card_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
