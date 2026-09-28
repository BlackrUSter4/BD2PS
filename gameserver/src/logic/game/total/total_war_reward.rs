use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, RewardDbInfoBundle, TotalWarRewardRequest, TotalWarRewardResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, total::total_war_info as db};
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, parse_scores, total_score};

/// Claim every real score-threshold tier (TotalWarRewardTable, real data) the account's
/// cumulative score has reached but hasn't been paid out for yet.
pub async fn handle(pool: &SqlitePool, uid: i64, req: TotalWarRewardRequest) -> GameResponse {
    info!("Handling TotalWarRewardRequest: {:?}", req);

    let mut row = match db::get_or_create(pool, uid).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("TotalWarReward get_or_create failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let score = total_score(&parse_scores(&row));
    let mut claimed: Vec<i32> = row
        .claimed_reward_ids
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default();

    let mut item_infos = Vec::new();
    for tier in data::exceldb::get().totalwarrewardtable.iter() {
        if (score as f32) < tier.score || claimed.contains(&tier.id) {
            continue;
        }
        let _ = item_info::grant(pool, uid, tier.reward_id, tier.reward_type, tier.reward_count).await;
        item_infos.push(ItemDbInfo {
            id: Some(tier.reward_id),
            r#type: Some(tier.reward_type),
            count: Some(tier.reward_count),
            ..Default::default()
        });
        claimed.push(tier.id);
    }

    row.claimed_reward_ids = serde_json::to_string(&claimed).ok();
    if let Err(e) = db::update_total_war_info(pool, &row).await {
        tracing::error!("TotalWarReward update failed: {}", e);
    }

    let response = TotalWarRewardResponse {
        reward_info_bundle: Some(RewardDbInfoBundle {
            item_info: item_infos,
            ..Default::default()
        }),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::TotalWarReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
