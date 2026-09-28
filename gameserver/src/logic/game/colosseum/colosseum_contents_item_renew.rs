use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumContentsItemRenewRequest, ColosseumContentsItemRenewResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::{build_deck_item_info_list, default_notify};

/// "Renew" reads back the currently-saved deck item info as-is — no formula/table exists
/// for recomputing `connect_potential_costume` bonuses, so this returns real stored state
/// rather than inventing a recompute step.
pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumContentsItemRenewRequest) -> GameResponse {
    info!("Handling ColosseumContentsItemRenewRequest: {:?}", req);

    let response = ColosseumContentsItemRenewResponse {
        deck_item_info: build_deck_item_info_list(pool, uid).await,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumContentsItemRenew.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
