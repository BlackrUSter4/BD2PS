use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRankUserInfo, EvilCastleRankingInfoRequest, EvilCastleRankingInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::evil_castle_info;
use sqlx::SqlitePool;
use tracing::info;

const RANK_LIMIT: i64 = 50;

async fn to_rank_user_info(pool: &SqlitePool, uid: i64, point: Option<i32>, rank: i32) -> EvilCastleRankUserInfo {
    // Real cross-account rank/point; user_id is the account's real in-game nickname
    // (see `logic::game::display_name`).
    EvilCastleRankUserInfo {
        owner_index: Some(uid),
        user_id: Some(crate::logic::game::display_name(pool, uid).await),
        user_exp: None,
        portrait_costume_id: None,
        portrait_costume_design_id: None,
        guild_base_info: None,
        rank: Some(rank),
        point,
        title_id: None,
        date: None,
    }
}

pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRankingInfoRequest) -> GameResponse {
    info!("Handling EvilCastleRankingInfoRequest: {:?}", req);

    let pack_id = req.pack_id.unwrap_or(0);

    let top = evil_castle_info::top_by_pack_id(pool, pack_id, RANK_LIMIT)
        .await
        .unwrap_or_default();
    let mut user_ranking_info = Vec::with_capacity(top.len());
    for (i, r) in top.iter().enumerate() {
        user_ranking_info.push(to_rank_user_info(pool, r.uid, r.point, i as i32 + 1).await);
    }

    let my_row = evil_castle_info::get_by_pack_id(pool, uid, pack_id)
        .await
        .ok()
        .flatten();
    let my_rank = evil_castle_info::rank_for(pool, uid, pack_id).await.unwrap_or(0) as i32;
    let my_ranking_info = match my_row {
        Some(r) => Some(to_rank_user_info(pool, uid, r.point, my_rank).await),
        None => None,
    };

    let response = EvilCastleRankingInfoResponse {
        user_ranking_info,
        my_ranking_info,
    };

    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::EvilCastleRakingList.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
