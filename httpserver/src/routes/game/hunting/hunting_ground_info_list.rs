use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::HuntingGroundInfoListRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::hunting::hunting_ground_info_list;
use sqlx::SqlitePool;

#[put("HuntingGroundInfoList")]
async fn hunting_ground_info_list_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = parse_packet::<HuntingGroundInfoListRequest>("HuntingGroundInfoList", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse HuntingGroundInfoList: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;

    let response = hunting_ground_info_list::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
