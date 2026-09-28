use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeWorldObjectPositionSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_world_object_position_save;
use sqlx::SqlitePool;

#[put("LifeWorldObjectPositionSave")]
async fn life_world_object_position_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<LifeWorldObjectPositionSaveRequest>("LifeWorldObjectPositionSave", &body)
            .map_err(|e| {
                tracing::warn!("Failed to parse LifeWorldObjectPositionSave: {}", e);
                actix_web::error::ErrorBadRequest("Invalid packet")
            })?;
    let response = life_world_object_position_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
