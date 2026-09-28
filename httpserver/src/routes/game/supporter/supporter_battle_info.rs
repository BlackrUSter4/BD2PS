use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SupporterBattleInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::supporter::supporter_battle_info;
use sqlx::SqlitePool;

#[put("SupporterBattleInfo")]
async fn supporter_battle_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<SupporterBattleInfoRequest>("SupporterBattleInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse SupporterBattleInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = supporter_battle_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
