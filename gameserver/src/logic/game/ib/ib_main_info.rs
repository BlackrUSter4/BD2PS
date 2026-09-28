use bd2::prost::Message;
use bd2::proto::proto_net::{IbMainInfoRequest, IbMainInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::ib::ib_cleared_dungeon;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, ensure_season, play_info_proto, REGULAR_SEASON};

pub async fn handle(pool: &SqlitePool, uid: i64, req: IbMainInfoRequest) -> GameResponse {
    info!("Handling IbMainInfoRequest: {:?}", req);

    let (is_season_reset, state) = ensure_season(pool, uid).await;
    let cleared = ib_cleared_dungeon::list_for_season(pool, uid, state.season).await.unwrap_or_default();

    let response = IbMainInfoResponse {
        play_info: Some(play_info_proto(&state)),
        is_season_reset: Some(is_season_reset),
        cleared_dungeon_id: cleared,
        regular_season: Some(REGULAR_SEASON),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::IbMainInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
