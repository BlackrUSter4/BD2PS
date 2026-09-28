use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipSequenceSmeltingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_sequence_smelting;
use sqlx::SqlitePool;

#[put("EquipSequenceSmelting")]
async fn equip_sequence_smelting_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EquipSequenceSmeltingRequest>("EquipSequenceSmelting", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse EquipSequenceSmelting: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = equip_sequence_smelting::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
