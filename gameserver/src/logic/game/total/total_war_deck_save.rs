use bd2::prost::Message;
use bd2::proto::proto_net::{TotalWarDeckSaveRequest, TotalWarDeckSaveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::total::total_war_deck_info as db;
use database::models::game::total::total_war_deck_info::TotalWarDeckInfo;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: TotalWarDeckSaveRequest,
) -> GameResponse {
    info!("Handling TotalWarDeckSaveRequest: {:?}", req);

    let items: Vec<TotalWarDeckInfo> = req
        .deck_info
        .iter()
        .map(|d| TotalWarDeckInfo {
            index: 0,
            uid,
            play_type: d.play_type,
            inven_index: d.inven_index,
            char_inven_index: d.char_inven_index,
        })
        .collect();

    if let Err(e) = db::replace_all(pool, uid, &items).await {
        tracing::error!("TotalWarDeckSave replace_all failed: {}", e);
    }

    let response = TotalWarDeckSaveResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::TotalWarDeckSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
