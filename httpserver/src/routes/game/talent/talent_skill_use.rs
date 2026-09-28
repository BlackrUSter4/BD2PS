use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::TalentSkillUseRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::talent::talent_skill_use;
use sqlx::SqlitePool;

#[put("TalentSkillUse")]
async fn talent_skill_use_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<TalentSkillUseRequest>("TalentSkillUse", &body).map_err(|e| {
        tracing::warn!("Failed to parse TalentSkillUse: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = talent_skill_use::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
