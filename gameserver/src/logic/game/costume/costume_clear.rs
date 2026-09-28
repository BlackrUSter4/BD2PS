use bd2::prost::Message;
use bd2::proto::proto_net::{CostumeClearRequest, CostumeClearResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{char::char_info, costume::costume_info as costume_db, pictorial::pictorial_book_info as book_db};
use database::models::game::pictorial::pictorial_book_info::PictorialBookInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Real pictorial-book unlock: resolves the owned costume+char inven indices to their real
/// master ids, looks up the matching CostumePictorialBookTable entry (chaUniqueId+costumeID),
/// and records it in PictorialBookInfo if not already unlocked.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CostumeClearRequest) -> GameResponse {
    info!("Handling CostumeClearRequest: {:?}", req);

    if let (Some(costume_index), Some(char_index)) = (req.costume_index, req.char_index) {
        let costume_id = costume_db::get_by_inven_index(pool, uid, costume_index)
            .await
            .ok()
            .flatten()
            .and_then(|c| c.id);
        let char_id = char_info::get_by_inven_index(pool, uid, char_index)
            .await
            .ok()
            .flatten()
            .and_then(|c| c.id);

        if let (Some(costume_id), Some(char_id)) = (costume_id, char_id) {
            let game_data = data::exceldb::get();
            if let Some(entry) = game_data
                .costumepictorialbooktable
                .iter()
                .find(|r| r.cha_unique_id == char_id && r.costume_i_d == costume_id)
            {
                let existing = book_db::get_pictorial_book_info(pool, uid).await.unwrap_or_default();
                if !existing.iter().any(|r| r.id == Some(entry.id)) {
                    let _ = book_db::add_pictorial_book_info(
                        pool,
                        &PictorialBookInfo { index: 0, uid, id: Some(entry.id), group_id: Some(entry.tab_type) },
                    )
                    .await;
                }
            }
        }
    }

    let response = CostumeClearResponse {};

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

    let (route, code) = PacketCodeType::CostumeClear.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
