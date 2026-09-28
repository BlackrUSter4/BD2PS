use bd2::prost::Message;
use bd2::proto::proto_net::{PvpBattleRankUserInfo, PvpBattleRankingRequest, PvpBattleRankingResponse, PvpBattleUserBaseInfo};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pvp::pvp_user_info;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: PvpBattleRankingRequest) -> GameResponse {
    info!("Handling PvpBattleRankingRequest: {:?}", req);

    // Real accounts only — deliberately not bot-padded, same convention as Colosseum/Guild
    // rankings (a fake leaderboard would mislead, unlike a fake sparring partner).
    let top = pvp_user_info::top_by_vp(pool, 100).await.unwrap_or_default();

    let mut user_ranking_info = Vec::with_capacity(top.len());
    let mut my_ranking_info = None;
    for (i, row) in top.iter().enumerate() {
        let account = database::db::user::user::find_account(pool, row.uid).await.ok().flatten();
        let user_id = account.map(|a| a.user_name).unwrap_or_else(|| format!("Player{}", row.uid));
        let info = PvpBattleRankUserInfo {
            owner_index: Some(row.uid),
            user_id: Some(user_id),
            user_exp: Some(0),
            portrait_costume_id: None,
            portrait_costume_design_id: None,
            guild_base_info: None,
            base_info: Some(PvpBattleUserBaseInfo {
                vp: Some(row.vp),
                rank: Some(i as i32 + 1),
                win_count: Some(row.win_count),
                lose_count: Some(row.lose_count),
            }),
            title_id: None,
        };
        if row.uid == uid {
            my_ranking_info = Some(info.clone());
        }
        user_ranking_info.push(info);
    }

    if my_ranking_info.is_none() {
        let user = pvp_user_info::get_or_create(pool, uid).await.unwrap_or_default();
        let rank = pvp_user_info::rank_of(pool, uid).await.unwrap_or(1);
        my_ranking_info = Some(PvpBattleRankUserInfo {
            owner_index: Some(uid),
            user_id: None,
            user_exp: Some(0),
            portrait_costume_id: None,
            portrait_costume_design_id: None,
            guild_base_info: None,
            base_info: Some(PvpBattleUserBaseInfo {
                vp: Some(user.vp),
                rank: Some(rank),
                win_count: Some(user.win_count),
                lose_count: Some(user.lose_count),
            }),
            title_id: None,
        });
    }

    let response = PvpBattleRankingResponse { user_ranking_info, my_ranking_info };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PvpBattleRanking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
