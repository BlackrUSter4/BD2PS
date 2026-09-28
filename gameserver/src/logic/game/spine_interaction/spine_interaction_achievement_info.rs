use bd2::prost::Message;
use bd2::proto::proto_net::{
    SpineInteractionAchievementDbInfo, SpineInteractionAchievementInfoRequest,
    SpineInteractionAchievementInfoResponse, SpineInteractionAchievementPointDbInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::spine_interaction::spine_interaction_achievement;
use sqlx::SqlitePool;
use std::collections::BTreeMap;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: SpineInteractionAchievementInfoRequest,
) -> GameResponse {
    info!("Handling SpineInteractionAchievementInfoRequest: {:?}", req);

    let rows = spine_interaction_achievement::list_for_uid(pool, uid)
        .await
        .unwrap_or_default();

    // Group rows into (interaction_group_id, group_id) -> point_id -> [motion_id].
    let mut groups: BTreeMap<(i32, i32), BTreeMap<i32, Vec<i32>>> = BTreeMap::new();
    for row in rows {
        groups
            .entry((row.interaction_group_id, row.group_id))
            .or_default()
            .entry(row.point_id)
            .or_default()
            .push(row.motion_id);
    }

    let spine_interaction_achievement_info = groups
        .into_iter()
        .map(|((interaction_group_id, group_id), points)| SpineInteractionAchievementDbInfo {
            interaction_group_id: Some(interaction_group_id),
            group_id: Some(group_id),
            point_info: points
                .into_iter()
                .map(|(point_id, motion_id)| SpineInteractionAchievementPointDbInfo {
                    point_id: Some(point_id),
                    motion_id,
                })
                .collect(),
        })
        .collect();

    let response = SpineInteractionAchievementInfoResponse {
        spine_interaction_achievement_info,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::SpineInteractionAchievementInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
