use bd2::prost::Message;
use bd2::proto::proto_net::{
    Notify, PictorialBookDbInfo, PictorialBookInfoRequest, PictorialBookInfoResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pictorial::pictorial_book_info::get_pictorial_book_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: PictorialBookInfoRequest) -> GameResponse {
    info!("Handling PictorialBookInfoRequest: {:?}", req);

    let pictorials = match get_pictorial_book_info(pool, uid).await {
        Ok(list) => list,
        Err(err) => {
            eprintln!("get_pictorial_book_info failed: {:?}", err);
            vec![]
        }
    };

    let pictorialbook_info = pictorials
        .into_iter()
        .map(|p| PictorialBookDbInfo {
            id: p.id,
            group_id: p.group_id,
        })
        .collect::<Vec<_>>();

    let response = PictorialBookInfoResponse {
        pictorialbook_info,
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

    let (route, code) = PacketCodeType::PictorialBookInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
