use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRaidBossQuickBattleRequest, GuildRaidBossQuickBattleResponse, ItemDbInfo, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Auto-resolves today's raid attempt without a client-run battle. Consumes
/// the daily attempt like a normal battle would; reward contents are a
/// documented placeholder (no boss reward table was captured).
pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidBossQuickBattleRequest) -> GameResponse {
    info!("Handling GuildRaidBossQuickBattleRequest: {:?}", req);

    let _ = sqlx::query(
        "UPDATE GuildRaidMainInfo SET TodayNormalBattleCount = COALESCE(TodayNormalBattleCount, 0) + 1 WHERE Uid = ?",
    )
    .bind(uid)
    .execute(pool)
    .await;

    let response = GuildRaidBossQuickBattleResponse {
        reward_info_bundle: Some(RewardDbInfoBundle {
            item_info: vec![ItemDbInfo { id: Some(4), count: Some(100), ..Default::default() }],
            ..Default::default()
        }),
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
    let (route, code) = PacketCodeType::GuildRaidBossQuickBattle.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
