use super::{
    citizen_row_to_dbinfo, group_into_place_infos, helper_gacha_pool_to_dbinfo,
    helper_row_to_dbinfo, life_user_to_dbinfo, now_ms,
};
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeInfoRequest, LifeInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use std::collections::BTreeSet;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeInfoRequest) -> GameResponse {
    info!("Handling LifeInfoRequest: {:?}", req);

    let user = database::db::life::life_user_info::get_or_create(pool, uid)
        .await
        .unwrap_or_default();
    let chunks = database::db::life::life_chunk_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();
    let objects = database::db::life::life_world_object_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();
    let citizens = database::db::life::life_citizen_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();
    let tools = database::db::life::life_tool_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();
    let now = now_ms();
    let _ = database::db::life::life_duration_buff_info::delete_expired(pool, uid, now).await;
    let buffs = database::db::life::life_duration_buff_info::get_active(pool, uid, now)
        .await
        .unwrap_or_default();
    let collection = database::db::life::life_collection_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();
    let helpers = database::db::life::life_helper_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();

    let helper_slots: BTreeSet<i32> = helpers.iter().filter_map(|h| h.helper_slot_id).collect();
    let mut helper_gacha_info = Vec::new();
    for slot in helper_slots {
        let pool_rows = database::db::life::life_helper_gacha_info::get_by_slot(pool, uid, slot)
            .await
            .unwrap_or_default();
        if !pool_rows.is_empty() {
            helper_gacha_info.push(helper_gacha_pool_to_dbinfo(slot, &pool_rows));
        }
    }

    let response = LifeInfoResponse {
        object_place_info: group_into_place_infos(objects),
        life_user_info: Some(life_user_to_dbinfo(
            &user,
            chunks.iter().map(|c| c.chunk_id).collect(),
        )),
        // No consumer for these yet in the item system (would need to distinguish "life"
        // context items/furnishings from regular inventory) — left empty rather than guessing.
        life_item_info: vec![],
        life_furnishings_info: vec![],
        life_tool_info: tools
            .iter()
            .map(|t| bd2::proto::proto_net::LifeToolDbInfo {
                group_id: Some(t.group_id),
                id: Some(t.tool_id),
            })
            .collect(),
        life_duration_buff_info: buffs
            .iter()
            .map(|b| bd2::proto::proto_net::LifeEatFoodDbInfo {
                item_id: b.item_id,
                end_time: b.end_time,
            })
            .collect(),
        life_citizen_list: citizens.iter().map(citizen_row_to_dbinfo).collect(),
        life_collection_id: collection.iter().map(|c| c.collection_id).collect(),
        helper_info: helpers.iter().map(helper_row_to_dbinfo).collect(),
        helper_gacha_info,
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

    let (route, code) = PacketCodeType::LifeInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
