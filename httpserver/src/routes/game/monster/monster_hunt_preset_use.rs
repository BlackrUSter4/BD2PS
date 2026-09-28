use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MonsterHuntPresetUseRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::monster::monster_hunt_preset_use;
use sqlx::SqlitePool;

#[put("MonsterHuntPresetUse")]
async fn monster_hunt_preset_use_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MonsterHuntPresetUseRequest>("MonsterHuntPresetUse", &body).map_err(
        |e| {
            tracing::warn!("Failed to parse MonsterHuntPresetUse: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        },
    )?;
    let response = monster_hunt_preset_use::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
