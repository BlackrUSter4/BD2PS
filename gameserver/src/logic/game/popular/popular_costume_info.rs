use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, PopularCostumeCountDbInfo, PopularCostumeInfoRequest, PopularCostumeInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::{Row, SqlitePool};
use tracing::info;

/// Real cross-account popularity: how many real accounts currently have each costume id
/// equipped on a character (`CostumeInfo.UseChar IS NOT NULL`). `count_type_1`/`count_type_2`
/// have no distinguishing data source anywhere in this project (no per-mode usage tracking
/// exists) so only `count_type_0` (overall use) is real; the other two stay honestly empty.
pub async fn handle(pool: &SqlitePool, _uid: i64, req: PopularCostumeInfoRequest) -> GameResponse {
    info!("Handling PopularCostumeInfoRequest: {:?}", req);

    let rows = sqlx::query(
        "SELECT Id, COUNT(*) as cnt FROM CostumeInfo WHERE UseChar IS NOT NULL AND Id IS NOT NULL GROUP BY Id ORDER BY cnt DESC LIMIT 50",
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    let info = rows
        .into_iter()
        .map(|row| PopularCostumeCountDbInfo {
            id: row.get::<Option<i32>, _>("Id"),
            count_type_0: Some(row.get::<i64, _>("cnt")),
            count_type_1: None,
            count_type_2: None,
        })
        .collect();

    let response = PopularCostumeInfoResponse { info };

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

    let (route, code) = PacketCodeType::PopularCostumeInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
