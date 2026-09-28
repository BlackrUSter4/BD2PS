use bd2::prost::Message;
use bd2::proto::proto_net::{CashBonusDbInfo, CashBonusInfoRequest, CashBonusInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::cash::cash_bonus_info as db;
use sqlx::SqlitePool;
use tracing::info;

fn parse_rewarded(text: &Option<String>) -> Vec<i32> {
    text.as_deref()
        .map(|s| s.split(',').filter_map(|p| p.parse().ok()).collect())
        .unwrap_or_default()
}

pub async fn handle(pool: &SqlitePool, uid: i64, req: CashBonusInfoRequest) -> GameResponse {
    info!("Handling CashBonusInfoRequest: {:?}", req);

    let bonus_info = db::get_all(pool, uid)
        .await
        .into_iter()
        .map(|row| CashBonusDbInfo {
            group_id: Some(row.group_id),
            contents_group_id: Some(row.contents_group_id),
            buy_count: Some(row.buy_count),
            rewarded_id: parse_rewarded(&row.rewarded_ids),
        })
        .collect();

    let response = CashBonusInfoResponse { bonus_info };
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
    let (route, code) = PacketCodeType::CashBonusInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
