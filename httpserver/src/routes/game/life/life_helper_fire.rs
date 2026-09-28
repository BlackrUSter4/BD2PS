use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeHelperFireRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_helper_fire;
use sqlx::SqlitePool;

#[put("LifeHelperFire")]
async fn life_helper_fire_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeHelperFireRequest>("LifeHelperFire", &body).map_err(|e| {
        tracing::warn!("Failed to parse LifeHelperFire: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = life_helper_fire::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
