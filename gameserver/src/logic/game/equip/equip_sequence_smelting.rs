use super::{get_equip_with_base, to_dbinfo};
use bd2::prost::Message;
use bd2::proto::proto_net::{
    DefineEquipUpgradeResultType, EquipSequenceSmeltingRequest, EquipSequenceSmeltingResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

const MAX_RANK: i32 = 4;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipSequenceSmeltingRequest) -> GameResponse {
    info!("Handling EquipSequenceSmeltingRequest: {:?}", req);

    let mut equip_dbinfo = None;
    let mut try_count = 0;
    let mut result_type = DefineEquipUpgradeResultType::UpgradeFail;

    if let Some(inven_index) = req.inven_index {
        if let Some((equip, Some(mut base))) = get_equip_with_base(pool, uid, inven_index).await {
            let attempts = req.smelting_count.unwrap_or(1).max(1);
            let stop_on_success = req.is_success_stop.unwrap_or(false);
            for _ in 0..attempts {
                if base.rank >= MAX_RANK {
                    result_type = DefineEquipUpgradeResultType::UpgradeStopMaxLevel;
                    break;
                }
                base.rank += 1;
                try_count += 1;
                result_type = DefineEquipUpgradeResultType::UpgradeSuccess;
                if stop_on_success {
                    break;
                }
            }
            if try_count > 0 {
                let _ = database::db::equip::equip_base_info::set_rank(pool, uid, base.index, base.rank).await;
            }
            equip_dbinfo = Some(to_dbinfo(&equip, Some(&base)));
        }
    }

    let response = EquipSequenceSmeltingResponse {
        equip_info: equip_dbinfo,
        char_info: None,
        result_type: Some(result_type as i32),
        try_count: Some(try_count),
        consume_item_info: vec![],
        lack_item_info: vec![],
    };

    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipSequenceSmelting.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
