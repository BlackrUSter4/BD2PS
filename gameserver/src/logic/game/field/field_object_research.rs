use bd2::prost::Message;
use bd2::proto::proto_net::{FieldObjectResearchRequest, FieldObjectResearchResponse, ItemDbInfo, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

/// Real reward grant from FieldResearchObjectTable (real reward_id/type/count fields).
/// Pack-scoped lookup (`req.pack_id`) since `object_id` collides across packs
/// — see Fieldresearchobjecttable::pack_id's doc comment.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FieldObjectResearchRequest) -> GameResponse {
    info!("Handling FieldObjectResearchRequest: {:?}", req);

    let mut reward_item = Vec::new();
    if let (Some(object_id), Some(pack_id)) = (req.object_id, req.pack_id) {
        if let Some(obj) = data::exceldb::get().fieldresearchobjecttable.get_by_pack(pack_id, object_id) {
            if let (Some(id), Some(count)) = (obj.reward_id, obj.reward_count) {
                let ty = obj.reward_type.unwrap_or(1);
                let _ = item_info::grant(pool, uid, id, ty, count).await;
                reward_item.push(ItemDbInfo {
                    id: Some(id),
                    r#type: Some(ty),
                    count: Some(count),
                    ..Default::default()
                });
            }
        }
    }

    let response = FieldObjectResearchResponse {
        seq: req.seq,
        reward_item,
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
    
    let (route, code) = PacketCodeType::FieldObjectResearch.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}