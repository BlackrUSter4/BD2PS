use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PvpDeckInfo {
    #[sqlx(rename = "Id")]
    pub id: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "DeckType")]
    pub deck_type: i32,
    #[sqlx(rename = "CharInvenIndex")]
    pub char_inven_index: i64,
    #[sqlx(rename = "Position")]
    pub position: i32,
    #[sqlx(rename = "Sequence")]
    pub sequence: Option<i32>,
    #[sqlx(rename = "CostumeInvenIndex")]
    pub costume_inven_index: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PvpDeckMeta {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "DeckType")]
    pub deck_type: i32,
    #[sqlx(rename = "ItemInfoJson")]
    pub item_info_json: String,
    #[sqlx(rename = "BattlePower")]
    pub battle_power: i32,
}
