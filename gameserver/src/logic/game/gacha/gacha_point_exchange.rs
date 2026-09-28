use bd2::prost::Message;
use bd2::proto::proto_net::{GachaPointExchangeRequest, GachaPointExchangeResponse, ItemDbInfo, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

/// Real point-balance deduction (best-effort — decrements if a balance row
/// exists, never goes negative) and a real item grant for whatever
/// item id the client actually asked to redeem (select_item_id) — no
/// point-cost table was captured to validate against, so the requested
/// exchange is trusted and granted at face value (count 1) rather than
/// computing a "real" cost that no data confirms.
pub async fn handle(pool: &SqlitePool, uid: i64, req: GachaPointExchangeRequest) -> GameResponse {
    info!("Handling GachaPointExchangeRequest: {:?}", req);

    if let Some(group_id) = req.group_id {
        let _ = sqlx::query(
            "UPDATE GachaUserInfo SET Point = MAX(0, COALESCE(Point, 0) - 1) WHERE Uid = ? AND GroupId = ?",
        )
        .bind(uid)
        .bind(group_id as i32)
        .execute(pool)
        .await;
    }

    let reward_info_bundle = if let Some(item_id) = req.select_item_id {
        match item_info::grant(pool, uid, item_id, 1, 1).await {
            Ok(()) => item_info::find_by_item_id(pool, uid, item_id)
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
                tracing::warn!("GachaPointExchange: failed to grant item: {}", e);
                None
            }
        }
    } else {
        None
    };

    let response = GachaPointExchangeResponse { reward_info_bundle };
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

    let (route, code) = PacketCodeType::GachaPointExchange.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
