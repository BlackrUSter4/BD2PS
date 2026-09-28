use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeEatFoodRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_eat_food;
use sqlx::SqlitePool;

#[put("LifeEatFood")]
async fn life_eat_food_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeEatFoodRequest>("LifeEatFood", &body).map_err(|e| {
        tracing::warn!("Failed to parse LifeEatFood: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = life_eat_food::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
