use bd2::prost::Message;
use bd2::proto::proto_net::{
    FieldMonsterEventRequest, FieldMonsterEventResponse, ItemDbInfo, MonsterDbInfo, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

/// Real reward grant from FieldMonsterTable's real reward_id/type/count fields when the
/// monster has one configured. `char_info`/`equip_info` left empty (no server-side field
/// combat state exists — same as field_monster_damage.rs).
pub async fn handle(pool: &SqlitePool, uid: i64, req: FieldMonsterEventRequest) -> GameResponse {
    info!("Handling FieldMonsterEventRequest: {:?}", req);

    let mut item_infos = Vec::new();
    let mut monster_info = None;
    if let Some(monster_id) = req.monster_id {
        if let Some(monster) = data::exceldb::get().fieldmonstertable.get(monster_id) {
            monster_info = Some(MonsterDbInfo {
                monster_id: Some(monster_id),
                active_flag: Some(true),
                ..Default::default()
            });
            if let (Some(id), Some(count)) = (monster.reward_id, monster.reward_count) {
                let ty = monster.reward_type.unwrap_or(1);
                let _ = item_info::grant(pool, uid, id, ty, count).await;
                item_infos.push(ItemDbInfo {
                    id: Some(id),
                    r#type: Some(ty),
                    count: Some(count),
                    ..Default::default()
                });
            }
        }
    }

    let response = FieldMonsterEventResponse {
        char_info: vec![],
        monster_info,
        item_info: item_infos,
        equip_info: vec![],
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
    
    let (route, code) = PacketCodeType::FieldMonsterEvent.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}