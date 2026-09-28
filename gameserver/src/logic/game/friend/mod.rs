use bd2::proto::proto_net::FriendDbInfo;
use database::models::game::friend::friend_info::FriendInfo;

/// Build the response DTO from a stored relationship row. `user_id` falls back to the
/// other account's numeric uid stringified — same placeholder precedent as CharVote's/
/// Colosseum's/PvP's ranking (no display-name lookup exists anywhere in this project).
pub fn to_proto(row: &FriendInfo) -> FriendDbInfo {
    FriendDbInfo {
        owner_index: row.owner_index,
        portrait_costume_id: row.portrait_costume_id,
        portrait_costume_design_id: row.portrait_costume_design_id,
        user_id: row
            .user_id
            .clone()
            .or_else(|| row.owner_index.map(|o| o.to_string())),
        title_id: row.title_id,
        date: row.date,
        guild_base_info: None,
        last_login_date: row.last_login_date,
        supporter_info: vec![],
    }
}

pub mod friend_accept;
pub mod friend_info_list;
pub mod friend_receive_list;
pub mod friend_recommend;
pub mod friend_refuse;
pub mod friend_remove;
pub mod friend_search;
pub mod friend_send;
pub mod friend_send_list;
pub mod friend_send_remove;
