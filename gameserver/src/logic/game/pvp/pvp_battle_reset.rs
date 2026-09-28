use bd2::prost::Message;
use bd2::proto::proto_net::{PvpBattleResetRequest, PvpBattleResetResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pvp::pvp_user_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, now_ms};

pub async fn handle(pool: &SqlitePool, uid: i64, req: PvpBattleResetRequest) -> GameResponse {
    info!("Handling PvpBattleResetRequest: {:?}", req);

    let now = now_ms();
    let deck_type = req.deck_type.unwrap_or(0);
    let _ = pvp_user_info::reset_deck_season(pool, uid, deck_type, now).await;
    let user = pvp_user_info::get_or_create(pool, uid).await.unwrap_or_default();

    let response = PvpBattleResetResponse {
        deck_season_attack_reset_date: user.deck_season_attack_reset_time,
        deck_season_defense_reset_date: user.deck_season_defense_reset_time,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PvpBattleReset.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
