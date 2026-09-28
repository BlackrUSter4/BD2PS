use bd2::prost::Message;
use bd2::proto::proto_net::{CashProductDbInfo, CashShopInfoRequest, CashShopInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use serde_json::Value;
use tracing::info;

pub async fn handle(
    _pool: &sqlx::SqlitePool,
    _uid: i64,
    _req: CashShopInfoRequest,
) -> GameResponse {
    info!("Handling CashShopInfoRequest");

    // Load the JSON file from starter data
    let data: Value = serde_json::from_str(include_str!(
        "../../../../../data/starter/cash_shop_info.json"
    ))
    .expect("Failed to parse cash_shop_info.json");

    let products = data
        .get("productInfo")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut product_info = Vec::new();

    for p in products {
        product_info.push(CashProductDbInfo {
            group_id: p.get("groupId").and_then(|v| v.as_i64()).map(|v| v as i32),
            id: p.get("id").and_then(|v| v.as_i64()).map(|v| v as i32),
            sale_group: p
                .get("saleGroup")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32),
            start_time: p.get("startTime").and_then(|v| v.as_i64()),
            end_time: p.get("endTime").and_then(|v| v.as_i64()),
            end_delay_minutes: p
                .get("endDelayMinutes")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32),
            event_index: p.get("eventIndex").and_then(|v| v.as_i64()),
            ..Default::default()
        });
    }

    // Parse reset times
    let daily_reset_time = data
        .get("dailyResetTime")
        .and_then(|v| v.as_i64())
        .unwrap_or_default();
    let weekly_reset_time = data
        .get("weeklyResetTime")
        .and_then(|v| v.as_i64())
        .unwrap_or_default();
    let monthly_reset_time = data
        .get("monthlyResetTime")
        .and_then(|v| v.as_i64())
        .unwrap_or_default();

    let response = CashShopInfoResponse {
        product_info,
        daily_reset_time: Some(daily_reset_time),
        weekly_reset_time: Some(weekly_reset_time),
        monthly_reset_time: Some(monthly_reset_time),
        ..Default::default()
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::CashShopInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
