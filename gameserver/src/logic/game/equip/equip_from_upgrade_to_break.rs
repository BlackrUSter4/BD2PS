use super::{auto_upgrade_to_level, get_equip_with_base, to_dbinfo};
use bd2::prost::Message;
use bd2::proto::proto_net::{
    DefineEquipUpgradeResultType, EquipFromUpgradeToBreakRequest, EquipFromUpgradeToBreakResponse,
    ItemDbInfo, Notify, RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

const BREAK_REWARD_ITEM_ID: i32 = 10;
const BREAK_REWARD_ITEM_TYPE: i32 = 8;
const BREAK_REWARD_COUNT: i32 = 10;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipFromUpgradeToBreakRequest) -> GameResponse {
    info!("Handling EquipFromUpgradeToBreakRequest: {:?}", req);

    let mut equip_dbinfo = None;
    let mut try_count = 0;
    let mut result_type = DefineEquipUpgradeResultType::UpgradeFail;
    let mut reward_info_bundle = None;

    if let Some(inven_index) = req.equip_inven_index {
        if let Some((equip, Some(mut base))) = get_equip_with_base(pool, uid, inven_index).await {
            let target = req.upgrade_target_level.unwrap_or(base.level.unwrap_or(1));
            let (level, tries) = auto_upgrade_to_level(pool, uid, &mut base, target).await;
            try_count = tries;
            if level < target {
                let _ = database::db::equip::equip_info::delete_by_inven_index(pool, uid, inven_index).await;
                let _ = database::db::equip::equip_base_info::delete(pool, uid, base.index).await;
                let _ = database::db::item::item_info::grant(
                    pool,
                    uid,
                    BREAK_REWARD_ITEM_ID,
                    BREAK_REWARD_ITEM_TYPE,
                    BREAK_REWARD_COUNT,
                )
                .await;
                result_type = DefineEquipUpgradeResultType::UpgradeStopMaxLevel;
                reward_info_bundle = Some(RewardDbInfoBundle {
                    item_info: vec![ItemDbInfo {
                        id: Some(BREAK_REWARD_ITEM_ID),
                        r#type: Some(BREAK_REWARD_ITEM_TYPE),
                        count: Some(BREAK_REWARD_COUNT),
                        ..Default::default()
                    }],
                    ..Default::default()
                });
            } else {
                result_type = DefineEquipUpgradeResultType::UpgradeSuccess;
                equip_dbinfo = Some(to_dbinfo(&equip, Some(&base)));
            }
        }
    }

    let response = EquipFromUpgradeToBreakResponse {
        equip_info: equip_dbinfo,
        result_type: Some(result_type as i32),
        try_count: Some(try_count),
        consume_item_info: vec![],
        lack_item_info: vec![],
        upgrade_used_gold: Some(0),
        reward_info_bundle,
    };

    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipFromUpgradeToBreak.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
