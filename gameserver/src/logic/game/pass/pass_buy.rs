use bd2::prost::Message;
use bd2::proto::proto_net::{PassBuyRequest, PassBuyResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, pass::pass_info as pass_db};
use sqlx::SqlitePool;
use tracing::info;

/// Real premium-pass purchase: consumes the real PassBuyTable cost (matched by the pass's
/// PassTable.pass_level_group_id — judgment call, no direct pass_id->PassBuyTable FK exists)
/// and marks the account's real PassInfo row as premium.
pub async fn handle(pool: &SqlitePool, uid: i64, req: PassBuyRequest) -> GameResponse {
    info!("Handling PassBuyRequest: {:?}", req);

    let mut exp = None;
    if let Some(pass_id) = req.pass_id {
        let game_data = data::exceldb::get();
        if let Some(pass_def) = game_data.passtable.get(pass_id) {
            if let Some(buy_def) = game_data.passbuytable.by_group(pass_def.pass_level_group_id).next() {
                if let Some(item_id) = buy_def.buy_item_id {
                    let _ = item_info::consume(pool, uid, item_id, buy_def.buy_item_count).await;
                }
            }
        }

        let _ = pass_db::set_active_premium_1(pool, uid, pass_id).await;
        exp = pass_db::get_by_pass_id(pool, uid, pass_id).await.ok().flatten().and_then(|r| r.exp);
    }

    let response = PassBuyResponse { exp };

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

    let (route, code) = PacketCodeType::PassBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
