use bd2::prost::Message;
use bd2::proto::proto_net::{CharClassUpRequest, CharClassUpResponse, CharDbInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::char::char_info;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

/// No table anywhere maps a class stage to a max-level requirement (only one
/// sparse `CharGrowthTable` row exists, id 101), so class-tier gating is a
/// documented placeholder — the class stage is real and persisted (new
/// `ClassLevel` column), consumed items are real, only the *requirement to
/// class up* isn't validated against real data.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CharClassUpRequest) -> GameResponse {
    info!("Handling CharClassUpRequest: {:?}", req);

    let Some(inven_index) = req.inven_index else {
        return GameResponse::error(1);
    };

    let Ok(Some(row)) = char_info::get_by_inven_index(pool, uid, inven_index).await else {
        return GameResponse::error(1);
    };

    let new_class_level = row.class_level + 1;
    let _ = char_info::set_class_level(pool, uid, inven_index, new_class_level).await;

    for item in &req.item_material_info {
        if let Some(idx) = item.inven_index {
            let _ = item_info::delete_by_inven_index(pool, uid, idx).await;
        }
    }

    let response = CharClassUpResponse {
        char_info: Some(CharDbInfo {
            inven_index: row.inven_index,
            id: row.id,
            hp: row.hp,
            level: row.level,
            costume_id: row.costume_id,
            exp: row.exp,
            use_costume: row.use_costume,
            talent_level: row.talent_level,
            talent_exp: row.talent_exp,
            solidarity_reward: row.solidarity_reward,
            expiry_time: row.expiry_time,
            connect_potential_costume: row.connect_potential_costume,
            pictorialbook_info: vec![],
        }),
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
    let (route, code) = PacketCodeType::CharClassUp.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
