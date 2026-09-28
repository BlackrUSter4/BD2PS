use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SaveFieldCharControlDeckTypeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::save::save_field_char_control_deck_type;
use sqlx::SqlitePool;

#[put("SaveFieldCharControlDeckType")]
async fn save_field_char_control_deck_type_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<SaveFieldCharControlDeckTypeRequest>("SaveFieldCharControlDeckType", &body)
            .map_err(|e| {
                tracing::warn!("Failed to parse SaveFieldCharControlDeckType: {}", e);
                actix_web::error::ErrorBadRequest("Invalid packet")
            })?;
    let response = save_field_char_control_deck_type::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
