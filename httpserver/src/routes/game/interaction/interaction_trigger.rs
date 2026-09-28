use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::InteractionTriggerRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::interaction::interaction_trigger;
use sqlx::SqlitePool;

#[put("InteractionTrigger")]
async fn interaction_trigger_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<InteractionTriggerRequest>("InteractionTrigger", &body).map_err(|e| {
            tracing::warn!("Failed to parse InteractionTrigger: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = interaction_trigger::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
