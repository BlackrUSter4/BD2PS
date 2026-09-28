use bd2::prost::Message;
use bd2::proto::proto_net::{CashMailInfoRequest, CashMailInfoResponse, MailDbInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mail::mail_info::get_mail_info;
use sqlx::SqlitePool;
use std::collections::HashMap;
use tracing::info;

/// Real filtered view over the same real MailInfo rows mail_info.rs reads (Cash Mail = the
/// `IsCash` subset), instead of the previously-unused CashMailInfo starter-seed table (a
/// JSON-imported per-account cache with no runtime write path). Was hardcoding an always-empty
/// response despite real mail already carrying the IsCash flag needed to serve this correctly.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CashMailInfoRequest) -> GameResponse {
    info!("Handling CashMailInfoRequest: {:?}", req);

    let mails = get_mail_info(pool, uid).await.unwrap_or_default();

    let mut grouped: HashMap<i64, MailDbInfo> = HashMap::new();
    for m in mails.into_iter().filter(|m| m.is_cash == Some(true)) {
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

    let total_count = mail_info.len() as i32;
    let max_inven_index = mail_info.iter().filter_map(|m| m.inven_index).max().unwrap_or_default();

    let response = CashMailInfoResponse {
        mail_info,
        total_count: Some(total_count),
        max_inven_index: Some(max_inven_index),
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::CashMailInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
