use bd2::prost::Message;
use bd2::proto::proto_net::{IdCardShopDbInfo, IdCardShopInfoRequest, IdCardShopInfoResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::id::id_card_shop_info as shop_db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account buy counts for every real shop item (IdCardItemTshopTable, 1002 real
/// rows) — unbought items report 0.
pub async fn handle(pool: &SqlitePool, uid: i64, req: IdCardShopInfoRequest) -> GameResponse {
    info!("Handling IdCardShopInfoRequest: {:?}", req);

    let owned = shop_db::get_id_card_shop_info(pool, uid).await.unwrap_or_default();
    let shop_buy_info = data::exceldb::get()
        .idcarditemtshoptable
        .all()
        .iter()
        .map(|item| {
            let buy_count = owned.iter().find(|o| o.id == Some(item.id)).and_then(|o| o.buy_count).unwrap_or(0);
            IdCardShopDbInfo { id: Some(item.id), buy_count: Some(buy_count) }
        })
        .collect();

    let response = IdCardShopInfoResponse { shop_buy_info };
    
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
    
    let (route, code) = PacketCodeType::IdCardShopInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}