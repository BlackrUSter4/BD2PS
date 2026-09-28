use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeWorldObjectUnplaceSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_world_object_unplace_save;
use sqlx::SqlitePool;

#[put("LifeWorldObjectUnplaceSave")]
async fn life_world_object_unplace_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeWorldObjectUnplaceSaveRequest>("LifeWorldObjectUnplaceSave", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse LifeWorldObjectUnplaceSave: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = life_world_object_unplace_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
