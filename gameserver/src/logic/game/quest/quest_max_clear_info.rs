use bd2::prost::Message;
use bd2::proto::proto_net::{
    Notify, QuestMaxClearDbInfo, QuestMaxClearInfoRequest, QuestMaxClearInfoResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::quest::quest_max_clear_info::{
    get_quest_max_clear_info, upsert_quest_max_clear_info,
};
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: QuestMaxClearInfoRequest) -> GameResponse {
    info!("Handling QuestMaxClearInfoRequest: {:?}", req);

    let rows = get_quest_max_clear_info(pool, uid)
        .await
        .unwrap_or_default();

    let max_clear_info = if rows.is_empty() {
        info!(
            "No QuestMaxClearInfo for UID {}, initializing default...",
            uid
        );

        let is_new_user = true;

        if is_new_user {
            // insert a default starter record
            upsert_quest_max_clear_info(pool, uid, 1, 1).await.ok();
            vec![QuestMaxClearDbInfo {
                pack_id: Some(1),
                max_clear_id: Some(1),
            }]
        } else {
            vec![]
        }
    } else {
        rows.into_iter()
            .map(|r| QuestMaxClearDbInfo {
                pack_id: r.pack_id,
                max_clear_id: r.max_clear_id,
            })
            .collect()
    };

    let response = QuestMaxClearInfoResponse {
        max_clear_info,
        ..Default::default()
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8916),
        active_login_event: vec![1, 2],
        ..Default::default()
    };

    let (route, code) = PacketCodeType::QuestMaxClearInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
