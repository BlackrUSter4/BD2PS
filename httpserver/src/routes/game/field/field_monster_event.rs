use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FieldMonsterEventRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::field::field_monster_event;
use sqlx::SqlitePool;

#[put("FieldMonsterEvent")]
async fn field_monster_event_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<FieldMonsterEventRequest>("FieldMonsterEvent", &body).map_err(|e| {
            tracing::warn!("Failed to parse FieldMonsterEvent: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = field_monster_event::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
