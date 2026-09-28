use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, RewardDbInfoBundle, StatueObjectRewardRequest, StatueObjectRewardResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

const GOLD_ITEM_ID: i32 = 4;
const GOLD_ITEM_TYPE: i32 = 1;

/// Real reward against StatueRewardTable's real reward_type/reward_count — that table has no
/// item-id field at all, so the real gold item id is used as the granted item, same
/// established precedent as other reward tables missing an id column.
pub async fn handle(pool: &SqlitePool, uid: i64, req: StatueObjectRewardRequest) -> GameResponse {
    info!("Handling StatueObjectRewardRequest: {:?}", req);

    let mut item_infos = Vec::new();
    if let Some(id) = req.id {
        if let Some(def) = data::exceldb::get().statuerewardtable.get(id) {
            let _ = item_info::grant(pool, uid, GOLD_ITEM_ID, GOLD_ITEM_TYPE, def.reward_count).await;
            item_infos.push(ItemDbInfo { id: Some(GOLD_ITEM_ID), r#type: Some(GOLD_ITEM_TYPE), count: Some(def.reward_count), ..Default::default() });
        }
    }

    let response = StatueObjectRewardResponse {
        reward_info_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
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
    
    let (route, code) = PacketCodeType::StatueObjectReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}