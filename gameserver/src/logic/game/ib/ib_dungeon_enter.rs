use bd2::prost::Message;
use bd2::proto::proto_net::{IbDungeonEnterRequest, IbDungeonEnterResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::ib::{ib_deck, ib_inventory, ib_play_state};
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, deck_info_proto, item_info_proto, play_info_proto, ensure_season, STARTING_LIFE};

pub async fn handle(pool: &SqlitePool, uid: i64, req: IbDungeonEnterRequest) -> GameResponse {
    info!("Handling IbDungeonEnterRequest: {:?}", req);

    ensure_season(pool, uid).await;
    let dungeon_id = req.dungeon_id.unwrap_or(1);
    let _ = ib_play_state::enter_dungeon(pool, uid, dungeon_id, STARTING_LIFE).await;
    let state = ib_play_state::get_or_create(pool, uid).await.unwrap_or_default();

    let items = ib_inventory::list(pool, uid).await.unwrap_or_default();
    let deck = ib_deck::list(pool, uid).await.unwrap_or_default();

    let response = IbDungeonEnterResponse {
        play_info: Some(play_info_proto(&state)),
        item_info: item_info_proto(&items),
        deck_info: deck_info_proto(&deck),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::IbDungeonEnter.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
