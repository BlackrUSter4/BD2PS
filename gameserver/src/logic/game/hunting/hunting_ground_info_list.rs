use bd2::prost::Message;
use bd2::proto::proto_net::{
    HuntingGroundDbInfo, HuntingGroundInfoListRequest, HuntingGroundInfoListResponse,
    MonsterDbInfo, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{
    hunting::hunting_ground_info_list::get_hunting_ground_info,
    monster::monster_info::get_monster_info,
};
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    _req: HuntingGroundInfoListRequest,
) -> GameResponse {
    info!("Handling HuntingGroundInfoRequest for uid: {}", uid);

    let grounds = get_hunting_ground_info(pool, uid).await.unwrap_or_default();
    let monsters = get_monster_info(pool, uid).await.unwrap_or_default();

    // Map each HuntingGroundDBInfo
    let hunting_ground_info = grounds
        .into_iter()
        .map(|g| {
            let monster_info = monsters
                .iter()
                .map(|m| MonsterDbInfo {
                    monster_id: m.monster_id,
                    battle_deck: m.battle_deck,
                    respawn_time: m.respawn_time,
                    life_end_time: m.life_end_time,
                    group_id: m.group_id,
                    active_flag: m.active_flag,
                })
                .collect();

            HuntingGroundDbInfo {
                is_auto: g.is_auto,
                current_id: g.current_id,
                highest_id: g.highest_id,
                monster_info,
                pack_id: g.pack_id,
            }
        })
        .collect::<Vec<_>>();

    let response = HuntingGroundInfoListResponse {
        hunting_ground_info,
        ..Default::default()
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8916),
        active_login_event: vec![1, 2],
        ..Default::default()
    };

    let (route, code) = PacketCodeType::HuntingGroundInfoList.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
