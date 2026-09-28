use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeWorldChoiceRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_world_choice;
use sqlx::SqlitePool;

#[put("LifeWorldChoice")]
async fn life_world_choice_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeWorldChoiceRequest>("LifeWorldChoice", &body).map_err(|e| {
        tracing::warn!("Failed to parse LifeWorldChoice: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = life_world_choice::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
