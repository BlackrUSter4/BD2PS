use bd2::prost::Message;
use bd2::proto::proto_net::{SupporterBattleInfoRequest, SupporterBattleInfoResponse, SupporterDeckInfo, SupporterSlotInfo as ProtoSlot, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{friend::friend_info as friend_db, supporter::supporter_slot_info as db};
use sqlx::SqlitePool;
use tracing::info;

async fn build_deck(pool: &SqlitePool, owner: i64, is_friend: bool) -> Option<SupporterDeckInfo> {
    let slots = db::get_supporter_slot_info(pool, owner).await.ok()?;
    if slots.is_empty() {
        return None;
    }
    let supporter_slots: Vec<ProtoSlot> = slots
        .iter()
        .map(|r| ProtoSlot {
            owner_index: Some(owner),
            slot_index: r.slot_index,
            costume_id: r.costume_id,
            power: r.power,
            battle_use_count: r.battle_use_count,
            supporter_char_info: r.supporter_char_info.clone(),
            date: r.date,
        })
        .collect();
    Some(SupporterDeckInfo {
        owner_index: Some(owner),
        user_id: Some(crate::logic::game::display_name(pool, owner).await),
        title_id: None,
        portrait_costume_id: None,
        greeting: None,
        is_friend: Some(is_friend),
        guild_base_info: None,
        portrait_costume_design_id: None,
        usage_count: None,
        supporter_slots,
    })
}

/// Real friend/recommended supporter decks, built from each account's actually-registered
/// SupporterSlotInfo rows (empty-slot accounts are skipped rather than returned empty).
pub async fn handle(pool: &SqlitePool, uid: i64, req: SupporterBattleInfoRequest) -> GameResponse {
    info!("Handling SupporterBattleInfoRequest: {:?}", req);

    let friends = friend_db::get_by_status(pool, uid, 0).await.unwrap_or_default();
    let mut friend_supporter_info = Vec::new();
    for f in &friends {
        if let Some(owner) = f.owner_index {
            if let Some(deck) = build_deck(pool, owner, true).await {
                friend_supporter_info.push(deck);
            }
        }
    }

    let recommended = friend_db::list_recommendable(pool, uid, 10).await.unwrap_or_default();
    let mut recommended_supporter_info = Vec::new();
    for owner in recommended {
        if let Some(deck) = build_deck(pool, owner, false).await {
            recommended_supporter_info.push(deck);
        }
    }

    let response = SupporterBattleInfoResponse { friend_supporter_info, recommended_supporter_info };
    
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
    
    let (route, code) = PacketCodeType::SupporterBattleInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}