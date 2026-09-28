use bd2::prost::Message;
use bd2::proto::proto_net::{CostumeDbInfo, EvilCastleRogueLikeCostumeUpgradeRequest, EvilCastleRogueLikeCostumeUpgradeResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::costume::costume_info;
use database::db::evil::evil_castle_rogue_like_info;
use sqlx::SqlitePool;
use tracing::info;

/// Spends real gold (RLDefaultTable.costume_upgrade_price, scaled up since
/// the table stores a small multiplier rather than a final cost) to bump a
/// costume's level.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeCostumeUpgradeRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeCostumeUpgradeRequest: {:?}", req);

    let price_mult = exceldb::get().rldefaulttable.all().first().map(|d| d.costume_upgrade_price).unwrap_or(2);
    let cost = price_mult * 50;

    let mut costume_info_out = None;
    if let Some(inven_index) = req.inven_index {
        if let Some(mut run) = evil_castle_rogue_like_info::get_one(pool, uid).await.ok().flatten() {
            if run.rogue_like_gold.unwrap_or(0) >= cost {
                if let Ok(c) = costume_info::get_by_index(pool, uid, inven_index).await {
                    let new_level = c.level.unwrap_or(0) + 1;
                    let _ = costume_info::set_level(pool, uid, inven_index, new_level).await;
                    costume_info_out = Some(CostumeDbInfo {
                        inven_index: c.inven_index,
                        id: c.id,
                        design_id: c.design_id,
                        level: Some(new_level),
                        ..Default::default()
                    });
                    run.rogue_like_gold = Some(run.rogue_like_gold.unwrap_or(0) - cost);
                    let _ = evil_castle_rogue_like_info::upsert(pool, &run).await;
                }
            }
        }
    }

    let response = EvilCastleRogueLikeCostumeUpgradeResponse { costume_info: costume_info_out, used_gold: Some(cost) };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeCostumeUpgrade.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
