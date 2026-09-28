use bd2::prost::Message;
use bd2::proto::proto_net::{RoomChatReportRequest, RoomChatReportResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::room_chat::{room_chat_block, room_chat_report};
use sqlx::SqlitePool;
use tracing::info;

/// Reports are tallied against the account being reported (the target), not the reporter — the
/// only fields `RoomChatReportAndBlockInfoResponse` returns for "self" are `report_user_info`
/// (how many times *I've* been reported) and `penalty_info` (an active penalty on *me*), and
/// `RoomChatReportAndBlockInfoRequest` carries no target — it's asking about the caller's own
/// standing. No table anywhere defines a report threshold or penalty duration; both are
/// documented placeholders in `database::db::room_chat::room_chat_report`.
pub async fn handle(pool: &SqlitePool, uid: i64, req: RoomChatReportRequest) -> GameResponse {
    info!("Handling RoomChatReportRequest: {:?}", req);

    let target_owner_index = req.target_owner_index.unwrap_or_default();
    let now = chrono::Utc::now().timestamp_millis();

    room_chat_report::log_report(
        pool,
        uid,
        target_owner_index,
        req.report_id,
        req.text.as_deref(),
        req.reason.as_deref(),
        req.chat_time,
        now,
    )
    .await;

    if let Some(target_uid) = room_chat_block::resolve_uid(pool, target_owner_index).await {
        let new_count = room_chat_report::record_against_target(pool, target_uid, now).await;
        if new_count >= room_chat_report::REPORT_THRESHOLD {
            room_chat_report::add_penalty(
                pool,
                target_uid,
                now,
                now + room_chat_report::PENALTY_DURATION_MS,
                req.report_id,
            )
            .await;
            room_chat_report::reset_report_count(pool, target_uid).await;
        }
    }

    let response = RoomChatReportResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::RoomChatReport.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
