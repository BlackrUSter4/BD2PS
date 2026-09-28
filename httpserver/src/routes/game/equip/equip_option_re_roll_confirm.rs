use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipOptionReRollConfirmRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_option_re_roll_confirm;
use sqlx::SqlitePool;

#[put("EquipOptionReRollConfirm")]
async fn equip_option_re_roll_confirm_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EquipOptionReRollConfirmRequest>("EquipOptionReRollConfirm", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse EquipOptionReRollConfirm: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = equip_option_re_roll_confirm::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
