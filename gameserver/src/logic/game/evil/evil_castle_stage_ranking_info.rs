use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleStageRankingInfoRequest, EvilCastleStageRankingInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::evil_castle_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleStageRankingInfoRequest) -> GameResponse {
    info!("Handling EvilCastleStageRankingInfoRequest: {:?}", req);

    let pack_id = req.pack_id.unwrap_or(0);

    let my_row = evil_castle_info::get_by_pack_id(pool, uid, pack_id)
        .await
        .ok()
        .flatten();
    let rank = evil_castle_info::rank_for(pool, uid, pack_id).await.unwrap_or(0) as i32;
    let total = evil_castle_info::get_evil_castle_info(pool, uid)
        .await
        .unwrap_or_default();
    let total_rank = evil_castle_info::rank_for(pool, uid, pack_id).await.unwrap_or(0) as i32;
    let total_point: i32 = total.iter().filter_map(|r| r.point).sum();

    let response = EvilCastleStageRankingInfoResponse {
        rank: Some(rank),
        point: my_row.and_then(|r| r.point),
        total_rank: Some(total_rank),
        total_point: Some(total_point),
    };

    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::EvilCastleStageRankingInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
