pub mod tactics_bingo_deck_save;
pub mod tactics_bingo_info;

use bd2::proto::proto_net::Notify;

pub fn default_notify() -> Notify {
    Notify {
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
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DeckEntry {
    pub char_inven_index: Option<i64>,
    pub char_id: Option<i32>,
    pub costume_id: Option<i32>,
    pub position: Option<i32>,
    pub sequence: Option<i32>,
}
