use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeGrowthRequest, EvilCastleRogueLikeGrowthResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::evil::{evil_castle_rogue_like_growth_info, evil_castle_rogue_like_info};
use sqlx::SqlitePool;
use tracing::info;

/// Spends real Obsidian (RLGrowthTable's real priceCount) to raise a
/// permanent (across-runs) growth stat by one level.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeGrowthRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeGrowthRequest: {:?}", req);

    let mut new_level = 0;
    let mut obsidian = 0;
    if let Some(g) = req.growth_info {
        if let (Some(r#type), Some(target_level)) = (g.r#type, g.level) {
            if let Some(def) = exceldb::get().rlgrowthtable.get(target_level) {
                if def.growth_type == r#type {
                    if let Some(mut run) = evil_castle_rogue_like_info::get_one(pool, uid).await.ok().flatten() {
                        if run.obsidian.unwrap_or(0) >= def.price_count {
                            run.obsidian = Some(run.obsidian.unwrap_or(0) - def.price_count);
                            let _ = evil_castle_rogue_like_info::upsert(pool, &run).await;
                            let _ = evil_castle_rogue_like_growth_info::set_level(pool, uid, r#type, target_level).await;
                            new_level = target_level;
                        }
                        obsidian = run.obsidian.unwrap_or(0);
                    }
                }
            }
        }
    }

    let response = EvilCastleRogueLikeGrowthResponse { level: Some(new_level), obsidian: Some(obsidian) };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeGrowth.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
