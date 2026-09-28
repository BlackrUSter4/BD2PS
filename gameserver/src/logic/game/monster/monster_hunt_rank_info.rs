use bd2::prost::Message;
use bd2::proto::proto_net::{
    MonsterHuntRankInfoRequest, MonsterHuntRankInfoResponse, MonsterHuntRankUserInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::monster::monster_hunt_user_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real cross-account ranking (like Colosseum/PvP/Guild/EvilCastle) — never bot-padded.
/// `user_id` is the account's real in-game nickname (see `logic::game::display_name`).
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MonsterHuntRankInfoRequest,
) -> GameResponse {
    info!("Handling MonsterHuntRankInfoRequest: {:?}", req);

    let all = db::rank_all(pool, 100).await.unwrap_or_default();
    let total = all.len().max(1);

    let mut user_rank_info: Vec<MonsterHuntRankUserInfo> = Vec::with_capacity(all.len());
    for (i, r) in all.iter().enumerate() {
        user_rank_info.push(MonsterHuntRankUserInfo {
            owner_index: Some(r.uid),
            user_id: Some(crate::logic::game::display_name(pool, r.uid).await),
            user_exp: None,
            portrait_costume_id: None,
            portrait_costume_design_id: None,
            guild_base_info: None,
            rank: Some((i + 1) as i32),
            score: Some(
                (r.level.unwrap_or(1) as f64) * 1_000_000.0
                    + r.current_level_highest_damage.unwrap_or(0) as f64,
            ),
            title_id: None,
            rank_top_percent: Some(((i + 1) as f64 / total as f64) * 100.0),
        });
    }

    let my_rank_info = user_rank_info.iter().find(|r| r.owner_index == Some(uid)).cloned();

    let response = MonsterHuntRankInfoResponse {
        user_rank_info,
        my_rank_info,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MonsterHuntRankInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
