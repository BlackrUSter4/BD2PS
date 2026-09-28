use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MonsterHuntPresetSlotAddRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::monster::monster_hunt_preset_slot_add;
use sqlx::SqlitePool;

#[put("MonsterHuntPresetSlotAdd")]
async fn monster_hunt_preset_slot_add_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MonsterHuntPresetSlotAddRequest>("MonsterHuntPresetSlotAdd", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse MonsterHuntPresetSlotAdd: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = monster_hunt_preset_slot_add::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
