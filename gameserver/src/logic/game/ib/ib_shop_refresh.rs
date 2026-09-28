use bd2::prost::Message;
use bd2::proto::proto_net::{IbShopRefreshRequest, IbShopRefreshResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::ib::{ib_play_state, ib_shop};
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, generate_shop_rows, shop_item_info_proto, SHOP_REFRESH_COST};

pub async fn handle(pool: &SqlitePool, uid: i64, req: IbShopRefreshRequest) -> GameResponse {
    info!("Handling IbShopRefreshRequest: {:?}", req);

    let _ = ib_play_state::add_coin(pool, uid, -SHOP_REFRESH_COST).await;
    let refresh_count = ib_play_state::incr_shop_reload(pool, uid).await.unwrap_or(0);

    let fresh = generate_shop_rows(uid);
    let _ = ib_shop::replace_all(pool, uid, &fresh).await;

    let state = ib_play_state::get_or_create(pool, uid).await.unwrap_or_default();

    let response = IbShopRefreshResponse {
        shop_item_info: shop_item_info_proto(&fresh),
        decrease_coin: Some(SHOP_REFRESH_COST),
        current_coin: Some(state.coin),
        refresh_count: Some(refresh_count),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::IbShopRefresh.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
