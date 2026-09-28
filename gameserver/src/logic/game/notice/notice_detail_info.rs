use bd2::prost::Message;
use bd2::proto::proto_net::{
    DefineNoticeContentsType, NoticeContentsInfo, NoticeDbInfo, NoticeDetailInfoRequest,
    NoticeDetailInfoResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::notice::notice_info::get_notice_info;
use serde_json::Value;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: NoticeDetailInfoRequest) -> GameResponse {
    info!(
        "Handling NoticeDetailInfoRequest for uid {}: {:?}",
        uid, req
    );


    let notice_rows = match get_notice_info(pool, uid).await {
        Ok(rows) => rows,
        Err(err) => {
            eprintln!("Error fetching NoticeInfo for uid {}: {:?}", uid, err);
            vec![]
        }
    };

    let selected = req
        .notice_id
        .and_then(|id| notice_rows.into_iter().find(|row| row.id == Some(id)));

 
    let notice_info = selected.map(|row| {
        
        let contents: Vec<NoticeContentsInfo> = row
            .notice_contents_info_index
            .as_ref()
            .and_then(|json_str| serde_json::from_str::<Vec<Value>>(json_str).ok())
            .map(|items| {
                items
                    .into_iter()
                    .filter_map(|item| {
                        let content_type = item
                            .get("type")
                            .and_then(|v| v.as_str())
                            .map(|s| match s {
                                "NCT_IMAGE" => DefineNoticeContentsType::NctImage,
                                _ => DefineNoticeContentsType::NctText,
                            })
                            .unwrap_or(DefineNoticeContentsType::NctText);

                        let value = item
                            .get("value")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());

                        Some(NoticeContentsInfo {
                            r#type: Some(content_type as i32),
                            value,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        NoticeDbInfo {
            id: row.id,
            notice_type: row.notice_type,
            title: row.title.clone(),
            thumbnail: row.thumbnail.clone(),
            start_time: row.start_time,
            end_time: row.end_time,
            web_url: row.web_url.clone(),
            notice_contents_info: contents,
            promotion_banner_id: row.promotion_banner_id,
            is_pin: row.is_pin,
            sort: row.sort,
            sub_type: row.sub_type,
            is_invert: row.is_invert,
        }
    });

   
    let response = NoticeDetailInfoResponse {
        notice_info, // optional
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
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::NoticeDetailInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
