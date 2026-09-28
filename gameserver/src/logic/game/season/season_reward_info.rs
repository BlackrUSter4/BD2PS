use bd2::prost::Message;
use bd2::proto::proto_net::{
    Notify, SeasonRewardDbInfo, SeasonRewardInfoRequest, SeasonRewardInfoResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::season::season_reward_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account read of the previously-unused generic SeasonRewardInfo table (distinct from
/// the per-system season-reward tables already implemented for Pvp/Colosseum/MonsterHunt/
/// GuildRaid/EvilCastle). Was hardcoding the same 2 fake "already received" rewards for every
/// account — fixed. Correctly empty for a fresh account: no claim/write endpoint for this generic
/// table exists anywhere in this project's registered routes, so nothing populates it yet.
pub async fn handle(pool: &SqlitePool, uid: i64, req: SeasonRewardInfoRequest) -> GameResponse {
    info!("Handling SeasonRewardInfoRequest: {:?}", req);

    let reward_info = db::get_season_reward_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| SeasonRewardDbInfo { pack_id: r.pack_id, season: r.season, is_reward_received: r.is_reward_received })
        .collect();

    let response = SeasonRewardInfoResponse { reward_info };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::SeasonRewardInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
