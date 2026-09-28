use bd2::prost::Message;
use bd2::proto::proto_net::{MailDbInfo, MailHistoryInfoRequest, MailHistoryInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mail::mail_info as mail_db;
use sqlx::SqlitePool;
use std::collections::HashMap;
use tracing::info;

/// Real "already opened" mail history, paged the same way MailInfoRequest's live-mail listing
/// already groups multi-item mails into one MailDbInfo per inven_index.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MailHistoryInfoRequest) -> GameResponse {
    info!("Handling MailHistoryInfoRequest: {:?}", req);

    let start = req.start_inven_index.unwrap_or(0);
    let select_count = req.select_count.filter(|&c| c > 0).unwrap_or(20) as usize;

    let mails = mail_db::get_mail_info(pool, uid).await.unwrap_or_default();

    let mut grouped: HashMap<i64, MailDbInfo> = HashMap::new();
    for m in mails.into_iter().filter(|m| m.is_open.unwrap_or(false) && m.inven_index.unwrap_or(0) >= start) {
        if let Some(key) = m.inven_index {
            let entry = grouped.entry(key).or_insert_with(|| MailDbInfo {
                inven_index: Some(key),
                r#type: m.r#type,
                mail_id: m.mail_id,
                sender_text: Some(m.sender_text.clone().unwrap_or_default()),
                title_text: Some(m.title_text.clone().unwrap_or_default()),
                message_text: Some(m.message_text.clone().unwrap_or_default()),
                reward_expire_time: m.reward_expire_time,
                is_open: m.is_open,
                open_time: m.open_time,
                create_time: m.create_time,
                history_delete_time: m.history_delete_time,
                is_cash: m.is_cash,
                item_type: vec![],
                item_id: vec![],
                item_count: vec![],
                ..Default::default()
            });
            entry.item_type.push(m.item_type);
            entry.item_id.push(m.item_id);
            entry.item_count.push(m.item_count);
        }
    }

    let mut mail_info: Vec<MailDbInfo> = grouped.into_values().collect();
    mail_info.sort_by_key(|m| m.inven_index.unwrap_or_default());
    mail_info.truncate(select_count);

    let total_count = mail_info.len() as i32;

    let response = MailHistoryInfoResponse { mail_info, total_count: Some(total_count) };

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

    let (route, code) = PacketCodeType::MailHistoryInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
