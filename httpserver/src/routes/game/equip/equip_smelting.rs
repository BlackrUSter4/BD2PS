use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipSmeltingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_smelting;
use sqlx::SqlitePool;

#[put("EquipSmelting")]
async fn equip_smelting_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EquipSmeltingRequest>("EquipSmelting", &body).map_err(|e| {
        tracing::warn!("Failed to parse EquipSmelting: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = equip_smelting::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
