use base64::{engine::general_purpose, Engine as _};
use bd2::prost::Message;
use bd2::proto::proto_net::{IbProcessDbInfo, IbStageStartRequest, IbStageStartResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::ib::{ib_deck, ib_inventory, ib_play_state};
use rand::Rng;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, deck_info_proto, item_info_proto};

/// Builds the client-simulated-battle payload (same async pattern as Colosseum: server hands
/// back a deterministic seed + both sides' snapshots, client simulates locally, then
/// `IbStageEnd` just reports the result back). No `IBStageTable`/`IBDungeonTable` enemy data
/// was captured, so the red (enemy) side is left empty — documented placeholder, not a bug.
pub async fn handle(pool: &SqlitePool, uid: i64, req: IbStageStartRequest) -> GameResponse {
    info!("Handling IbStageStartRequest: {:?}", req);

    let dungeon_id = req.dungeon_id.unwrap_or(0);
    let stage_id = req.stage_id.unwrap_or(0);
    let battle_mode = 0;
    let random_seed = rand::thread_rng().gen_range(i32::MIN..=i32::MAX);
    let _ = ib_play_state::start_stage(pool, uid, battle_mode, random_seed).await;

    let deck = ib_deck::list(pool, uid).await.unwrap_or_default();
    let items = ib_inventory::list(pool, uid).await.unwrap_or_default();

    let process = IbProcessDbInfo {
        battle_mode: Some(battle_mode),
        random_seed: Some(random_seed),
        dungeon_id: Some(dungeon_id),
        stage_id: Some(stage_id),
        blue_deck_info: deck_info_proto(&deck),
        blue_item_info: item_info_proto(&items),
        red_deck_info: vec![],
        red_item_info: vec![],
    };
    let process_info = general_purpose::STANDARD.encode(process.encode_to_vec());

    let response = IbStageStartResponse { process_info: Some(process_info) };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::IbStageStart.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
