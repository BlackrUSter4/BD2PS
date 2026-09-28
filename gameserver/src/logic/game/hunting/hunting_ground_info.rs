use bd2::prost::Message;
use bd2::proto::proto_net::{
    HuntingGroundDbInfo, HuntingGroundInfoRequest, HuntingGroundInfoResponse, MonsterDbInfo, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{
    hunting::hunting_ground_info::get_hunting_ground_info,
    hunting::hunting_ground_monster::get_monsters_for_hunting_ground,
};
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: HuntingGroundInfoRequest) -> GameResponse {
    info!("Handling HuntingGroundInfoRequest: {:?}", req);

    let pack_id = req.pack_id.unwrap_or_default();

    let grounds = get_hunting_ground_info(pool, uid).await.unwrap_or_default();

    let ground_opt = grounds.into_iter().find(|g| g.pack_id == Some(pack_id));

    let hunting_ground_info = if let Some(g) = ground_opt {
        let monsters = get_monsters_for_hunting_ground(pool, g.index)
            .await
            .unwrap_or_default();

        let monster_info = monsters
            .into_iter()
            .map(|m| MonsterDbInfo {
                monster_id: m.monster_id,
                battle_deck: m.battle_deck,
                respawn_time: m.respawn_time,
                life_end_time: m.life_end_time,
                group_id: m.group_id,
                active_flag: m.active_flag,
            })
            .collect::<Vec<_>>();

        Some(HuntingGroundDbInfo {
            is_auto: g.is_auto,
            current_id: g.current_id,
            highest_id: g.highest_id,
            monster_info,
            pack_id: g.pack_id,
        })
    } else {
        None
    };

    let response = HuntingGroundInfoResponse {
        hunting_ground_info,
        ..Default::default()
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

    let (route, code) = PacketCodeType::HuntingGroundInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
