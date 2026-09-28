use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MonsterHuntTeamProtoInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "BlueCharProto")]
    pub blue_char_proto: Option<String>,
    #[sqlx(rename = "BlueEquipProto")]
    pub blue_equip_proto: Option<String>,
    #[sqlx(rename = "BlueBuffProto")]
    pub blue_buff_proto: Option<String>,
    #[sqlx(rename = "BlueCostumeProto")]
    pub blue_costume_proto: Option<String>,
}