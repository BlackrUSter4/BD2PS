use bd2::prost::Message;
use bd2::proto::proto_net::{FieldDeckDbInfo, FieldDeckInfoRequest, FieldDeckInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::field::field_deck_info::get_field_deck_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, _req: FieldDeckInfoRequest) -> GameResponse {
    info!("Handling FieldDeckInfoRequest for uid: {}", uid);

    let field_deck_infos = match get_field_deck_info(pool, uid).await {
        Ok(list) => list,
        Err(err) => {
            eprintln!("Error fetching FieldDeckInfo: {:?}", err);
            vec![]
        }
    };

    let field_deck_info_proto: Vec<FieldDeckDbInfo> = field_deck_infos
        .into_iter()
        .map(|f| FieldDeckDbInfo {
            sequence: f.sequence,
            char_inven_index: f.char_inven_index,
            costume_inven_index: f.costume_inven_index,
        })
        .collect();

    let response = FieldDeckInfoResponse {
        field_deck_info: field_deck_info_proto,
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8916),
        active_login_event: vec![1, 2],
        ..Default::default()
    };

    let (route, code) = PacketCodeType::FieldDeckInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
