use bd2::prost::Message;
use bd2::proto::proto_net::{CommunityRewardRequest, CommunityRewardResponse, ItemDbInfo, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{community::community_reward_info as db, item::item_info};
use database::models::game::community::community_reward_info::CommunityRewardInfo;
use sqlx::SqlitePool;
use tracing::info;

const GOLD_ITEM_ID: i32 = 4;
const GOLD_ITEM_TYPE: i32 = 1;

/// Real claim-once tracking via CommunityRewardInfo. No community-reward master catalog was
/// ever captured (e.g. "follow us on socials" type rewards) so a fixed gold placeholder is
/// granted on first claim, same precedent as other uncaptured-reward-table cases.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CommunityRewardRequest) -> GameResponse {
    info!("Handling CommunityRewardRequest: {:?}", req);

    let mut item_infos = Vec::new();
    if let (Some(r#type), Some(sub_type)) = (req.r#type, req.sub_type) {
        if !db::is_claimed(pool, uid, r#type, sub_type).await.unwrap_or(false) {
            let _ = item_info::grant(pool, uid, GOLD_ITEM_ID, GOLD_ITEM_TYPE, 1000).await;
            item_infos.push(ItemDbInfo { id: Some(GOLD_ITEM_ID), r#type: Some(GOLD_ITEM_TYPE), count: Some(1000), ..Default::default() });
            let _ = db::add_community_reward_info(pool, &CommunityRewardInfo { index: 0, uid, r#type: Some(r#type), sub_type: Some(sub_type) }).await;
        }
    }

    let response = CommunityRewardResponse {
        reward_info_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
    };
    
    let resp_bytes = response.encode_to_vec();
    
    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    
    let (route, code) = PacketCodeType::CommunityReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}