use bd2::prost::Message;
use bd2::proto::proto_net::{EquipBreakRequest, EquipBreakResponse, ItemDbInfo, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// No EquipmentGrowthTable row matches any currently-captured equip id (its single row is keyed
/// to a growthGroupId no captured EquipmentTable row references), so break rewards are a
/// documented placeholder rather than a derived formula.
const BREAK_REWARD_ITEM_ID: i32 = 10;
const BREAK_REWARD_ITEM_TYPE: i32 = 8;
const BREAK_REWARD_COUNT: i32 = 10;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipBreakRequest) -> GameResponse {
    info!("Handling EquipBreakRequest: {:?}", req);

    let mut item_info = vec![];
    for inven_index in &req.inven_index {
        if let Ok(Some(base_index)) =
            database::db::equip::equip_info::delete_by_inven_index(pool, uid, *inven_index).await
        {
            let _ = database::db::equip::equip_base_info::delete(pool, uid, base_index).await;
            let _ = database::db::item::item_info::grant(
                pool,
                uid,
                BREAK_REWARD_ITEM_ID,
                BREAK_REWARD_ITEM_TYPE,
                BREAK_REWARD_COUNT,
            )
            .await;
            item_info.push(ItemDbInfo {
                id: Some(BREAK_REWARD_ITEM_ID),
                r#type: Some(BREAK_REWARD_ITEM_TYPE),
                count: Some(BREAK_REWARD_COUNT),
                ..Default::default()
            });
        }
    }

    let response = EquipBreakResponse {
        reward_info_bundle: Some(RewardDbInfoBundle {
            item_info,
            ..Default::default()
        }),
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipBreak.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
