use bd2::prost::Message;
use bd2::proto::proto_net::{
    MonsterHuntScheduleInfo as MonsterHuntScheduleInfoProto, MonsterHuntScheduleInfoRequest,
    MonsterHuntScheduleInfoResponse, SeasonInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_boss_id, default_notify};

/// The one real MonsterHuntTable boss is treated as an always-active season (no schedule
/// table exists to say otherwise — start/end timestamps are a wide placeholder window,
/// season number and rank_reward_group_id (1) use MonsterHuntRankTable's real group id).
pub async fn handle(
    _pool: &SqlitePool,
    _uid: i64,
    req: MonsterHuntScheduleInfoRequest,
) -> GameResponse {
    info!("Handling MonsterHuntScheduleInfoRequest: {:?}", req);

    let boss_id = default_boss_id();

    let schedule = MonsterHuntScheduleInfoProto {
        season_info: Some(SeasonInfo {
            season: Some(1),
            start_time: Some(1_700_000_000_000),
            end_time: Some(2_000_000_000_000),
            error_flag: Some(false),
            return_flag: Some(false),
            rank_reward_group_id: Some(1),
        }),
        monster_hunt_id: Some(boss_id),
        info_open_day: Some(0),
        calculate_end_date: Some(2_000_000_000_000),
        error_flag: Some(false),
        independent_flag: Some(false),
        rank_reward_group_id: Some(1),
    };

    let response = MonsterHuntScheduleInfoResponse {
        monster_hunt_schedule_info: vec![schedule],
        start_regular_season: Some(1),
        season_history: vec![],
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MonsterHuntScheduleInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
