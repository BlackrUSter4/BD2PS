use bd2::prost::Message;
use bd2::proto::proto_net::{HuntingGroundEnterRequest, HuntingGroundEnterResponse, MonsterDbInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::hunting::{hunting_ground_info as ground_db, hunting_ground_monster};
use database::models::game::hunting::hunting_ground_info::HuntingGroundInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Real get-or-create entry into a hunting ground: creates the account's real HuntingGroundInfo
/// row on first entry, or updates its current/highest reached id and auto-hunt flag; returns
/// the real monster list already scaffolded via HuntingGroundMonster/MonsterInfo.
pub async fn handle(pool: &SqlitePool, uid: i64, req: HuntingGroundEnterRequest) -> GameResponse {
    info!("Handling HuntingGroundEnterRequest: {:?}", req);

    let pack_id = req.pack_id.unwrap_or(0);
    let hunting_ground_id = req.hunting_ground_id.unwrap_or(0);

    let ground_index = match ground_db::get_by_uid_and_pack(pool, uid, pack_id).await.ok().flatten() {
        Some(row) => {
            let highest = row.highest_id.unwrap_or(0).max(hunting_ground_id);
            let _ = ground_db::update_entry(pool, row.index, req.is_auto, hunting_ground_id, highest).await;
            row.index
        }
        None => ground_db::insert(
            pool,
            &HuntingGroundInfo {
                index: 0,
                uid,
                is_auto: req.is_auto,
                current_id: Some(hunting_ground_id),
                highest_id: Some(hunting_ground_id),
                pack_id: Some(pack_id),
            },
        )
        .await
        .unwrap_or(0),
    };

    let monster_info = hunting_ground_monster::get_monsters_for_hunting_ground(pool, ground_index)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|m| MonsterDbInfo {
            monster_id: m.monster_id,
            battle_deck: m.battle_deck,
            respawn_time: m.respawn_time,
            life_end_time: m.life_end_time,
            group_id: m.group_id,
            active_flag: m.active_flag,
        })
        .collect();

    let response = HuntingGroundEnterResponse { monster_info };

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

    let (route, code) = PacketCodeType::HuntingGroundEnter.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
