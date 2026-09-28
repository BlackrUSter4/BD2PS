use bd2::prost::Message;
use bd2::proto::proto_net::{CostumeBaseDbInfo, UserContentsInfoRequest, UserContentsInfoResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{
    friend::friend_info as friend_db, pvp::pvp_user_info as pvp_db,
    supporter::supporter_slot_info as supporter_db, user::user_info as user_db,
};
use sqlx::SqlitePool;
use tracing::info;

/// Real cross-account profile view. Pulls genuinely real data from every system that
/// already has real per-account persistence (User/Pvp/MonsterHunt/TotalWar/Friend/
/// Supporter). Left as documented placeholders (None/0) where pulling real data would
/// need re-verifying another system's exact schema well beyond this handler's own scope:
/// `guild_base_info`/`guild_raid_rank`/`guild_raid_score` (Guild), `room_info`/
/// `my_room_like_count` (MyRoom), the three `evil_castle_*_tower_top_floor` fields and
/// `evil_castle_rogue_like_level` (EvilCastle), `achievement_level` (Achievement),
/// `id_card_info` (IdCard), `like_count`, `total_battle_power`.
pub async fn handle(pool: &SqlitePool, uid: i64, req: UserContentsInfoRequest) -> GameResponse {
    info!("Handling UserContentsInfoRequest: {:?}", req);

    let target = req.target_owner_index.unwrap_or(uid);

    let user_row = user_db::get_user_info(pool, target).await.ok().and_then(|v| v.into_iter().next());

    let (pvp_season, pvp_vp, pvp_rank) = match pvp_db::get(pool, target).await.ok().flatten() {
        Some(p) => {
            let rank = pvp_db::rank_of(pool, target).await.unwrap_or(0);
            (Some(p.season), Some(p.vp), Some(rank))
        }
        None => (None, None, None),
    };

    let (monsterhunt_rank, monsterhunt_rank_top_percent) = {
        let (rank, percent) = super::super::monster::compute_rank(pool, target).await;
        (Some(rank), Some(percent))
    };

    let total_war_score = {
        let rows = database::db::total::total_war_info::get_total_war_info(pool, target).await.unwrap_or_default();
        rows.into_iter().next().map(|r| {
            let scores = super::super::total::parse_scores(&r);
            super::super::total::total_score(&scores)
        })
    };

    let is_friend = friend_db::get_by_uid_and_owner(pool, uid, target).await.ok().flatten()
        .map(|f| f.status == 0);

    let supporter_info = supporter_db::get_supporter_slot_info(pool, target).await.unwrap_or_default()
        .into_iter()
        .filter_map(|r| r.costume_id.map(|id| CostumeBaseDbInfo { id: Some(id), level: None, design_id: None }))
        .collect();

    let (user_id, title_id, portrait_costume_id, portrait_costume_design_id, greeting, is_all_private, options) =
        match &user_row {
            Some(r) => (
                r.user_id.clone().or_else(|| Some(target.to_string())),
                r.title_id,
                r.portrait_costume_id,
                r.portrait_costume_design_id,
                r.greeting.clone(),
                r.is_all_private != 0,
                r.privacy_options.as_deref().and_then(|s| serde_json::from_str::<Vec<i32>>(s).ok()).unwrap_or_default(),
            ),
            None => (Some(target.to_string()), None, None, None, None, false, vec![]),
        };
    
    let response = UserContentsInfoResponse {
        owner_index: Some(target),
        user_id,
        title_id,
        portrait_costume_id,
        greeting,
        pvp_season,
        pvp_vp,
        pvp_rank,
        monsterhunt_rank,
        like_count: None,
        is_all_private: Some(is_all_private),
        options,
        is_friend,
        room_info: None,
        total_war_score,
        total_battle_power: None,
        guild_base_info: None,
        my_room_like_count: None,
        portrait_costume_design_id,
        guild_raid_rank: None,
        guild_raid_score: None,
        evil_castle_greed_tower_top_floor: None,
        evil_castle_rage_tower_top_floor: None,
        evil_castle_envy_tower_top_floor: None,
        evil_castle_rogue_like_level: None,
        achievement_level: None,
        sort_id: vec![],
        id_card_info: None,
        monsterhunt_rank_top_percent,
        supporter_info,
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
    
    let (route, code) = PacketCodeType::UserContentsInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}