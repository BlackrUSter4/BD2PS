use bd2::prost::Message;
use bd2::proto::proto_net::{EventScheduleDbInfo, TotalWarBattleStartRequest, TotalWarBattleStartResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use rand::Rng;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Client-simulates/server-trusts pattern (same as Battle/Colosseum/Ib/Guild-raid): the
/// server hands out a random seed and an event-active schedule window; the actual battle
/// runs client-side and reports its result via TotalWarBattleEnd. `blue_char_info`/
/// `red_char_info` are real — the request already carries the client's own full
/// `BattleCharDbInfo` for both teams (it built these locally, same as every other
/// client-simulates battle in this project), so the response just echoes them back rather
/// than trying to reconstruct the same data server-side from inven indices alone.
/// `buff_stat_info` stays empty: that would need real stat-calculation formulas this project
/// has never modeled anywhere, not just a lookup.
pub async fn handle(
    _pool: &SqlitePool,
    _uid: i64,
    req: TotalWarBattleStartRequest,
) -> GameResponse {
    info!("Handling TotalWarBattleStartRequest: {:?}", req);

    let seed = rand::rng().random_range(i32::MIN..=i32::MAX);
    let blue_char_info = req.blue_char_info.clone();
    let red_char_info = req.red_char_info.clone();

    let response = TotalWarBattleStartResponse {
        battle_random_seed: Some(seed),
        blue_char_info,
        red_char_info,
        buff_stat_info: vec![],
        event_schedule_info: Some(EventScheduleDbInfo {
            id: Some(1),
            event_type: Some(0),
            event_id: req.battle_index,
            event_sub_id: Some(0),
            start_date: Some(1_700_000_000_000),
            end_date: Some(2_000_000_000_000),
            is_active: Some(true),
        }),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::TotalWarBattleStart.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
