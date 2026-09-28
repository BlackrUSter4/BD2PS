use super::{get_equip_with_base, max_level_for, to_dbinfo, try_consume_items};
use bd2::prost::Message;
use bd2::proto::proto_net::{
    DefineEquipUpgradeResultType, EquipUpgradeRequest, EquipUpgradeResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipUpgradeRequest) -> GameResponse {
    info!("Handling EquipUpgradeRequest: {:?}", req);

    let mut result_type = DefineEquipUpgradeResultType::UpgradeFail;
    let mut equip_dbinfo = None;

    if let Some(inven_index) = req.inven_index {
        if let Some((equip, Some(base))) = get_equip_with_base(pool, uid, inven_index).await {
            let equip_id = base.id.unwrap_or(0);
            let level = base.level.unwrap_or(1);
            let max_level = max_level_for(equip_id);
            if level >= max_level {
                result_type = DefineEquipUpgradeResultType::UpgradeStopMaxLevel;
            } else if try_consume_items(pool, uid, &req.item_info).await {
                let new_level = level + 1;
                let _ = database::db::equip::equip_base_info::set_level(
                    pool,
                    uid,
                    base.index,
                    new_level,
                )
                .await;
                result_type = DefineEquipUpgradeResultType::UpgradeSuccess;
                let mut updated_base = base;
                updated_base.level = Some(new_level);
                equip_dbinfo = Some(to_dbinfo(&equip, Some(&updated_base)));
            } else {
                result_type = DefineEquipUpgradeResultType::UpgradeStopNotEnough;
            }
        }
    }

    let response = EquipUpgradeResponse {
        equip_info: equip_dbinfo,
        result_type: Some(result_type as i32),
        char_info: None,
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipUpgrade.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
