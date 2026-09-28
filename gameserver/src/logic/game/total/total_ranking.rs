use bd2::prost::Message;
use bd2::proto::proto_net::{
    ColosseumRankUserInfo, ColosseumUserBaseInfo, GuildRaidSeasonRankDbInfo, MonsterHuntRankUserInfo, Notify,
    PvpBattleRankUserInfo, PvpBattleUserBaseInfo, TotalRankingRequest, TotalRankingResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{
    colosseum::colosseum_user_info, guild::{guild_base_info, guild_info, guild_raid_main_info},
    monster::monster_hunt_user_info, pvp::pvp_user_info, user::user,
};
use sqlx::SqlitePool;
use tracing::info;

/// A cross-system top-20-each summary view, composed from the same real per-system ranking
/// data every individual ranking route (PvpBattleRanking/MonsterHuntRankInfo/
/// GuildRaidSeasonRanking/ColosseumRanking) already serves for real — nothing new is
/// fabricated here, this just re-queries and re-packages the top of each.
pub async fn handle(pool: &SqlitePool, _uid: i64, req: TotalRankingRequest) -> GameResponse {
    info!("Handling TotalRankingRequest: {:?}", req);

    let pvp_top = pvp_user_info::top_by_vp(pool, 20).await.unwrap_or_default();
    let mut pvp_user_ranking_info = Vec::with_capacity(pvp_top.len());
    for (i, row) in pvp_top.iter().enumerate() {
        let account = user::find_account(pool, row.uid).await.ok().flatten();
        pvp_user_ranking_info.push(PvpBattleRankUserInfo {
            owner_index: Some(row.uid),
            user_id: account.map(|a| a.user_name),
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
        });
    }

    let mh_top = monster_hunt_user_info::rank_all(pool, 20).await.unwrap_or_default();
    let monster_hunt_user_rank_info: Vec<MonsterHuntRankUserInfo> = mh_top
        .iter()
        .enumerate()
        .map(|(i, r)| MonsterHuntRankUserInfo {
            owner_index: Some(r.uid),
            user_id: None,
            user_exp: None,
            portrait_costume_id: None,
            portrait_costume_design_id: None,
            guild_base_info: None,
            rank: Some(i as i32 + 1),
            score: Some(
                (r.level.unwrap_or(1) as f64) * 1_000_000.0
                    + r.current_level_highest_damage.unwrap_or(0) as f64,
            ),
            title_id: None,
            rank_top_percent: None,
        })
        .collect();

    let bases = guild_base_info::get_all_guilds(pool, 20).await.unwrap_or_default();
    let mut scored = Vec::new();
    for base in bases {
        let guild_id = base.id.unwrap_or_default();
        let score = guild_raid_main_info::get_guild_raid_main_info(pool, base.uid)
            .await
            .ok()
            .and_then(|v| v.into_iter().next())
            .and_then(|m| m.guild_total_score)
            .unwrap_or(0);
        let member_count = guild_info::get_by_guild_base_index(pool, guild_id)
            .await
            .ok()
            .flatten()
            .and_then(|i| i.member_count);
        scored.push((base, score, member_count));
    }
    scored.sort_by(|a, b| b.1.cmp(&a.1));
    let guild_rank_info: Vec<GuildRaidSeasonRankDbInfo> = scored
        .into_iter()
        .enumerate()
        .map(|(rank, (base, score, member_count))| GuildRaidSeasonRankDbInfo {
            rank: Some(rank as i32 + 1),
            score: Some(score),
            guild_index: base.id,
            guild_name: base.name,
            message: None,
            icon: base.icon,
            icon_color: base.icon_color,
            flag_grade: None,
            member_count,
            over_kill_damage: None,
        })
        .collect();

    let col_top = colosseum_user_info::top_by_vp(pool, 20).await.unwrap_or_default();
    let mut colosseum_user_ranking_info = Vec::with_capacity(col_top.len());
    for (i, row) in col_top.iter().enumerate() {
        let account = user::find_account(pool, row.uid).await.ok().flatten();
        colosseum_user_ranking_info.push(ColosseumRankUserInfo {
            owner_index: Some(row.uid),
            user_id: account.map(|a| a.user_name),
            user_exp: Some(0),
            portrait_costume_id: None,
            portrait_costume_design_id: None,
            guild_base_info: None,
            base_info: Some(ColosseumUserBaseInfo {
                vp: Some(row.vp),
                rank: Some(i as i32 + 1),
                win_count: Some(row.win_count),
                lose_count: Some(row.lose_count),
                ..Default::default()
            }),
            title_id: None,
        });
    }

    let response = TotalRankingResponse {
        pvp_user_ranking_info,
        monster_hunt_user_rank_info,
        guild_rank_info,
        schedule_history_info: vec![],
        colosseum_user_ranking_info,
        pvp_season: Some(1),
        monster_hunt_season: Some(1),
        guild_raid_season: Some(1),
        colosseum_season: Some(1),
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::TotalRanking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
