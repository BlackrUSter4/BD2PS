use bd2::prost::Message;
use bd2::proto::proto_net::{TotalWarDeckPresetSlotAddRequest, TotalWarDeckPresetSlotAddResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::total::total_war_deck_preset_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real cap: TotalWarDefaultTable.total_war_preset_max_count (real config, predates this
/// project) bounds how many slots an account can ever have.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: TotalWarDeckPresetSlotAddRequest,
) -> GameResponse {
    info!("Handling TotalWarDeckPresetSlotAddRequest: {:?}", req);

    let add_count = req.add_count.unwrap_or(1).max(0);
    let max_count = data::exceldb::get()
        .totalwardefaulttable
        .all()
        .first()
        .map(|d| d.total_war_preset_max_count)
        .unwrap_or(i32::MAX);

    let mut next = db::max_slot(pool, uid).await.unwrap_or(0);
    for _ in 0..add_count {
        if next >= max_count {
            break;
        }
        next += 1;
        let _ = db::add_slot(pool, uid, next).await;
    }

    let response = TotalWarDeckPresetSlotAddResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::TotalWarDeckPresetSlotAdd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
