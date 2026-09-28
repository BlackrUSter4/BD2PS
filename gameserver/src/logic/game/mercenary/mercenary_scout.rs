use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, MercenaryScoutRequest, MercenaryScoutResponse, RewardDbInfoBundle, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

/// Real gift-scouting against MercenaryScoutTable's real present_item_id/type/count arrays
/// (keyed by npc_id, judgment call — the table's own id field is the only plausible match for
/// the request's npc_id). Real item consumption for the client-reported cost.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MercenaryScoutRequest) -> GameResponse {
    info!("Handling MercenaryScoutRequest: {:?}", req);

    for item in &req.item_info {
        if let (Some(id), Some(count)) = (item.id, item.count) {
            let _ = item_info::consume(pool, uid, id, count).await;
        }
    }

    let mut item_infos = Vec::new();
    if let Some(npc_id) = req.npc_id {
        if let Some(def) = data::exceldb::get().mercenaryscouttable.get(npc_id) {
            for i in 0..def.present_item_id.len() {
                let id = def.present_item_id[i];
                let ty = *def.present_item_type.get(i).unwrap_or(&1);
                let count = *def.present_item_count.get(i).unwrap_or(&1);
                let _ = item_info::grant(pool, uid, id, ty, count).await;
                item_infos.push(ItemDbInfo { id: Some(id), r#type: Some(ty), count: Some(count), ..Default::default() });
            }
        }
    }

    let response = MercenaryScoutResponse {
        reward_info_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
        first_auto_revive_set_char_inven_index: None,
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
    
    let (route, code) = PacketCodeType::MercenaryScout.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}