use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleEndSeasonDbInfo, EvilCastleEndSeasonTotalDbInfo, EvilCastleRewardInfoRequest, EvilCastleRewardInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::{evil_castle_end_season_info, evil_castle_end_season_total_info};
use sqlx::SqlitePool;
use tracing::info;

/// No table anywhere maps a season-end tier to a specific reward list, so
/// unclaimed season-end rows grant a documented-placeholder amount (same
/// "item id 4 / type 1 = gold" convention established since the Battle
/// round), rather than fabricating a bigger "real-looking" bundle.
const PLACEHOLDER_SEASON_GOLD: i32 = 500;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRewardInfoRequest) -> GameResponse {
    info!("Handling EvilCastleRewardInfoRequest: {:?}", req);

    let end_season_rows = evil_castle_end_season_info::get_evil_castle_end_season_info(pool, uid)
        .await
        .unwrap_or_default();
    let end_season_info: Vec<EvilCastleEndSeasonDbInfo> = end_season_rows
        .iter()
        .map(|r| EvilCastleEndSeasonDbInfo {
            rank: None,
            stage_index: r.stage_index,
            point: r.point,
            is_rewarded: r.is_rewarded,
            reward_info: vec![],
        })
        .collect();

    let total_row = evil_castle_end_season_total_info::get_evil_castle_end_season_total_info(pool, uid)
        .await
        .ok()
        .and_then(|v| v.into_iter().next());
    let end_season_total_info = total_row.as_ref().map(|t| EvilCastleEndSeasonTotalDbInfo {
        rank: None,
        point: t.point,
        is_rewarded: t.is_rewarded,
        reward_info: vec![],
    });

    let has_unclaimed = end_season_rows.iter().any(|r| r.is_rewarded != Some(true))
        || total_row.as_ref().map(|t| t.is_rewarded != Some(true)).unwrap_or(false);

    let reward_info_bundle = if has_unclaimed {
        let bundle = super::grant_rewards(pool, uid, &[4], &[1], &[PLACEHOLDER_SEASON_GOLD]).await;
        for r in &end_season_rows {
            if r.is_rewarded != Some(true) {
                let _ = evil_castle_end_season_info::mark_latest_rewarded(pool, uid).await;
            }
        }
        if let Some(t) = &total_row {
            if t.is_rewarded != Some(true) {
                let _ = evil_castle_end_season_total_info::mark_latest_rewarded(pool, uid).await;
            }
        }
        Some(bundle)
    } else {
        None
    };

    let response = EvilCastleRewardInfoResponse {
        end_season_info,
        end_season_total_info,
        reward_info_bundle,
    };

    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::EvilCastleRewardInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
