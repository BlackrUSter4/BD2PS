use bd2::prost::Message;
use bd2::proto::proto_net::{MonsterDbInfo, MonsterInfoRequest, MonsterInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::monster::monster_info::get_monster_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, _req: MonsterInfoRequest) -> GameResponse {
    info!("Handling MonsterInfoRequest for uid: {}", uid);

    let monster_infos = match get_monster_info(pool, uid).await {
        Ok(list) => list,
        Err(err) => {
            eprintln!("Error fetching MonsterInfo: {:?}", err);
            vec![]
        }
    };

    let monster_info_proto: Vec<MonsterDbInfo> = monster_infos
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

    let response = MonsterInfoResponse {
        monster_info: monster_info_proto,
        ..Default::default()
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2],
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8916),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::MonsterInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
