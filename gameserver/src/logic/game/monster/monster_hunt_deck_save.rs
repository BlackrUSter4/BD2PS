use bd2::prost::Message;
use bd2::proto::proto_net::{MonsterHuntDeckSaveRequest, MonsterHuntDeckSaveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::monster::monster_hunt_deck_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MonsterHuntDeckSaveRequest,
) -> GameResponse {
    info!("Handling MonsterHuntDeckSaveRequest: {:?}", req);

    for deck in &req.monster_hunt_deck_info {
        let team = deck.team.unwrap_or(0);
        let json = serde_json::to_string(&deck.deck_info).ok();
        let _ = db::upsert(pool, uid, team, json.as_deref(), deck.battle_power).await;
    }

    let response = MonsterHuntDeckSaveResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MonsterHuntDeckSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
