use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MonsterHuntDeckSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::monster::monster_hunt_deck_save;
use sqlx::SqlitePool;

#[put("MonsterHuntDeckSave")]
async fn monster_hunt_deck_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<MonsterHuntDeckSaveRequest>("MonsterHuntDeckSave", &body).map_err(|e| {
            tracing::warn!("Failed to parse MonsterHuntDeckSave: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = monster_hunt_deck_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
