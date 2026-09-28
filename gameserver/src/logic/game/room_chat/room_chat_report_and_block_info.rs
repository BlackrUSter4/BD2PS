use bd2::prost::Message;
use bd2::proto::proto_net::{
    RoomChatBlockDbInfo, RoomChatPenaltyDbInfo, RoomChatReportAndBlockInfoRequest, RoomChatReportAndBlockInfoResponse,
    RoomChatReportUserDbInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::room_chat::{room_chat_block, room_chat_report};
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: RoomChatReportAndBlockInfoRequest) -> GameResponse {
    info!("Handling RoomChatReportAndBlockInfoRequest: {:?}", req);

    let now = chrono::Utc::now().timestamp_millis();

    let chat_block_info = room_chat_block::get_all(pool, uid)
        .await
        .into_iter()
        .map(|b| RoomChatBlockDbInfo {
            target_owner_index: Some(b.target_owner_index),
            target_user_id: b.target_user_id,
            block_date: Some(b.block_date),
        })
        .collect();

    let report_user = room_chat_report::get_report_user(pool, uid).await;
    let report_user_info = Some(RoomChatReportUserDbInfo {
        report_count: Some(report_user.report_count),
        report_count_reset_time: report_user.report_count_reset_time,
    });

    let penalty_info = room_chat_report::get_active_penalty(pool, uid, now)
        .await
        .map(|p| RoomChatPenaltyDbInfo {
            start_time: Some(p.start_time),
            end_time: Some(p.end_time),
            report_id: p.report_id,
        });

    let response = RoomChatReportAndBlockInfoResponse {
        chat_block_info,
        report_user_info,
        penalty_info,
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::RoomChatReportAndBlockInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
