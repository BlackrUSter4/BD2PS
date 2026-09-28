use bd2::prost::Message;
use bd2::proto::proto_net::{IbStageEndRequest, IbStageEndResponse, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::ib::{ib_cleared_dungeon, ib_play_state};
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, BATTLE_RESULT_WIN, STAGE_CLEAR_COIN};

/// Trusts the client-reported result, same as Colosseum's battle end — the client already
/// simulated the battle locally from the seed handed out in `IbStageStart`.
///
/// Simplification (documented, no real data to do better): with zero `IBStageTable`/
/// `IBDungeonTable` data captured, there's no known stage count per dungeon, so a win always
/// counts as clearing the whole dungeon rather than advancing to a next stage. A loss costs one
/// life and keeps you on the same stage.
pub async fn handle(pool: &SqlitePool, uid: i64, req: IbStageEndRequest) -> GameResponse {
    info!("Handling IbStageEndRequest: {:?}", req);

    let state_before = ib_play_state::get_or_create(pool, uid).await.unwrap_or_default();
    let won = req.battle_result.unwrap_or(0) == BATTLE_RESULT_WIN;
    let gain_coin = if won { STAGE_CLEAR_COIN } else { 0 };

    let (new_life, new_coin, next_stage_id) =
        ib_play_state::end_stage(pool, uid, won, gain_coin).await.unwrap_or((state_before.life, state_before.coin, state_before.stage_id));

    let is_first_clear = if won {
        ib_cleared_dungeon::mark_cleared(pool, uid, state_before.season, state_before.dungeon_id)
            .await
            .unwrap_or(false)
    } else {
        false
    };

    let response = IbStageEndResponse {
        reward_info_bundle: Some(RewardDbInfoBundle::default()),
        is_dungeon_cleared: Some(if won { 1 } else { 0 }),
        is_first_clear: Some(if is_first_clear { 1 } else { 0 }),
        life: Some(new_life),
        next_stage_id: Some(next_stage_id),
        gain_coin: Some(gain_coin),
        current_coin: Some(new_coin),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::IbStageEnd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
