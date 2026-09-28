use bd2::prost::Message;
use bd2::proto::proto_net::{PvpContentsItemRenewRequest, PvpContentsItemRenewResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pvp::pvp_deck_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, json_to_items};

/// Re-syncs and returns the deck's current per-char item/equip snapshot. No equip-recompute
/// logic exists in this codebase, so this returns exactly what's already stored rather than
/// recalculating anything — real data, just not actively "renewed".
pub async fn handle(pool: &SqlitePool, uid: i64, req: PvpContentsItemRenewRequest) -> GameResponse {
    info!("Handling PvpContentsItemRenewRequest: {:?}", req);

    let deck_type = req.deck_type.unwrap_or(0);
    let deck_item_info = pvp_deck_info::get_meta(pool, uid, deck_type)
        .await
        .ok()
        .flatten()
        .map(|m| json_to_items(&m.item_info_json))
        .unwrap_or_default();

    let response = PvpContentsItemRenewResponse { deck_item_info };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PvpContentsItemRenew.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
