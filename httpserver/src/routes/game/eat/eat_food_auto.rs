use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EatFoodAutoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::eat::eat_food_auto;
use sqlx::SqlitePool;

#[put("EatFoodAuto")]
async fn eat_food_auto_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EatFoodAutoRequest>("EatFoodAuto", &body).map_err(|e| {
        tracing::warn!("Failed to parse EatFoodAuto: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = eat_food_auto::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
