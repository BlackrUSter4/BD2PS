use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, NpcQuizClearDbInfo, NpcQuizClearRequest, NpcQuizClearResponse, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, npc::npc_quiz_clear_info as db};
use sqlx::SqlitePool;
use tracing::info;

/// No NpcQuizTable answer-key/reward data exists anywhere — every submitted answer is
/// accepted (no way to validate correctness), and the reward is a flat placeholder, same
/// precedent as Ib/Friendship's equivalent "no question data" judgment calls.
const REWARD_ITEM_ID: i32 = 4;
const REWARD_ITEM_TYPE: i32 = 1;
const REWARD_COUNT: i32 = 20;

pub async fn handle(pool: &SqlitePool, uid: i64, req: NpcQuizClearRequest) -> GameResponse {
    info!("Handling NpcQuizClearRequest: {:?}", req);

    let event_uid = req.event_uid.unwrap_or_default();
    let group_id = req.group_id.unwrap_or_default();
    let id = req.id.unwrap_or_default();
    let newly_cleared = db::try_clear(pool, uid, event_uid, group_id, id).await;

    let reward_info_bundle = if newly_cleared {
        let _ = item_info::grant(pool, uid, REWARD_ITEM_ID, REWARD_ITEM_TYPE, REWARD_COUNT).await;
        RewardDbInfoBundle {
            item_info: vec![ItemDbInfo {
                id: Some(REWARD_ITEM_ID),
                r#type: Some(REWARD_ITEM_TYPE),
                count: Some(REWARD_COUNT),
                ..Default::default()
            }],
            ..Default::default()
        }
    } else {
        RewardDbInfoBundle::default()
    };

    let response = NpcQuizClearResponse {
        reward_info_bundle: Some(reward_info_bundle),
        clear_info: Some(NpcQuizClearDbInfo {
            event_uid: Some(event_uid),
            group_id: Some(group_id),
            id: Some(id),
        }),
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
    let (route, code) = PacketCodeType::NpcQuizClear.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
