pub mod account;
pub mod achievement;
pub mod age_gate;
pub mod active;
pub mod alchemy;
pub mod all;
pub mod attendance;
pub mod avatar;
pub mod balance;
pub mod batch;
pub mod battle;
pub mod cafeteria;
pub mod cancel;
pub mod cash;
pub mod char;
pub mod char_vote;
pub mod charge;
pub mod chat;
pub mod clear;
pub mod client;
pub mod community;
pub mod content;
pub mod cooking;
pub mod daily_story;
pub mod field_event_spawn;
pub mod fireworks;
pub mod costume;
pub mod dating;
pub mod deck;
pub mod dispatch;
pub mod eat;
pub mod equip;
pub mod equips;
pub mod event;
pub mod evil;
pub mod field;
pub mod friend;
pub mod friendship;
pub mod gacha;
pub mod guild;
pub mod hunt;
pub mod hunting;
pub mod id;
pub mod inn;
pub mod interaction;
pub mod inven;
pub mod item;
pub mod join;
pub mod jp;
pub mod leave;
pub mod colosseum;
pub mod fishing;
pub mod ib;
pub mod life;
pub mod like;
pub mod login;
pub mod logout;
pub mod mail;
pub mod maintenace;
pub mod maintenance;
pub mod master_title;
pub mod mercenary;
pub mod mini;
pub mod mission;
pub mod monster;
pub mod my;
pub mod notice;
pub mod npc;
pub mod overwhelm;
pub mod pack;
pub mod pass;
pub mod personal;
pub mod pictorial;
pub mod ping;
pub mod platform;
pub mod popular;
pub mod preset;
pub mod prestige;
pub mod price;
pub mod pvp;
pub mod quest;
pub mod quick;
pub mod recipe;
pub mod recommend;
pub mod refresh;
pub mod report;
pub mod reputation;
pub mod room_chat;
pub mod root;
pub mod save;
pub mod schedule;
pub mod season;
pub mod select;
pub mod send;
pub mod server;
pub mod shop;
pub mod sky;
pub mod spine_interaction;
pub mod square;
pub mod statue;
pub mod storage;
pub mod stress_cheat;
pub mod tactics_bingo;
pub mod supporter;
pub mod talent;
pub mod time;
pub mod today;
pub mod total;
pub mod trap;
pub mod tutorial;
pub mod update;
pub mod use2;
pub mod user;

/// Real display name for a uid, for use in any cross-account listing (rankings, matching,
/// etc.) that previously fell back to the stringified uid "since no profile-lookup helper was
/// in scope" — that helper now exists here. `UserInfo.UserId` is this project's real in-game
/// nickname column (set via the User profile-settings round; `UserId` "doubles as the display
/// nickname" per its own update helper's doc comment) — falls back to a `Guest{uid}` name,
/// matching the naming style PvP/Colosseum's bot-opponent synthesis already uses for accounts
/// that never set one.
pub async fn display_name(pool: &sqlx::SqlitePool, uid: i64) -> String {
    database::db::user::user_info::get_user_info(pool, uid)
        .await
        .ok()
        .and_then(|rows| rows.into_iter().next())
        .and_then(|row| row.user_id)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| format!("Guest{uid}"))
}
pub mod waypoint;
