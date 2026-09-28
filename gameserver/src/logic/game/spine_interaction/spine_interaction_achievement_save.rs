use bd2::prost::Message;
use bd2::proto::proto_net::{SpineInteractionAchievementSaveRequest, SpineInteractionAchievementSaveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::spine_interaction::spine_interaction_achievement;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Unlocks are upsert-only (see spine_interaction_achievement::unlock) — the client resaves its
/// full known state, and we never delete on save, only add newly-reported unlocks.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: SpineInteractionAchievementSaveRequest,
) -> GameResponse {
    info!("Handling SpineInteractionAchievementSaveRequest: {:?}", req);

    for group in &req.spine_interaction_achievement_info {
        let interaction_group_id = group.interaction_group_id.unwrap_or_default();
        let group_id = group.group_id.unwrap_or_default();
        for point in &group.point_info {
            let point_id = point.point_id.unwrap_or_default();
            for &motion_id in &point.motion_id {
                let _ = spine_interaction_achievement::unlock(
                    pool,
                    uid,
                    interaction_group_id,
                    group_id,
                    point_id,
                    motion_id,
                )
                .await;
            }
        }
    }

    let response = SpineInteractionAchievementSaveResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::SpineInteractionAchievementSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
