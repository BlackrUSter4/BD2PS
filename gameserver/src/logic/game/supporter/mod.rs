pub mod supporter_battle_info;
pub mod supporter_borrow;
pub mod supporter_detail;
pub mod supporter_info;
pub mod supporter_register;
pub mod supporter_remove;
pub mod supporter_reward;
pub mod supporter_status;

use bd2::proto::proto_net::{CharDbInfo, GuildSupporterBattleCharInfo};
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
struct CharRef {
    char_inven_index: Option<i64>,
}

/// Real cross-account character+equip snapshot for a registered supporter slot.
/// `supporter_char_info` (the slot's stored JSON) records which of the *owning* account's
/// (`owner_uid`) characters was registered — this looks that character up on the owner's own
/// account (not the borrower's), exactly like the generic Preset system does for the caller's
/// own deck, just cross-account. `costume_info`/`awake_info` are left empty: no cheap real
/// derivation exists for either without a broader lookup this handler has no inputs for.
pub async fn build_battle_char_info(
    pool: &SqlitePool,
    owner_uid: i64,
    supporter_char_info_json: &Option<String>,
) -> Option<GuildSupporterBattleCharInfo> {
    let char_inven_index = supporter_char_info_json
        .as_deref()
        .and_then(|s| serde_json::from_str::<CharRef>(s).ok())
        .and_then(|r| r.char_inven_index)?;

    let char_row = database::db::char::char_info::get_by_inven_index(pool, owner_uid, char_inven_index)
        .await
        .ok()
        .flatten()?;

    let char_info = CharDbInfo {
        inven_index: char_row.inven_index,
        id: char_row.id,
        hp: char_row.hp,
        level: char_row.level,
        costume_id: char_row.costume_id,
        exp: char_row.exp,
        use_costume: char_row.use_costume,
        talent_level: char_row.talent_level,
        talent_exp: char_row.talent_exp,
        solidarity_reward: char_row.solidarity_reward,
        expiry_time: char_row.expiry_time,
        pictorialbook_info: vec![],
        connect_potential_costume: char_row.connect_potential_costume,
    };

    let equip_rows = database::db::equip::equip_info::get_by_use_char(pool, owner_uid, char_inven_index)
        .await
        .unwrap_or_default();
    let mut equip_info = Vec::with_capacity(equip_rows.len());
    for e in &equip_rows {
        if let Some(inven_index) = e.inven_index {
            if let Some((equip, base)) =
                super::equip::get_equip_with_base(pool, owner_uid, inven_index).await
            {
                equip_info.push(super::equip::to_dbinfo(&equip, base.as_ref()));
            }
        }
    }

    Some(GuildSupporterBattleCharInfo {
        char_info: Some(char_info),
        costume_info: vec![],
        equip_info,
        awake_info: None,
    })
}
