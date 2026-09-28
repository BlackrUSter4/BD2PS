use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRaidMemberRankDbInfo, GuildRaidMemberRankingRequest, GuildRaidMemberRankingResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_raid_main_info;
use sqlx::SqlitePool;
use tracing::info;

use super::common::my_guild;

/// Ranks the guild's real members by their own GuildRaidMainInfo.user_score
/// (computed fresh each call rather than relying on a separately-maintained
/// cache table, since nothing populates one yet).
pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidMemberRankingRequest) -> GameResponse {
    info!("Handling GuildRaidMemberRankingRequest: {:?}", req);

    let mut rank_info = Vec::new();
    if let Some((_info, base)) = my_guild(pool, uid).await {
        let guild_id = base.id.unwrap_or_default();
        let members = super::common::member_list(pool, guild_id).await;
        let mut scored: Vec<(i64, String, i64)> = Vec::new();
        for m in &members {
            if let Some(owner) = m.owner_index {
                let score = guild_raid_main_info::get_guild_raid_main_info(pool, owner)
                    .await
                    .ok()
                    .and_then(|v| v.into_iter().next())
                    .and_then(|r| r.user_score)
                    .unwrap_or(0);
                scored.push((owner, m.user_id.clone().unwrap_or_default(), score));
            }
        }
        scored.sort_by(|a, b| b.2.cmp(&a.2));
        for (rank, (owner, user_id, score)) in scored.into_iter().enumerate() {
            rank_info.push(GuildRaidMemberRankDbInfo {
                rank: Some(rank as i32 + 1),
                owner_index: Some(owner),
                user_id: Some(user_id),
                score: Some(score),
                portrait_costume_id: None,
                portrait_costume_design_id: None,
                title_id: None,
                over_kill_damage: None,
            });
        }
    }

    let user_rank_info = rank_info.iter().find(|r| r.owner_index == Some(uid)).cloned();

    let response = GuildRaidMemberRankingResponse { rank_info, user_rank_info };
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
    let (route, code) = PacketCodeType::GuildRaidMemberRanking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
