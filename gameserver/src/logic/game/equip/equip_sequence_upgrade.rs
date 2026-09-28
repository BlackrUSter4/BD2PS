use super::{auto_upgrade_to_level, get_equip_with_base, to_dbinfo};
use bd2::prost::Message;
use bd2::proto::proto_net::{
    DefineEquipUpgradeResultType, EquipSequenceUpgradeRequest, EquipSequenceUpgradeResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipSequenceUpgradeRequest) -> GameResponse {
    info!("Handling EquipSequenceUpgradeRequest: {:?}", req);

    let mut equip_dbinfo = None;
    let mut try_count = 0;
    let mut result_type = DefineEquipUpgradeResultType::UpgradeFail;

    if let Some(inven_index) = req.inven_index {
        if let Some((equip, Some(mut base))) = get_equip_with_base(pool, uid, inven_index).await {
            let start_level = base.level.unwrap_or(1);
            let target = req
                .upgrade_target_level
                .unwrap_or(start_level + req.upgrade_count.unwrap_or(1));
            let (_level, tries) = auto_upgrade_to_level(pool, uid, &mut base, target).await;
            try_count = tries;
            result_type = DefineEquipUpgradeResultType::UpgradeSuccess;
            equip_dbinfo = Some(to_dbinfo(&equip, Some(&base)));
        }
    }

    let response = EquipSequenceUpgradeResponse {
        equip_info: equip_dbinfo,
        char_info: None,
        result_type: Some(result_type as i32),
        try_count: Some(try_count),
        consume_item_info: vec![],
        lack_item_info: vec![],
        upgrade_used_gold: Some(0),
    };

    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipSequenceUpgrade.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
