use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FieldMonsterRegenRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::field::field_monster_regen;
use sqlx::SqlitePool;

#[put("FieldMonsterRegen")]
async fn field_monster_regen_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<FieldMonsterRegenRequest>("FieldMonsterRegen", &body).map_err(|e| {
            tracing::warn!("Failed to parse FieldMonsterRegen: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = field_monster_regen::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
