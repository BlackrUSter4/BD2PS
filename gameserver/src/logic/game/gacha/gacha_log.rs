use bd2::prost::Message;
use bd2::proto::proto_net::{GachaLogDbInfo, GachaLogRequest, GachaLogResponse, GachaTotalCountInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::gacha::{gacha_log_info, gacha_total_count_info};
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GachaLogRequest) -> GameResponse {
    info!("Handling GachaLogRequest: {:?}", req);

    let since = req.last_log_time.unwrap_or(0);
    let gacha_log = gacha_log_info::get_gacha_log_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|l| l.log_time.unwrap_or(0) >= since)
        .map(|l| GachaLogDbInfo {
            gacha_group_id: l.gacha_group_id,
            gacha_id: l.gacha_id,
            buy_type: None,
            gacha_count: l.gacha_count,
            get_point: l.get_point,
            gacha_type: None,
            pickup_item_id: l.pickup_item_id,
            gacha_fixed_info: vec![],
            reward_info_bundle: None,
            new_sort: vec![],
            ..Default::default()
        })
        .collect();

    let gacha_total_count_info = gacha_total_count_info::get_gacha_total_count_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|t| GachaTotalCountInfo {
            gacha_log_type: None,
            char5_pick_up_costume: t.char5_pick_up_costume,
            char5_costume: t.char5_costume,
            char4_costume: t.char4_costume,
            char3_costume: t.char3_costume,
            char5_pick_up_equip4: t.char5_pick_up_equip4,
            char5_equip4: t.char5_equip4,
            char5_equip3: t.char5_equip3,
            char4_equip4: t.char4_equip4,
            char4_equip3: t.char4_equip3,
            char4_equip2: t.char4_equip2,
            char3_equip4: t.char3_equip4,
            char3_equip3: t.char3_equip3,
            char3_equip2: t.char3_equip2,
        })
        .collect();

    let response = GachaLogResponse {
        gacha_log,
        gacha_total_count_info,
    };

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

    let (route, code) = PacketCodeType::Common.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
