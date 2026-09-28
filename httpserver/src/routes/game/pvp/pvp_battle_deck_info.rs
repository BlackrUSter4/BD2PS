use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PvpBattleDeckInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pvp::pvp_battle_deck_info;
use sqlx::SqlitePool;

#[put("PvpBattleDeckInfo")]
async fn pvp_battle_deck_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<PvpBattleDeckInfoRequest>("PvpBattleDeckInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse PvpBattleDeckInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = pvp_battle_deck_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
