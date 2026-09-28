use bd2::prost::Message;
use bd2::proto::proto_net::{CharScoutInfoRequest, CharScoutInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::char::char_scout_info;
use rand::prelude::IndexedRandom;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: CharScoutInfoRequest) -> GameResponse {
    info!("Handling CharScoutInfoRequest: {:?}", req);

    let mut rows = char_scout_info::get_char_scout_info(pool, uid)
        .await
        .unwrap_or_default();

    if rows.is_empty() {
        let game_data = exceldb::get();
        let config = game_data.specialscoutinfotable.all().first();
        let appear_total = config.map(|c| c.appear_total_count).unwrap_or(2).max(1) as usize;
        let auto_reset_minute = config.map(|c| c.auto_reset_minute).unwrap_or(120) as i64;

        let mut rng = rand::thread_rng();
        let ids: Vec<i32> = game_data
            .chartable
            .all()
            .choose_multiple(&mut rng, appear_total)
            .map(|c| c.id)
            .collect();

        let now = chrono::Utc::now().timestamp_millis();
        let next_reset = now + auto_reset_minute * 60_000;
        let _ = char_scout_info::replace_lineup(pool, uid, &ids, 0, next_reset).await;
        rows = char_scout_info::get_char_scout_info(pool, uid)
            .await
            .unwrap_or_default();
    }

    let appear_char_id = rows.iter().map(|r| r.appear_char_id).collect();
    let scout_complete_char_id = rows.iter().map(|r| r.scout_complete_char_id).collect();
    let use_reset_count = rows.first().and_then(|r| r.use_reset_count);
    let next_auto_reset_time = rows.first().and_then(|r| r.next_auto_reset_time);

    let response = CharScoutInfoResponse {
        appear_char_id,
        use_reset_count,
        next_auto_reset_time,
        scout_complete_char_id,
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    // NOTE: the enum variant for this route is named `CharSpecialScoutInfo`,
    // not `CharScoutInfo` — same naming quirk as CharAwakeInfo above.
    let (route, code) = PacketCodeType::CharSpecialScoutInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
