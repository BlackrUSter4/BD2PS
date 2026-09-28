use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, PackDbInfo, PackInfoRequest, PackInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pack::pack_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account progress for every real pack in PackTable (79 real rows) — was
/// previously two hardcoded fake pack entries. `quest_level_info`/
/// `pack_clear_reward_info`/`evil_castle_clear_reward_info` are left empty — those need
/// deep joins into the quest/evil-castle systems that are out of scope for this handler.
pub async fn handle(pool: &SqlitePool, uid: i64, req: PackInfoRequest) -> GameResponse {
    info!("Handling PackInfoRequest: {:?}", req);

    let mut pack_info = Vec::new();
    for pack in data::exceldb::get().packtable.all() {
        if let Ok(row) = db::get_or_create(pool, uid, pack.id).await {
            pack_info.push(PackDbInfo {
                id: row.id,
                clear_quest_count: row.clear_quest_count,
                is_pack_complete: row.is_pack_complete,
                quest_level: row.quest_level,
                quest_opt: row.quest_opt,
                sub_quest_count: row.sub_quest_count,
                active_time: row.active_time,
                is_buy: row.is_buy,
            });
        }
    }

    let response = PackInfoResponse {
        pack_info,
        quest_level_info: vec![],
        pack_clear_reward_info: vec![],
        evil_castle_clear_reward_info: vec![],
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

    let (route, code) = PacketCodeType::PackInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
