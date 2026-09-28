use bd2::prost::Message;
use bd2::proto::proto_net::{
    DeckDbInfo, MonsterHuntDeckDbInfo, MonsterHuntDeckInfoRequest, MonsterHuntDeckInfoResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::monster::monster_hunt_deck_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MonsterHuntDeckInfoRequest,
) -> GameResponse {
    info!("Handling MonsterHuntDeckInfoRequest: {:?}", req);

    let rows = db::get_monster_hunt_deck_info(pool, uid).await.unwrap_or_default();
    let monster_hunt_deck_info = rows
        .into_iter()
        .map(|r| MonsterHuntDeckDbInfo {
            team: r.team,
            deck_info: r
                .deck_info_index
                .as_deref()
                .and_then(|s| serde_json::from_str::<Vec<DeckDbInfo>>(s).ok())
                .unwrap_or_default(),
            battle_power: r.battle_power,
        })
        .collect();

    let response = MonsterHuntDeckInfoResponse {
        monster_hunt_deck_info,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MonsterHuntDeckInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
