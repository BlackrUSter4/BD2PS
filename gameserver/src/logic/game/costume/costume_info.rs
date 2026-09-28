use bd2::prost::Message;
use bd2::proto::proto_net::{CostumeDbInfo, CostumeInfoRequest, CostumeInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::costume::costume_info::get_costume_info;
use serde_json;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: CostumeInfoRequest) -> GameResponse {
    info!("Handling CostumeInfoRequest: {:?}", req);

    let costume_rows = match get_costume_info(pool, uid).await {
        Ok(rows) => rows,
        Err(err) => {
            eprintln!("get_costume_info error: {:?}", err);
            vec![]
        }
    };

    let costume_info = costume_rows
        .into_iter()
        .filter_map(|row| {
            // Ensure the ID is valid
            let id = row.id?;
            if id == 0 {
                eprintln!(
                    "Skipping costume with id=0 (invenIndex={:?}, uid={})",
                    row.inven_index, row.uid
                );
                return None;
            }

            //aParse PotentialId JSON -> Vec<i32>
            let parsed_potential_ids: Vec<i32> = row
                .potential_id
                .as_deref()
                .and_then(|s| serde_json::from_str::<Vec<i32>>(s).ok())
                .unwrap_or_default();

            Some(CostumeDbInfo {
                inven_index: row.inven_index,
                id: Some(id),
                level: row.level,
                use_char: row.use_char,
                sort_id: row.sort_id,
                use_my_room_count: row.use_my_room_count,
                potential_id: parsed_potential_ids,
                pictorialbook_info: vec![],
                design_id: row.design_id,
            })
        })
        .collect::<Vec<_>>();

    let response = CostumeInfoResponse {
        costume_info,
        ..Default::default()
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::CostumeInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
