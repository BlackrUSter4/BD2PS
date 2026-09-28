use bd2::prost::Message;
use bd2::proto::proto_net::{
    CharDbInfo, CostumePotentialConnectRequest, CostumePotentialConnectResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{char::char_info, costume::costume_potential_connect_info as db};
use database::models::game::costume::costume_potential_connect_info::CostumePotentialConnectInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Real potential-connect: `inven_index` addresses the character being granted the connected
/// costume's potential (CharInfo.ConnectPotentialCostume), logged per-account in
/// CostumePotentialConnectInfo. Returns the real resulting CharDbInfo rows.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CostumePotentialConnectRequest) -> GameResponse {
    info!("Handling CostumePotentialConnectRequest: {:?}", req);

    let mut char_info_resp = Vec::new();
    for entry in &req.costume_potential_connect_info {
        let (Some(inven_index), Some(costume_id)) = (entry.inven_index, entry.costume_id) else {
            continue;
        };

        let _ = char_info::set_connect_potential_costume(pool, uid, inven_index, costume_id).await;
        let _ = db::add_costume_potential_connect_info(
            pool,
            &CostumePotentialConnectInfo {
                index: 0,
                uid,
                inven_index: Some(inven_index),
                costume_id: Some(costume_id),
            },
        )
        .await;

        if let Ok(Some(row)) = char_info::get_by_inven_index(pool, uid, inven_index).await {
            char_info_resp.push(CharDbInfo {
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
                pictorialbook_info: vec![],
                connect_potential_costume: row.connect_potential_costume,
            });
        }
    }

    let response = CostumePotentialConnectResponse { char_info: char_info_resp };

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

    let (route, code) = PacketCodeType::CostumePotentialConnect.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
