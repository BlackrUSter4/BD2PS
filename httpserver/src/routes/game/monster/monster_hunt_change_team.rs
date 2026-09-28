use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MonsterHuntChangeTeamRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::monster::monster_hunt_change_team;
use sqlx::SqlitePool;

#[put("MonsterHuntChangeTeam")]
async fn monster_hunt_change_team_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MonsterHuntChangeTeamRequest>("MonsterHuntChangeTeam", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse MonsterHuntChangeTeam: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = monster_hunt_change_team::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
