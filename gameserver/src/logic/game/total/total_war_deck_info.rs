use bd2::prost::Message;
use bd2::proto::proto_net::{TotalWarDeckDbInfo, TotalWarDeckInfoRequest, TotalWarDeckInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::total::total_war_deck_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: TotalWarDeckInfoRequest,
) -> GameResponse {
    info!("Handling TotalWarDeckInfoRequest: {:?}", req);

    let rows = db::get_total_war_deck_info(pool, uid).await.unwrap_or_default();
    let deck_info = rows
        .into_iter()
        .map(|r| TotalWarDeckDbInfo {
            play_type: r.play_type,
            inven_index: r.inven_index,
            char_inven_index: r.char_inven_index,
        })
        .collect();

    let response = TotalWarDeckInfoResponse {
        deck_info,
        contents_item_info: vec![],
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::TotalWarDeckInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
