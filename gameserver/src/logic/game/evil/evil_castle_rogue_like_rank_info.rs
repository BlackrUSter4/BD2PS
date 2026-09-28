use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeRankInfoRequest, EvilCastleRogueLikeRankInfoResponse, EvilCastleRogueLikeRankUserInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::evil_castle_rogue_like_score_info;
use sqlx::SqlitePool;
use tracing::info;

const RANK_LIMIT: i64 = 50;

async fn to_rank_user_info(pool: &SqlitePool, uid: i64, score: Option<i32>, rank: i32, crystal_damage: Option<i64>) -> EvilCastleRogueLikeRankUserInfo {
    EvilCastleRogueLikeRankUserInfo {
        owner_index: Some(uid),
        user_id: Some(crate::logic::game::display_name(pool, uid).await),
        user_exp: None,
        portrait_costume_id: None,
        portrait_costume_design_id: None,
        guild_base_info: None,
        rank: Some(rank),
        score,
        title_id: None,
        crystal_damage,
    }
}

pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeRankInfoRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeRankInfoRequest: {:?}", req);
    let _ = req;

    let top = evil_castle_rogue_like_score_info::top_by_score(pool, RANK_LIMIT).await.unwrap_or_default();
    let mut user_rank_info = Vec::with_capacity(top.len());
    for (i, r) in top.iter().enumerate() {
        user_rank_info.push(to_rank_user_info(pool, r.uid, r.total_score, i as i32 + 1, r.crystal_damage).await);
    }

    let my = evil_castle_rogue_like_score_info::get_one(pool, uid).await.ok().flatten();
    let my_rank = evil_castle_rogue_like_score_info::rank_for(pool, uid).await.unwrap_or(0) as i32;
    let my_rank_info = match my {
        Some(r) => Some(to_rank_user_info(pool, uid, r.total_score, my_rank, r.crystal_damage).await),
        None => None,
    };

    let response = EvilCastleRogueLikeRankInfoResponse { user_rank_info, my_rank_info };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeRankInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
