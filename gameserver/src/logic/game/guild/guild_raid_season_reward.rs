use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRaidSeasonRewardRequest, GuildRaidSeasonRewardResponse, ItemDbInfo, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_raid_main_info;
use sqlx::SqlitePool;
use tracing::info;

/// Claim-once tracking is real (via GuildRaidMainInfo.ObtainableSeasonReward),
/// but the reward CONTENTS are a documented placeholder — no season-reward
/// table was captured to map a rank/score to a real bundle.
pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidSeasonRewardRequest) -> GameResponse {
    info!("Handling GuildRaidSeasonRewardRequest: {:?}", req);

    let already_claimed = guild_raid_main_info::get_guild_raid_main_info(pool, uid)
        .await
        .ok()
        .and_then(|v| v.into_iter().next())
        .and_then(|m| m.obtainable_season_reward)
        .map(|v| v == 0)
        .unwrap_or(true);

    let (reward_info_bundle, is_received_reward) = if already_claimed {
        (None, Some(true))
    } else {
        let _ = sqlx::query("UPDATE GuildRaidMainInfo SET ObtainableSeasonReward = 0 WHERE Uid = ?")
            .bind(uid)
            .execute(pool)
            .await;
        (
            Some(RewardDbInfoBundle {
                item_info: vec![ItemDbInfo { id: Some(4), count: Some(1000), ..Default::default() }],
                ..Default::default()
            }),
            Some(false),
        )
    };

    let response = GuildRaidSeasonRewardResponse { reward_info_bundle, is_received_reward };
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
    let (route, code) = PacketCodeType::GuildRaidSeasonReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
