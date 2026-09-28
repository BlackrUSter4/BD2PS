use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeAllUserScoreInfoRequest, EvilCastleRogueLikeAllUserScoreInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::evil_castle_rogue_like_score_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, _uid: i64, req: EvilCastleRogueLikeAllUserScoreInfoRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeAllUserScoreInfoRequest: {:?}", req);

    let total = evil_castle_rogue_like_score_info::sum_all_user_total_score(pool).await.unwrap_or(0);

    let response = EvilCastleRogueLikeAllUserScoreInfoResponse { all_user_total_score: Some(total) };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeAllUserScoreInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
