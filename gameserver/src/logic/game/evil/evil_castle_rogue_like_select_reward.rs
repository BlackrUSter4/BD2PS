use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeSelectRewardRequest, EvilCastleRogueLikeSelectRewardResponse, Notify, RelicDbInfo};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::{evil_castle_rogue_like_choice_info, evil_castle_rogue_like_state_info};
use database::db::relic::relic_info;
use database::models::game::relic::relic_info::RelicInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Consumes the current reward choice, granting whichever relic id the
/// client picked (type 1 = relic — the only choice kind this run's reward
/// flow generates; char/costume choices aren't produced anywhere yet so
/// those branches are left real-but-inert rather than fabricated).
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeSelectRewardRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeSelectRewardRequest: {:?}", req);

    let mut relic_info_out = None;
    if req.r#type == Some(1) {
        if let Some(id) = req.id {
            let new_index = chrono::Utc::now().timestamp_millis();
            let relic = RelicInfo { index: 0, uid, inven_index: Some(new_index), id: Some(id) };
            let _ = relic_info::insert(pool, &relic).await;
            relic_info_out = Some(RelicDbInfo { inven_index: Some(new_index), id: Some(id) });
        }
    }
    let _ = evil_castle_rogue_like_choice_info::delete_evil_castle_rogue_like_choice_info(pool, uid).await;

    let state = evil_castle_rogue_like_state_info::get_one(pool, uid).await.ok().flatten();
    let response = EvilCastleRogueLikeSelectRewardResponse {
        state_info: state.map(|s| super::roguelike::default_state(s.floor.unwrap_or(1), s.room.unwrap_or(0))),
        choice_info: None,
        char_info: vec![],
        costume_info: None,
        relic_info: relic_info_out,
        clear_floor: None,
        clear_room_info: None,
        rogue_like_gold: None,
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeSelectReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
