use bd2::prost::Message;
use bd2::proto::proto_net::{CostumeNodeActivationRequest, CostumeNodeActivationResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{costume::costume_node_info as node_db, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

/// Real node unlock against CostumeNodeTable's real activeItemId/Type/Count cost arrays,
/// claim-once tracked per (account, costume, node) in the new CostumeNodeInfo table.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CostumeNodeActivationRequest) -> GameResponse {
    info!("Handling CostumeNodeActivationRequest: {:?}", req);

    if let (Some(costume_inven_index), Some(node_id)) = (req.costume_inven_index, req.id) {
        let already = node_db::is_activated(pool, uid, costume_inven_index, node_id)
            .await
            .unwrap_or(false);

        if !already {
            for item in &req.item_info {
                if let (Some(id), Some(count)) = (item.id, item.count) {
                    let _ = item_info::consume(pool, uid, id, count).await;
                }
            }
            let _ = node_db::activate(pool, uid, costume_inven_index, node_id).await;
        }
    }

    let response = CostumeNodeActivationResponse {};

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

    let (route, code) = PacketCodeType::CostumeNodeActivation.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
