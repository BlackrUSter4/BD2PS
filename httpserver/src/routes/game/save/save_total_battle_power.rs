use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SaveTotalBattlePowerRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::save::save_total_battle_power;
use sqlx::SqlitePool;

#[put("SaveTotalBattlePower")]
async fn save_total_battle_power_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = parse_packet::<SaveTotalBattlePowerRequest>("SaveTotalBattlePower", &body).map_err(
        |e| {
            tracing::warn!("Failed to parse SaveTotalBattlePower: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        },
    )?;

    let response = save_total_battle_power::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
