use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CostumeNodeActivationRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::costume::costume_node_activation;
use sqlx::SqlitePool;

#[put("CostumeNodeActivation")]
async fn costume_node_activation_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CostumeNodeActivationRequest>("CostumeNodeActivation", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse CostumeNodeActivation: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = costume_node_activation::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
