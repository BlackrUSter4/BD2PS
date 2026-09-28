use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct LifeHelperGachaInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "HelperSlotId")]
    pub helper_slot_id: Option<i32>,
    #[sqlx(rename = "HelperId")]
    pub helper_id: Option<i32>,
    #[sqlx(rename = "HelperName")]
    pub helper_name: Option<String>,
    #[sqlx(rename = "UseCharId")]
    pub use_char_id: Option<i32>,
    #[sqlx(rename = "UseHairId")]
    pub use_hair_id: Option<i32>,
    #[sqlx(rename = "UseHairAccessoryId")]
    pub use_hair_accessory_id: Option<i32>,
    #[sqlx(rename = "UseFaceAccessoryId")]
    pub use_face_accessory_id: Option<i32>,
    #[sqlx(rename = "UseCostumeId")]
    pub use_costume_id: Option<i32>,
    #[sqlx(rename = "UseBodyAccessoryId")]
    pub use_body_accessory_id: Option<i32>,
    #[sqlx(rename = "UseHandAccessoryId")]
    pub use_hand_accessory_id: Option<i32>,
    #[sqlx(rename = "UsePetId")]
    pub use_pet_id: Option<i32>,
    #[sqlx(rename = "UseMountId")]
    pub use_mount_id: Option<i32>,
    #[sqlx(rename = "UseEffectId")]
    pub use_effect_id: Option<i32>,
    #[sqlx(rename = "AvatarDate")]
    pub avatar_date: Option<i64>,
}
