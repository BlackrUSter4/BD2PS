use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, PvpBattleRewardRequest, PvpBattleRewardResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pvp::pvp_user_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, parse_claimed, rank_row_for_vp, GOLD_ITEM_ID, GOLD_ITEM_TYPE};

/// A "catch-up claim" distinct from BattleEnd's auto-grant: any once-reward tier at or below the
/// caller's current Vp that isn't marked claimed yet gets granted and marked now. In normal play
/// BattleEnd already keeps this in sync, so this mostly matters if state ever drifts.
pub async fn handle(pool: &SqlitePool, uid: i64, req: PvpBattleRewardRequest) -> GameResponse {
    info!("Handling PvpBattleRewardRequest: {:?}", req);

    let user = pvp_user_info::get_or_create(pool, uid).await.unwrap_or_default();
    let mut claimed = parse_claimed(&user.once_reward_claimed);

    let table = &data::exceldb::get().pvpranktable;
    let mut thresholds: Vec<i32> = table.all().iter().map(|r| r.vp).collect();
    thresholds.sort_unstable();
    thresholds.dedup();

    let mut item_info = Vec::new();
    for (tier_id, threshold) in thresholds.iter().enumerate() {
        let tier_id = tier_id as i32;
        if user.vp >= *threshold && !claimed.contains(&tier_id) {
            claimed.push(tier_id);
            if let Some(row) = rank_row_for_vp(*threshold) {
                let count: i32 = row.battle_win_reward_count.iter().sum();
                if count > 0 {
                    let _ = database::db::item::item_info::grant(pool, uid, GOLD_ITEM_ID, GOLD_ITEM_TYPE, count).await;
                    item_info.push(ItemDbInfo {
                        inven_index: None,
                        id: Some(GOLD_ITEM_ID),
                        r#type: Some(GOLD_ITEM_TYPE),
                        count: Some(count),
                        keep_flag: None,
                        time_value: None,
                        pictorialbook_info: None,
                        expiry_time: None,
                        sort_id: None,
                        use_count: None,
                    });
                }
            }
        }
    }

    let csv = claimed.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(",");
    let _ = pvp_user_info::set_once_reward_claimed(pool, uid, &csv).await;

    let response = PvpBattleRewardResponse { item_info };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PvpBattleReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
