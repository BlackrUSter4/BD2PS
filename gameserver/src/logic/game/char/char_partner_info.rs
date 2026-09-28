use bd2::prost::Message;
use bd2::proto::proto_net::{CharPartnerDbInfo, CharPartnerInfoRequest, CharPartnerInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::char::char_partner_info::get_char_partner_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: CharPartnerInfoRequest) -> GameResponse {
    info!("Handling CharPartnerInfoRequest: {:?}", req);

    let rows = get_char_partner_info(pool, uid).await.unwrap_or_default();
    let char_partner_info = rows
        .into_iter()
        .map(|r| CharPartnerDbInfo {
            main_unique_id: r.main_unique_id,
            sub_unique_id: r.sub_unique_id,
            point: r.point,
            reward: r.reward,
        })
        .collect::<Vec<_>>();

    let response = CharPartnerInfoResponse { char_partner_info };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::CharPartnerInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
