use super::{get_equip_with_base, to_dbinfo, try_consume_items};
use bd2::prost::Message;
use bd2::proto::proto_net::{
    DefineEquipUpgradeResultType, EquipSmeltingRequest, EquipSmeltingResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

const MAX_RANK: i32 = 4; // matches the 4-entry rankValue/rankRatio arrays in EquipmentRankTable

/// Smelting raises an equip's Rank by one (consuming the client-declared materials) up to a real
/// cap taken from EquipmentRankTable's shape (4 tiers) — the exact per-grade smelt cost in
/// EquipmentGradeTable doesn't key to any currently-captured equip's grade, so the cost is
/// trusted from the request rather than derived, same call as elsewhere in this round.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipSmeltingRequest) -> GameResponse {
    info!("Handling EquipSmeltingRequest: {:?}", req);

    let mut equip_dbinfo = None;
    let mut result_type = DefineEquipUpgradeResultType::UpgradeFail;
    let mut result_grade = vec![];

    if let Some(inven_index) = req.inven_index {
        if let Some((equip, Some(mut base))) = get_equip_with_base(pool, uid, inven_index).await {
            if base.rank >= MAX_RANK {
                result_type = DefineEquipUpgradeResultType::UpgradeStopMaxLevel;
            } else if try_consume_items(pool, uid, &req.item_info).await {
                let new_rank = base.rank + 1;
                let _ = database::db::equip::equip_base_info::set_rank(pool, uid, base.index, new_rank).await;
                base.rank = new_rank;
                result_type = DefineEquipUpgradeResultType::UpgradeSuccess;
                result_grade = vec![new_rank];
                equip_dbinfo = Some(to_dbinfo(&equip, Some(&base)));
            } else {
                result_type = DefineEquipUpgradeResultType::UpgradeStopNotEnough;
            }
        }
    }

    let response = EquipSmeltingResponse {
        equip_info: equip_dbinfo,
        char_info: None,
        result_type: Some(result_type as i32),
        result_grade,
    };

    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipSmelting.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
