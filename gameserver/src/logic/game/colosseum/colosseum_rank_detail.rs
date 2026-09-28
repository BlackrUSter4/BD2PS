use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumDeckFullInfo, ColosseumRankDetailRequest, ColosseumRankDetailResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::colosseum_user_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{build_deck_info_list, default_notify};

pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumRankDetailRequest) -> GameResponse {
    info!("Handling ColosseumRankDetailRequest: {:?}", req);

    let target = req.target_owner_index.unwrap_or(uid);
    let row = colosseum_user_info::get(pool, target).await.ok().flatten().unwrap_or_default();
    let deck_info = build_deck_info_list(pool, target).await;

    let response = ColosseumRankDetailResponse {
        owner_index: Some(target),
        win_count: Some(row.win_count),
        lose_count: Some(row.lose_count),
        deck_info: Some(ColosseumDeckFullInfo { deck_info, ..Default::default() }),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumRankDetail.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
