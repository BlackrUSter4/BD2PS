use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_info;
use sqlx::SqlitePool;

#[put("EquipInfo")]
async fn equip_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = parse_packet::<EquipInfoRequest>("ChargeCostInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse ChargeCostInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = equip_info::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
