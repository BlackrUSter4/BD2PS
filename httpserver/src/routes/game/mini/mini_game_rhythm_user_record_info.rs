use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameRhythmUserRecordInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_rhythm_user_record_info;
use sqlx::SqlitePool;

#[put("MiniGameRhythmUserRecordInfo")]
async fn mini_game_rhythm_user_record_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniGameRhythmUserRecordInfoRequest>("MiniGameRhythmUserRecordInfo", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse MiniGameRhythmUserRecordInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = mini_game_rhythm_user_record_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
