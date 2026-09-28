use bd2::prost::Message;
use bd2::proto::proto_net::{
    CafeteriaIntroductionStoryRewardRequest, CafeteriaIntroductionStoryRewardResponse, ItemDbInfo,
    RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{cafeteria::cafeteria_info, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

/// Claim-once, real reward: `CafeteriaDefaultTable.visualNovelEndReward{Id,Type,Count}`.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: CafeteriaIntroductionStoryRewardRequest,
) -> GameResponse {
    info!("Handling CafeteriaIntroductionStoryRewardRequest: {:?}", req);

    let mut info_row = match cafeteria_info::get_or_create(pool, uid).await {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("CafeteriaIntroductionStoryReward get_or_create failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let mut item_infos = Vec::new();
    if info_row.intro_story_reward_claimed == 0 {
        if let Some(default) = data::exceldb::get().cafeteriadefaulttable.all().first() {
            let _ = item_info::grant(
                pool,
                uid,
                default.visual_novel_end_reward_id,
                default.visual_novel_end_reward_type,
                default.visual_novel_end_reward_count,
            )
            .await;
            item_infos.push(ItemDbInfo {
                id: Some(default.visual_novel_end_reward_id),
                r#type: Some(default.visual_novel_end_reward_type),
                count: Some(default.visual_novel_end_reward_count),
                ..Default::default()
            });
        }
        info_row.intro_story_reward_claimed = 1;
        if let Err(e) = cafeteria_info::update_cafeteria_info(pool, &info_row).await {
            tracing::error!("CafeteriaIntroductionStoryReward update failed: {}", e);
        }
    }

    let response = CafeteriaIntroductionStoryRewardResponse {
        reward_info: Some(RewardDbInfoBundle {
            item_info: item_infos,
            ..Default::default()
        }),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CafeteriaIntroductionStoryReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
