use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::BattleEndTestRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::battle::battle_end_test;
use sqlx::SqlitePool;

#[put("BattleEndTest")]
async fn battle_end_test_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<BattleEndTestRequest>("BattleEndTest", &body).map_err(|e| {
        tracing::warn!("Failed to parse BattleEndTest: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = battle_end_test::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
