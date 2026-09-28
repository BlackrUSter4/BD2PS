use bd2::prost::Message;
use bd2::proto::proto_net::{CostumeUseRequest, CostumeUseResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{char::char_info, costume::costume_use_info as use_db};
use database::models::game::costume::costume_use_info::CostumeUseInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Real costume-equip: for each (costume, char) pair, both sides of the equip relationship
/// are updated for real (CharInfo.UseCostume, CostumeInfo.UseChar) and the action is logged
/// to CostumeUseInfo. `battle_mode` has no separate loadout concept anywhere else in this
/// project, so it's accepted but not branched on.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CostumeUseRequest) -> GameResponse {
    info!("Handling CostumeUseRequest: {:?}", req);

    for entry in &req.costume_use_info {
        let (Some(costume_index), Some(char_index)) = (entry.costume_index, entry.char_index) else {
            continue;
        };
        let _ = char_info::set_use_costume(pool, uid, char_index, costume_index).await;
        let _ = database::db::costume::costume_info::set_use_char(pool, uid, costume_index, char_index).await;
        let _ = use_db::add_costume_use_info(
            pool,
            &CostumeUseInfo {
                index: 0,
                uid,
                costume_index: Some(costume_index),
                char_index: Some(char_index),
            },
        )
        .await;
    }

    let response = CostumeUseResponse {};

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

    let (route, code) = PacketCodeType::CostumeUse.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
