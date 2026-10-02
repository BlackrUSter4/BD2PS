// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fishingdefaulttable {
    #[serde(rename = "actionPointCount", default)]
    pub action_point_count: i32,
    #[serde(rename = "aquariumBgmPath", default)]
    pub aquarium_bgm_path: String,
    #[serde(rename = "aquariumMapGroupId", default)]
    pub aquarium_map_group_id: i32,
    #[serde(rename = "aquariumTrackingSound", default)]
    pub aquarium_tracking_sound: String,
    #[serde(rename = "autoBiteTime", default)]
    pub auto_bite_time: i32,
    #[serde(rename = "autoFishingButtonOff", default)]
    pub auto_fishing_button_off: String,
    #[serde(rename = "autoFishingButtonOn", default)]
    pub auto_fishing_button_on: String,
    #[serde(rename = "autoFishingOpenCondition", default)]
    pub auto_fishing_open_condition: i32,
    #[serde(rename = "autoGradePoolId", default)]
    pub auto_grade_pool_id: i32,
    #[serde(rename = "autoStartCount", default)]
    pub auto_start_count: i32,
    #[serde(rename = "criticalMultiplier", default)]
    pub critical_multiplier: i32,
    #[serde(rename = "defaultItemCount", default)]
    pub default_item_count: Vec<i32>,
    #[serde(rename = "defaultItemId", default)]
    pub default_item_id: Vec<i32>,
    #[serde(rename = "defaultItemType", default)]
    pub default_item_type: Vec<i32>,
    #[serde(rename = "defaultMapId", default)]
    pub default_map_id: i32,
    #[serde(rename = "finishAlert", default)]
    pub finish_alert: i32,
    #[serde(rename = "fishTrapFishPoolId", default)]
    pub fish_trap_fish_pool_id: i32,
    #[serde(rename = "fishTrapGradePoolId", default)]
    pub fish_trap_grade_pool_id: i32,
    #[serde(rename = "fishTrapMaxTime", default)]
    pub fish_trap_max_time: i32,
    #[serde(rename = "fishTrapOpenCondition", default)]
    pub fish_trap_open_condition: i32,
    #[serde(rename = "fishTrapOpenContentTicket", default)]
    pub fish_trap_open_content_ticket: i32,
    #[serde(rename = "fishingBaseTime", default)]
    pub fishing_base_time: i32,
    #[serde(rename = "gaugeCharge", default)]
    pub gauge_charge: String,
    #[serde(rename = "guideEventMissionGroupId", default)]
    pub guide_event_mission_group_id: i32,
    #[serde(rename = "loadingLimit", default)]
    pub loading_limit: i32,
    #[serde(rename = "lobbyAmbienceName", default)]
    pub lobby_ambience_name: String,
    #[serde(rename = "lobbySceneBgm", default)]
    pub lobby_scene_bgm: String,
    #[serde(rename = "lobbySceneName", default)]
    pub lobby_scene_name: String,
    #[serde(rename = "maxBaitTime", default)]
    pub max_bait_time: i32,
    #[serde(rename = "maxBigLuck", default)]
    pub max_big_luck: i32,
    #[serde(rename = "maxCharLevel", default)]
    pub max_char_level: i32,
    #[serde(rename = "maxRareLuck", default)]
    pub max_rare_luck: i32,
    #[serde(rename = "minBaitTime", default)]
    pub min_bait_time: i32,
    #[serde(rename = "multiOpenCondition", default)]
    pub multi_open_condition: i32,
    #[serde(rename = "multiSearchTime", default)]
    pub multi_search_time: i32,
    #[serde(rename = "openMapUiObjectId", default)]
    pub open_map_ui_object_id: i32,
    #[serde(rename = "questNameTextId", default)]
    pub quest_name_text_id: i32,
    #[serde(rename = "questSkipTextId", default)]
    pub quest_skip_text_id: i32,
    #[serde(rename = "questTitleQuestTextId", default)]
    pub quest_title_quest_text_id: i32,
    #[serde(rename = "reconnectTimeout", default)]
    pub reconnect_timeout: i32,
    #[serde(rename = "roomDuration", default)]
    pub room_duration: i32,
    #[serde(rename = "roomListCount", default)]
    pub room_list_count: i32,
    #[serde(rename = "roomRerollInterval", default)]
    pub room_reroll_interval: i32,
    #[serde(rename = "skillHitzoneSize", default)]
    pub skill_hitzone_size: i32,
    #[serde(rename = "skinOpenCondition", default)]
    pub skin_open_condition: i32,
    #[serde(rename = "staminaDevider", default)]
    pub stamina_devider: i32,
    #[serde(rename = "startTimelineName", default)]
    pub start_timeline_name: String,
    #[serde(rename = "startVisualNovelDialogId", default)]
    pub start_visual_novel_dialog_id: i32,
    #[serde(rename = "weekEventMissionGroupId", default)]
    pub week_event_mission_group_id: i32,
}

pub struct FishingdefaulttableTable {
    records: Vec<Fishingdefaulttable>,
}

impl FishingdefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fishingdefaulttable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Fishingdefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fishingdefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
