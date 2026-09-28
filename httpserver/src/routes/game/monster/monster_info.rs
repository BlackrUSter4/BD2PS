use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MonsterInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::monster::monster_info;
use sqlx::SqlitePool;

#[put("MonsterInfo")]
async fn monster_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = parse_packet::<MonsterInfoRequest>("MonsterInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse MonsterInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = monster_info::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
