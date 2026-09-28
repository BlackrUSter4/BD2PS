use bd2::prost::Message;
use bd2::proto::proto_net::{TotalWarContentsItemRenewRequest, TotalWarContentsItemRenewResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Judgment call: `deck_item_info` (ContentsCharItemInfo, per-character equip loadout for
/// this event) has no persistence path within this handler's scope — that data belongs to
/// `TotalWarEquipSaveRequest`, which is a separate route not in this stub cluster (not yet
/// implemented anywhere in this project). Returns an honestly-empty list rather than
/// fabricating equip data.
pub async fn handle(
    _pool: &SqlitePool,
    _uid: i64,
    req: TotalWarContentsItemRenewRequest,
) -> GameResponse {
    info!("Handling TotalWarContentsItemRenewRequest: {:?}", req);

    let response = TotalWarContentsItemRenewResponse {
        deck_item_info: vec![],
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::TotalWarContentsItemRenew.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
