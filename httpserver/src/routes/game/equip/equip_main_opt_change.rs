use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipMainOptChangeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_main_opt_change;
use sqlx::SqlitePool;

#[put("EquipMainOptChange")]
async fn equip_main_opt_change_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EquipMainOptChangeRequest>("EquipMainOptChange", &body).map_err(|e| {
        tracing::warn!("Failed to parse EquipMainOptChange: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = equip_main_opt_change::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
