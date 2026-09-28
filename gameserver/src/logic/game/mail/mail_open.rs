use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, MailOpenRequest, MailOpenResponse, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, mail::mail_info as mail_db};
use sqlx::SqlitePool;
use tracing::info;

/// Real mail open: grants every real item attached to each requested mail (a mail can carry
/// several items, each its own MailInfo row sharing the same InvenIndex) and marks them opened.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MailOpenRequest) -> GameResponse {
    info!("Handling MailOpenRequest: {:?}", req);

    let now = chrono::Utc::now().timestamp_millis();
    let mut item_infos = Vec::new();

    for inven_index in &req.inven_index {
        let rows = mail_db::get_by_inven_index(pool, uid, *inven_index).await.unwrap_or_default();
        for row in &rows {
            if row.is_open.unwrap_or(false) {
                continue;
            }
            if row.item_id != 0 {
                let _ = item_info::grant(pool, uid, row.item_id, row.item_type, row.item_count.max(1)).await;
                item_infos.push(ItemDbInfo { id: Some(row.item_id), r#type: Some(row.item_type), count: Some(row.item_count.max(1)), ..Default::default() });
            }
        }
        let _ = mail_db::mark_opened(pool, uid, *inven_index, now).await;
    }

    let response = MailOpenResponse {
        reward_info_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
        first_auto_revive_set_char_inven_index: None,
    };

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

    let (route, code) = PacketCodeType::MailOpen.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
