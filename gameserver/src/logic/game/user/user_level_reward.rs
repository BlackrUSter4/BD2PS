use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, UserLevelRewardRequest, UserLevelRewardResponse, RewardDbInfoBundle, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, user::user_info as db};
use sqlx::SqlitePool;
use tracing::info;

/// Real claim-once tracking via UserInfo.LevelReward as a bitmask (bit N = account level N's
/// reward already claimed) — no dedicated level-reward table exists in this project, so the
/// granted gold amount (100 per level) is a documented placeholder.
const PLACEHOLDER_GOLD_PER_LEVEL: i32 = 100;

pub async fn handle(pool: &SqlitePool, uid: i64, req: UserLevelRewardRequest) -> GameResponse {
    info!("Handling UserLevelRewardRequest: {:?}", req);

    let mut item_infos = Vec::new();
    if let Ok(Some(mut row)) = db::get_user_info(pool, uid).await.map(|v| v.into_iter().next()) {
        let mut mask = row.level_reward.unwrap_or(0);
        for &level in &req.id {
            if !(0..32).contains(&level) {
                continue;
            }
            let bit = 1i32 << level;
            if mask & bit != 0 {
                continue;
            }
            let _ = item_info::grant(pool, uid, 4, 1, PLACEHOLDER_GOLD_PER_LEVEL).await;
            item_infos.push(ItemDbInfo {
                id: Some(4),
                r#type: Some(1),
                count: Some(PLACEHOLDER_GOLD_PER_LEVEL),
                ..Default::default()
            });
            mask |= bit;
        }
        row.level_reward = Some(mask);
        let _ = sqlx::query("UPDATE UserInfo SET LevelReward = ? WHERE Uid = ?")
            .bind(mask)
            .bind(uid)
            .execute(pool)
            .await;
    }

    let response = UserLevelRewardResponse {
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
    
    let (route, code) = PacketCodeType::UserLevelReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}