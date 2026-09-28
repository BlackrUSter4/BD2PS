use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ChatSettingUpdateRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::chat::chat_setting_update;
use sqlx::SqlitePool;

#[put("ChatSettingUpdate")]
async fn chat_setting_update_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ChatSettingUpdateRequest>("ChatSettingUpdate", &body).map_err(|e| {
        tracing::warn!("Failed to parse ChatSettingUpdate: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = chat_setting_update::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
