use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipSequenceUpgradeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_sequence_upgrade;
use sqlx::SqlitePool;

#[put("EquipSequenceUpgrade")]
async fn equip_sequence_upgrade_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EquipSequenceUpgradeRequest>("EquipSequenceUpgrade", &body).map_err(
        |e| {
            tracing::warn!("Failed to parse EquipSequenceUpgrade: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        },
    )?;
    let response = equip_sequence_upgrade::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
