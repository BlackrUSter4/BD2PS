use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, PassDbInfo, PassInfoRequest, PassInfoResponse, PassRewardDbInfo};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pass::{pass_info as pass_db, pass_reward_info as reward_db};
use database::models::game::pass::{pass_info::PassInfo, pass_reward_info::PassRewardInfo};
use serde_json::Value;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account pass progress. `pass_info.json`/`pass_reward_info.json`'s captured rows
/// are all `exp:0, activePremium_1:false, basic:false` — a fresh-account snapshot, not master
/// definitions — so they're used purely as a one-time catalog seed for real per-account
/// `PassInfo`/`PassRewardInfo` rows (previously this handler just re-served that static
/// snapshot forever, so `pass_buy`/`pass_reward` had nothing real to persist into).
pub async fn handle(pool: &SqlitePool, uid: i64, _req: PassInfoRequest) -> GameResponse {
    info!("Handling PassInfoRequest");

    let data: Value = serde_json::from_str(include_str!("../../../../../data/starter/pass_info.json"))
        .expect("Failed to parse pass_info.json");

    if pass_db::get_pass_info(pool, uid).await.unwrap_or_default().is_empty() {
        for p in data.get("passInfo").and_then(|v| v.as_array()).cloned().unwrap_or_default() {
            if let Some(id) = p.get("id").and_then(|v| v.as_i64()).map(|v| v as i32) {
                let _ = pass_db::add_pass_info(pool, &PassInfo { index: 0, uid, id: Some(id), exp: Some(0), active_premium_1: Some(false) }).await;
            }
        }
        for r in data.get("passRewardInfo").and_then(|v| v.as_array()).cloned().unwrap_or_default() {
            if let (Some(pass_id), Some(id)) = (
                r.get("passId").and_then(|v| v.as_i64()).map(|v| v as i32),
                r.get("id").and_then(|v| v.as_i64()).map(|v| v as i32),
            ) {
                let _ = reward_db::add_pass_reward_info(pool, &PassRewardInfo { index: 0, uid, pass_id: Some(pass_id), id: Some(id), basic: Some(false), premium_1: Some(false) }).await;
            }
        }
    }

    let pass_info = pass_db::get_pass_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| PassDbInfo { id: r.id, exp: r.exp, active_premium_1: r.active_premium_1, ..Default::default() })
        .collect();

    let pass_reward_info = reward_db::get_pass_reward_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| PassRewardDbInfo { pass_id: r.pass_id, id: r.id, basic: r.basic, premium_1: r.premium_1, ..Default::default() })
        .collect();

    let response = PassInfoResponse { pass_info, pass_reward_info, ..Default::default() };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::PassInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
