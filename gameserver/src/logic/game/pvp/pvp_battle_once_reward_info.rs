use bd2::prost::Message;
use bd2::proto::proto_net::{PvpBattleOnceRewardInfoRequest, PvpBattleOnceRewardInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pvp::pvp_user_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, parse_claimed};

pub async fn handle(pool: &SqlitePool, uid: i64, req: PvpBattleOnceRewardInfoRequest) -> GameResponse {
    info!("Handling PvpBattleOnceRewardInfoRequest: {:?}", req);

    let user = pvp_user_info::get_or_create(pool, uid).await.unwrap_or_default();
    let once_reward_info = parse_claimed(&user.once_reward_claimed);

    let response = PvpBattleOnceRewardInfoResponse { once_reward_info };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PvpBattleOnceRewardInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
