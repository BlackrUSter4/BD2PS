use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::TalentSkillUpgradeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::talent::talent_skill_upgrade;
use sqlx::SqlitePool;

#[put("TalentSkillUpgrade")]
async fn talent_skill_upgrade_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<TalentSkillUpgradeRequest>("TalentSkillUpgrade", &body).map_err(|e| {
            tracing::warn!("Failed to parse TalentSkillUpgrade: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = talent_skill_upgrade::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
