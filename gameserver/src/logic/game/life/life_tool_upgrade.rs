use super::try_consume_items;
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeToolUpgradeRequest, LifeToolUpgradeResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Consumes the claimed materials for real. The actual next-tier tool id would come from
/// `LifeToolTable` (not captured) — as a reasonable placeholder, upgrading just increments the
/// current tool id by 1 within its group (tiers 1,2,3.. is a common enough convention).
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeToolUpgradeRequest) -> GameResponse {
    info!("Handling LifeToolUpgradeRequest: {:?}", req);

    let mut result_group = None;
    let mut result_id = None;

    if let Some(group_id) = req.group_id {
        if try_consume_items(pool, uid, &req.use_item).await {
            let current = database::db::life::life_tool_info::get_by_uid(pool, uid)
                .await
                .unwrap_or_default()
                .into_iter()
                .find(|t| t.group_id == group_id)
                .map(|t| t.tool_id)
                .unwrap_or(0);
            let next = current + 1;
            let _ = database::db::life::life_tool_info::upsert(pool, uid, group_id, next).await;
            result_group = Some(group_id);
            result_id = Some(next);
        }
    }

    let response = LifeToolUpgradeResponse {
        group_id: result_group,
        id: result_id,
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeToolUpgrade.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
