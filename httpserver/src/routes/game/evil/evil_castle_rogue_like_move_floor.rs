use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EvilCastleRogueLikeMoveFloorRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::evil::evil_castle_rogue_like_move_floor;
use sqlx::SqlitePool;

#[put("EvilCastleRogueLikeMoveFloor")]
async fn evil_castle_rogue_like_move_floor_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<EvilCastleRogueLikeMoveFloorRequest>("EvilCastleRogueLikeMoveFloor", &body)
            .map_err(|e| {
                tracing::warn!("Failed to parse EvilCastleRogueLikeMoveFloor: {}", e);
                actix_web::error::ErrorBadRequest("Invalid packet")
            })?;
    let response = evil_castle_rogue_like_move_floor::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
