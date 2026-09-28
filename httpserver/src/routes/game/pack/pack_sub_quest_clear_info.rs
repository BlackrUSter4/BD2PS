use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PackSubQuestClearInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pack::pack_sub_quest_clear_info;
use sqlx::SqlitePool;

#[put("PackSubQuestClearInfo")]
async fn pack_sub_quest_clear_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PackSubQuestClearInfoRequest>("PackSubQuestClearInfo", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse PackSubQuestClearInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = pack_sub_quest_clear_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
