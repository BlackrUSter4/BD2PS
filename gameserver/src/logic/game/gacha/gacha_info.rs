use bd2::prost::Message;
use bd2::proto::proto_net::{
    GachaFixedDbInfo, GachaInfoRequest, GachaInfoResponse, GachaScheduleDbInfo,
    GachaSelectionCountChangeDbInfo, GachaSelectionDbInfo, GachaStepUpScheduleDbInfo,
    GachaStepUpUserDbInfo, GachaUserDbInfo, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::gacha::{
    gacha_selection_count_change_info, gacha_selection_info, gacha_step_up_user_info,
    gacha_user_info,
};
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account gacha state (points, pick counts, selections, step-up
/// progress) via the existing query layer. Schedule info comes from the
/// real (if thin) GachaGroupTable — every active banner, always-open
/// (no real GachaScheduleInfo table/capture exists to say otherwise).
pub async fn handle(pool: &SqlitePool, uid: i64, req: GachaInfoRequest) -> GameResponse {
    info!("Handling GachaInfoRequest: {:?}", req);

    let schedule_info = exceldb::get()
        .gachagrouptable
        .all()
        .iter()
        .map(|g| GachaScheduleDbInfo {
            group_id: Some(g.id),
            start_time: Some(0),
            end_time: Some(0),
            is_gacha_free_count_bonus: Some(false),
            is_gacha_cash_count_bonus: Some(false),
        })
        .collect();

    let gacha_user_info_list = gacha_user_info::get_gacha_user_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|u| GachaUserDbInfo {
            group_id: u.group_id,
            point: u.point,
            total_buy_count: u.total_buy_count,
            one_free_pick_count: u.one_free_pick_count,
            one_cash_pick_count: u.one_cash_pick_count,
            ten_free_pick_count: u.ten_free_pick_count,
            ten_cash_pick_count: u.ten_cash_pick_count,
            exchange_item_count: u.exchange_item_count,
            exchange_mileage_count: u.exchange_mileage_count,
        })
        .collect();

    let gacha_selection_info_list = gacha_selection_info::get_gacha_selection_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|s| GachaSelectionDbInfo {
            group_id: s.group_id,
            slot: s.slot,
            item_id: s.item_id,
        })
        .collect();

    let gacha_selection_change_count_info =
        gacha_selection_count_change_info::get_gacha_selection_count_change_info(pool, uid)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|c| GachaSelectionCountChangeDbInfo {
                group_id: c.group_id,
                count: c.count,
            })
            .collect();

    let step_up_user_info = gacha_step_up_user_info::get_gacha_step_up_user_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|s| GachaStepUpUserDbInfo {
            group_id: s.group_id,
            id: s.id,
        })
        .collect();

    let response = GachaInfoResponse {
        schedule_info,
        gacha_user_info: gacha_user_info_list,
        schedule_end_exchange_point: Some(0),
        gacha_fixed_info: Vec::<GachaFixedDbInfo>::new(),
        gacha_selection_info: gacha_selection_info_list,
        gacha_selection_change_count_info,
        step_up_schedule_info: Vec::<GachaStepUpScheduleDbInfo>::new(),
        step_up_user_info,
        resemara_preview_item_info: vec![],
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

    let (route, code) = PacketCodeType::GachaInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
