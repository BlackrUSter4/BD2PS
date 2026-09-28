use bd2::prost::Message;
use bd2::proto::proto_net::{MailDbInfo, MailInfoRequest, MailInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mail::mail_info::get_mail_info;
use sqlx::SqlitePool;
use std::collections::HashMap;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: MailInfoRequest) -> GameResponse {
    info!("Handling MailInfoRequest: {:?}", req);

    // Fetch all mail rows for this user
    let mails = match get_mail_info(pool, uid).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Failed to load mail info for uid {}: {:?}", uid, e);
            vec![]
        }
    };

    let mut grouped: HashMap<i64, MailDbInfo> = HashMap::new();
    for m in mails {
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

    let mut mail_info_proto: Vec<MailDbInfo> = grouped.into_values().collect();

    // Sort by inven_index (so order matches the DB)
    mail_info_proto.sort_by_key(|m| m.inven_index.unwrap_or_default());

    let total_count = mail_info_proto.len() as i32;
    let max_inven_index = mail_info_proto
        .iter()
        .filter_map(|m| m.inven_index)
        .max()
        .unwrap_or_default();

    // Build the protobuf response
    let response = MailInfoResponse {
        mail_info: mail_info_proto,
        total_count: Some(total_count),
        max_inven_index: Some(max_inven_index),
        ..Default::default()
    };

    let resp_bytes = response.encode_to_vec();

    // Notify same as others
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

    let (route, code) = PacketCodeType::MailInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
