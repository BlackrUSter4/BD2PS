use bd2::prost::Message;
use bd2::proto::proto_net::{GachaPointManualExchangeRequest, GachaPointManualExchangeResponse, ItemDbInfo, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

/// "Manual" point exchange — converts a chosen quantity of banner points
/// into a reward, as opposed to picking one fixed item (that's
/// GachaPointExchange). No conversion-rate table was captured, so this
/// deducts `count` points for real and grants a placeholder gold amount
/// scaled by that count, rather than fabricating a specific item mapping.
const PLACEHOLDER_GOLD_PER_POINT: i32 = 10;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GachaPointManualExchangeRequest) -> GameResponse {
    info!("Handling GachaPointManualExchangeRequest: {:?}", req);

    let count = req.count.unwrap_or(0).max(0);

    if let Some(group_id) = req.group_id {
        let _ = sqlx::query(
            "UPDATE GachaUserInfo SET Point = MAX(0, COALESCE(Point, 0) - ?) WHERE Uid = ? AND GroupId = ?",
        )
        .bind(count)
        .bind(uid)
        .bind(group_id as i32)
        .execute(pool)
        .await;
    }

    let reward_info_bundle = if count > 0 {
        match item_info::grant(pool, uid, 4, 1, count * PLACEHOLDER_GOLD_PER_POINT).await {
            Ok(()) => item_info::find_by_item_id(pool, uid, 4)
                .await
                .ok()
                .flatten()
                .map(|item| RewardDbInfoBundle {
                    item_info: vec![ItemDbInfo {
                        inven_index: item.inven_index,
                        id: item.id,
                        r#type: item.r#type,
                        count: item.count,
                        keep_flag: item.keep_flag,
                        time_value: item.time_value,
                        pictorialbook_info: None,
                        expiry_time: item.expiry_time,
                        sort_id: item.sort_id,
                        use_count: item.use_count,
                    }],
                    ..Default::default()
                }),
            Err(e) => {
                tracing::warn!("GachaPointManualExchange: failed to grant reward: {}", e);
                None
            }
        }
    } else {
        None
    };

    let response = GachaPointManualExchangeResponse { reward_info_bundle };
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

    let (route, code) = PacketCodeType::GachaPointManualExchange.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
