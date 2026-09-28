use super::{auto_upgrade_to_level, create_new_equip, get_equip_with_base, to_dbinfo, try_consume_items};
use bd2::prost::Message;
use bd2::proto::proto_net::{
    DefineEquipUpgradeResultType, EquipFromMakingToUpgradeRequest, EquipFromMakingToUpgradeResponse,
    Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Makes a new equip then immediately auto-upgrades it toward upgrade_target_level. Upgrade
/// attempts in this combo flow carry no per-attempt item cost in the schema (only the making
/// materials do) — see mod.rs's auto_upgrade_to_level for why every attempt succeeds.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipFromMakingToUpgradeRequest) -> GameResponse {
    info!("Handling EquipFromMakingToUpgradeRequest: {:?}", req);

    let mut equip_dbinfo = None;
    let mut try_count = 0;
    let mut result_type = DefineEquipUpgradeResultType::UpgradeFail;

    if let Some(making_id) = req.making_id {
        if try_consume_items(pool, uid, &req.making_item_info).await {
            if let Ok(idx) = create_new_equip(pool, uid, making_id).await {
                if let Some((equip, Some(mut base))) = get_equip_with_base(pool, uid, idx).await {
                    let target = req.upgrade_target_level.unwrap_or(base.level.unwrap_or(1));
                    let (_level, tries) = auto_upgrade_to_level(pool, uid, &mut base, target).await;
                    try_count = tries;
                    result_type = DefineEquipUpgradeResultType::UpgradeSuccess;
                    equip_dbinfo = Some(to_dbinfo(&equip, Some(&base)));
                }
            }
        }
    }

    let response = EquipFromMakingToUpgradeResponse {
        add_talent_exp: Some(0),
        equip_info: equip_dbinfo,
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

    let (route, code) = PacketCodeType::EquipFromMakingToUpgrade.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
