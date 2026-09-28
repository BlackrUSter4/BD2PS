use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::TalentSlotSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::talent::talent_slot_save;
use sqlx::SqlitePool;

#[put("TalentSlotSave")]
async fn talent_slot_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<TalentSlotSaveRequest>("TalentSlotSave", &body).map_err(|e| {
        tracing::warn!("Failed to parse TalentSlotSave: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = talent_slot_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
