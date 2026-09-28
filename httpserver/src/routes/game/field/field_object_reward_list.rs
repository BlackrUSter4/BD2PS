use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FieldObjectRewardListRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::field::field_object_reward_list;
use sqlx::SqlitePool;

#[put("FieldObjectRewardList")]
async fn field_object_reward_list_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FieldObjectRewardListRequest>("FieldObjectRewardList", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse FieldObjectRewardList: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = field_object_reward_list::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
