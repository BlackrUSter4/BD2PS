use bd2::proto::proto_net::{
    ChargeCostEventScheduleDbInfo, ChargeCostInfoRequest, ChargeCostInfoResponse, CostTimeDbInfo,
    ItemDbInfo, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::charge::charge_cost_event_schedule_info::get_charge_cost_event_schedule_info;
use database::models::game::charge::charge_cost_event_schedule_info::ChargeCostEventScheduleInfo;
use prost::Message;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: ChargeCostInfoRequest) -> GameResponse {
    info!("Handling ChargeCostInfoRequest for uid: {}", uid);

    // Load all cost-time entries for this user
    let all_cost_time_info = get_cost_time_with_items(pool, uid)
        .await
        .unwrap_or_default();

    // Make a lookup map: type -> CostTimeDbInfo
    use std::collections::HashMap;
    let mut by_type: HashMap<i32, CostTimeDbInfo> = HashMap::new();
    for entry in all_cost_time_info {
        if let Some(ref item) = entry.cost_item_info {
            if let Some(t) = item.r#type {
                by_type.insert(t, entry);
            }
        }
    }

    // Build cost_time_info in the same order as request.element_type
    let mut cost_time_info = Vec::new();
    for t in &req.element_type {
        if let Some(entry) = by_type.get(t) {
            cost_time_info.push(entry.clone());
        } else {
            // Fallback if type missing in DB
            cost_time_info.push(CostTimeDbInfo {
                last_charge_time: Some(0),
                cost_item_info: Some(ItemDbInfo {
                    inven_index: None,
                    id: None,
                    r#type: Some(*t),
                    count: Some(0),
                    keep_flag: None,
                    time_value: None,
                    pictorialbook_info: None,
                    expiry_time: None,
                    sort_id: None,
                    use_count: None,
                }),
            });
        }
    }

    // Load event schedule info
    let db_sched = get_charge_cost_event_schedule_info(pool, uid)
        .await
        .unwrap_or_default();

    let event_schedule_info = map_event_schedule(db_sched);

    // Build protobuf response
    let response = ChargeCostInfoResponse {
        cost_time_info,
        event_schedule_info,
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8916),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::ChargeCostInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}

// New function to JOIN CostTimeInfo with ItemInfo
async fn get_cost_time_with_items(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<CostTimeDbInfo>> {
    #[derive(sqlx::FromRow)]
    struct CostTimeWithItem {
        last_charge_time: i64,
        // Item fields
        inven_index: Option<i64>,
        id: Option<i32>,
        r#type: Option<i32>,
        count: Option<i32>,
        keep_flag: Option<i32>,
        time_value: Option<i64>,
        expiry_time: Option<i64>,
        sort_id: Option<i32>,
        use_count: Option<i32>,
    }

    let rows = sqlx::query_as::<_, CostTimeWithItem>(
        r#"
        SELECT
            c.LastChargeTime as last_charge_time,
            i.InvenIndex as inven_index,
            i.Id as id,
            i.Type as type,
            i.Count as count,
            i.KeepFlag as keep_flag,
            i.TimeValue as time_value,
            i.ExpiryTime as expiry_time,
            i.SortId as sort_id,
            i.UseCount as use_count
        FROM CostTimeInfo c
        LEFT JOIN CostItemInfo i ON c.CostItemInfoIndex = i.InvenIndex AND c.Uid = i.Uid
        WHERE c.Uid = ?
        "#,
    )
    .bind(uid)
    .fetch_all(pool)
    .await?;

    let result = rows
        .into_iter()
        .map(|r| {
            let item = if r.inven_index.is_some() {
                Some(ItemDbInfo {
                    inven_index: r.inven_index,
                    id: r.id,
                    r#type: r.r#type,
                    count: r.count,
                    keep_flag: r.keep_flag,
                    time_value: r.time_value,
                    pictorialbook_info: None,
                    expiry_time: r.expiry_time,
                    sort_id: r.sort_id,
                    use_count: r.use_count,
                })
            } else {
                None
            };

            CostTimeDbInfo {
                last_charge_time: Some(r.last_charge_time),
                cost_item_info: item,
            }
        })
        .collect();

    Ok(result)
}

fn map_event_schedule(
    rows: Vec<ChargeCostEventScheduleInfo>,
) -> Vec<ChargeCostEventScheduleDbInfo> {
    rows.into_iter()
        .map(|r| ChargeCostEventScheduleDbInfo {
            schedule_index: r.schedule_index,
            item_type: r.item_type,
            max_count: r.max_count,
            start_time: r.start_time,
            end_time: r.end_time,
        })
        .collect()
}
