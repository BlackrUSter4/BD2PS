use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipOptionReRollRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_option_re_roll;
use sqlx::SqlitePool;

#[put("EquipOptionReRoll")]
async fn equip_option_re_roll_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<EquipOptionReRollRequest>("EquipOptionReRoll", &body).map_err(|e| {
            tracing::warn!("Failed to parse EquipOptionReRoll: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = equip_option_re_roll::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
