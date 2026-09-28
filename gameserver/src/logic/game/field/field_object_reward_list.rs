use crate::logic::field::reward_object::collect_field_object;
use bd2::prost::Message;
use bd2::proto::proto_net::{
    FieldObjectRewardListRequest, FieldObjectRewardListResponse, FieldObjectRewardRequest,
    Notify, RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Batch version of FieldObjectReward — collects each listed field object via the same
/// real reward-rolling/inventory-granting logic (`collect_field_object`) and merges the
/// results into one bundle.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FieldObjectRewardListRequest) -> GameResponse {
    info!("Handling FieldObjectRewardListRequest: {:?}", req);

    let mut item_info = Vec::new();
    let mut view_item_info = Vec::new();
    let mut original_item_info = Vec::new();

    for obj in &req.field_object_info {
        let single_req = FieldObjectRewardRequest {
            seq: req.seq,
            pack_id: req.pack_id,
            field_object_group_id: obj.group_id,
            field_object_id: obj.id,
        };
        if let Ok(resp) = collect_field_object(pool, uid, single_req).await {
            if let Some(bundle) = resp.reward_info_bundle {
                item_info.extend(bundle.item_info);
                view_item_info.extend(bundle.view_item_info);
                original_item_info.extend(bundle.original_item_info);
            }
        }
    }

    let response = FieldObjectRewardListResponse {
        reward_info_bundle: Some(RewardDbInfoBundle {
            item_info,
            view_item_info,
            original_item_info,
            ..Default::default()
        }),
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
    
    let (route, code) = PacketCodeType::FieldObjectRewardList.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}