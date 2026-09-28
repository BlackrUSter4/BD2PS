use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipMakingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_making;
use sqlx::SqlitePool;

#[put("EquipMaking")]
async fn equip_making_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EquipMakingRequest>("EquipMaking", &body).map_err(|e| {
        tracing::warn!("Failed to parse EquipMaking: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = equip_making::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
