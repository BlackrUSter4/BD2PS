use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, RewardDbInfoBundle, SquareRewardRequest, SquareRewardResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, square::square_reward_info as db};
use sqlx::SqlitePool;
use tracing::info;

use super::{SQUARE_REWARD_COUNT, SQUARE_REWARD_ITEM_ID, SQUARE_REWARD_ITEM_TYPE};

/// Real once-per-calendar-day claim; reward contents are a placeholder (no SquareTable
/// exists anywhere in this project's schema).
pub async fn handle(pool: &SqlitePool, uid: i64, req: SquareRewardRequest) -> GameResponse {
    info!("Handling SquareRewardRequest: {:?}", req);

    let row = db::get_or_default(pool, uid).await;
    let today = chrono::Utc::now().date_naive().to_string();
    let already_claimed = row.last_claim_date.as_deref() == Some(today.as_str());

    let (id, reward_info_bundle) = if already_claimed {
        (None, RewardDbInfoBundle::default())
    } else {
        let _ = db::set_claimed(pool, uid, &today).await;
        let _ = item_info::grant(pool, uid, SQUARE_REWARD_ITEM_ID, SQUARE_REWARD_ITEM_TYPE, SQUARE_REWARD_COUNT).await;
        (
            Some(1),
            RewardDbInfoBundle {
                item_info: vec![ItemDbInfo {
                    id: Some(SQUARE_REWARD_ITEM_ID),
                    r#type: Some(SQUARE_REWARD_ITEM_TYPE),
                    count: Some(SQUARE_REWARD_COUNT),
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
    };

    let response = SquareRewardResponse {
        id,
        reward_info_bundle: Some(reward_info_bundle),
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::SquareReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
