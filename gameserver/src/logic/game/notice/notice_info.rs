use bd2::prost::Message;
use bd2::proto::proto_net::{
    DefineNoticeType, NoticeDbInfo, NoticeInfoRequest, NoticeInfoResponse, Notify,
};
use crypto::network::GameResponse;
use tracing::info;

pub async fn handle(req: NoticeInfoRequest) -> GameResponse {
    info!("NoticeInfo request - last_seq: {:?}", req.notice_last_seq);

    let response = NoticeInfoResponse {
        notice_info: get_active_notices(),
    };

    let notify = build_notify(8818);

    GameResponse::success("NoticeInfo", &response.encode_to_vec(), 0).with_notify(&notify)
}

fn get_active_notices() -> Vec<NoticeDbInfo> {
    vec![
        notice(
            8818,
            DefineNoticeType::NtNotice,
            "BrownDust2 Halloween Party Song 『La 'Rou' la li la』 Reveal!",
            "https://bd2-cdn.akamaized.net/notice/live/1760674362070_notice.png",
            1760674298000,
            1761922778000,
        ),
        notice(
            8817,
            DefineNoticeType::NtNotice,
            "BrownDust2 Live Stream Reward Announcement",
            "https://bd2-cdn.akamaized.net/notice/live/1760674284396_notice.png",
            1760674214000,
            1760972354000,
        ),
        notice(
            8816,
            DefineNoticeType::NtDeveloperNote,
            "37th Developer Notes: Halloween Update",
            "https://bd2-cdn.akamaized.net/notice/live/1760674200226_developer.png",
            1760674020000,
            1761922740000,
        ),
        notice(
            8803,
            DefineNoticeType::NtEvent,
            "Quiz Challenge with the Moon Rabbits! Results Announcement",
            "https://bd2-cdn.akamaized.net/notice/live/1760502729526_event.png",
            1760504429000,
            1761145149000,
        ),
        notice(
            8795,
            DefineNoticeType::NtUpdate,
            "Notice of Live Update on October 16th (UTC)",
            "https://bd2-cdn.akamaized.net/notice/live/1760424844869_notice.png",
            1760425244000,
            1761836384000,
        ),
    ]
}

fn notice(
    id: i32,
    notice_type: DefineNoticeType,
    title: &str,
    thumbnail: &str,
    start_time: i64,
    end_time: i64,
) -> NoticeDbInfo {
    NoticeDbInfo {
        id: Some(id),
        notice_type: Some(notice_type as i32),
        title: Some(title.to_string()),
        thumbnail: Some(thumbnail.to_string()),
        start_time: Some(start_time),
        end_time: Some(end_time),
        web_url: Some(String::new()),
        notice_contents_info: vec![],
        promotion_banner_id: Some(0),
        is_pin: Some(true),
        sort: Some(0),
        sub_type: Some(0),
        is_invert: Some(false),
    }
}

fn build_notify(latest_seq: i32) -> Notify {
    Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: None,
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(latest_seq),
        ..Default::default()
    }
}

// If you want to load from a database later:
#[allow(dead_code)]
async fn get_notices_from_db(_last_seq: i32) -> Vec<NoticeDbInfo> {
    // TODO: Replace with actual DB query
    // sqlx::query_as!(NoticeDbInfo, "SELECT * FROM notices WHERE id > ?", last_seq)
    //     .fetch_all(&pool)
    //     .await
    //     .unwrap_or_default()
    get_active_notices()
}
