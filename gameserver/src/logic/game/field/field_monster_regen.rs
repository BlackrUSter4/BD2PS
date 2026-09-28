use bd2::prost::Message;
use bd2::proto::proto_net::{FieldMonsterRegenRequest, FieldMonsterRegenResponse, MonsterDbInfo, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Real regen timing from FieldMonsterTable.regen_id -> FieldMonsterRegenTable.regen_sec
/// (real data) when both are configured for this monster.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: FieldMonsterRegenRequest) -> GameResponse {
    info!("Handling FieldMonsterRegenRequest: {:?}", req);

    let monster_info = req.monster_id.and_then(|monster_id| {
        let game_data = data::exceldb::get();
        let monster = game_data.fieldmonstertable.get(monster_id)?;
        let regen_sec = monster
            .regen_id
            .and_then(|rid| game_data.fieldmonsterregentable.get(rid))
            .map(|r| r.regen_sec as i64)
            .unwrap_or(300);
        Some(MonsterDbInfo {
            monster_id: Some(monster_id),
            respawn_time: Some(chrono::Utc::now().timestamp_millis() + regen_sec * 1000),
            active_flag: Some(false),
            ..Default::default()
        })
    });

    let response = FieldMonsterRegenResponse { monster_info };
    
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
    
    let (route, code) = PacketCodeType::FieldMonsterRegen.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}