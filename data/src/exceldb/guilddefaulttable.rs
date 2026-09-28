// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guilddefaulttable {
    #[serde(rename = "chatDeleteCount")]
    pub chat_delete_count: i32,
    #[serde(rename = "chatDeleteTime")]
    pub chat_delete_time: i32,
    #[serde(rename = "guildAttendanceRewardCount")]
    pub guild_attendance_reward_count: i32,
    #[serde(rename = "guildAttendanceRewardId")]
    pub guild_attendance_reward_id: i32,
    #[serde(rename = "guildAttendanceRewardType")]
    pub guild_attendance_reward_type: i32,
    #[serde(rename = "guildCreateItemCount")]
    pub guild_create_item_count: i32,
    #[serde(rename = "guildCreateItemType")]
    pub guild_create_item_type: i32,
    #[serde(rename = "guildDeleteCancelTime")]
    pub guild_delete_cancel_time: i32,
    #[serde(rename = "guildGuideId")]
    pub guild_guide_id: i32,
    #[serde(rename = "guildListCount")]
    pub guild_list_count: i32,
    #[serde(rename = "guildMaxMemberCount")]
    pub guild_max_member_count: i32,
    #[serde(rename = "guildSignDelayTime")]
    pub guild_sign_delay_time: i32,
    #[serde(rename = "guildSignListCount")]
    pub guild_sign_list_count: i32,
    #[serde(rename = "guildSignRequestExpireTime")]
    pub guild_sign_request_expire_time: i32,
    #[serde(rename = "preGuildGuideId")]
    pub pre_guild_guide_id: i32,
}

pub struct GuilddefaulttableTable {
    records: Vec<Guilddefaulttable>,
}

impl GuilddefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Guilddefaulttable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Guilddefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Guilddefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
