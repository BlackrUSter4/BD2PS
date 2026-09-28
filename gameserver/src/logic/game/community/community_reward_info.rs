use bd2::prost::Message;
use bd2::proto::proto_net::{CommunityRewardDbInfo, CommunityRewardInfoRequest, CommunityRewardInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::community::community_reward_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account claimed community-reward list (was silently returning an empty response
/// with no stub markers despite a real, already-scaffolded table sitting unused).
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: CommunityRewardInfoRequest,
) -> GameResponse {
    info!("Handling CommunityRewardInfoRequest: {:?}", req);

    let community_reward_info = db::get_community_reward_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| CommunityRewardDbInfo { r#type: r.r#type, sub_type: r.sub_type })
        .collect();

    let response = CommunityRewardInfoResponse { community_reward_info };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::CommunityRewardInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
