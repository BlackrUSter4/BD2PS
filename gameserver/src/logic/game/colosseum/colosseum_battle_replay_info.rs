use bd2::prost::Message;
use bd2::proto::proto_net::{
    ColosseumBattleReplayInfoRequest, ColosseumBattleReplayInfoResponse, ColosseumDeckFullInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::colosseum_battle_history;
use rand::Rng;
use sqlx::SqlitePool;
use tracing::info;

use super::{build_deck_info_list, default_notify};

/// This project doesn't persist a point-in-time deck/stat snapshot per battle (only the live
/// deck tables), so a replay reconstructs both sides from *current* state rather than what
/// was actually equipped at battle time — the closest available approximation, not a true
/// historical replay. `battle_random_seed` also isn't stored per-battle, so a fresh one is
/// returned (won't reproduce the original battle's RNG either).
pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumBattleReplayInfoRequest) -> GameResponse {
    info!("Handling ColosseumBattleReplayInfoRequest: {:?}", req);

    let battle_index = req.battle_inven_index.unwrap_or(0);
    let attack_rows = colosseum_battle_history::recent_by_uid(pool, uid, true, 1000).await.unwrap_or_default();
    let defense_rows = colosseum_battle_history::recent_by_uid(pool, uid, false, 1000).await.unwrap_or_default();
    let row = attack_rows
        .into_iter()
        .chain(defense_rows)
        .find(|r| r.index == battle_index);

    let (blue_uid, red_uid) = match &row {
        Some(r) if r.is_attacker => (uid, r.enemy_owner_index.unwrap_or(0)),
        Some(r) => (r.enemy_owner_index.unwrap_or(0), uid),
        None => (uid, 0),
    };

    let blue_deck_full_info = Some(ColosseumDeckFullInfo {
        deck_info: build_deck_info_list(pool, blue_uid).await,
        ..Default::default()
    });
    let red_deck_full_info = if red_uid > 0 {
        Some(ColosseumDeckFullInfo {
            deck_info: build_deck_info_list(pool, red_uid).await,
            ..Default::default()
        })
    } else {
        Some(ColosseumDeckFullInfo::default())
    };

    let response = ColosseumBattleReplayInfoResponse {
        blue_deck_full_info,
        red_deck_full_info,
        battle_random_seed: Some(rand::thread_rng().gen_range(i32::MIN..=i32::MAX)),
        world_buff_id: None,
        first_attack_team: Some(0),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumBattleReplayInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
