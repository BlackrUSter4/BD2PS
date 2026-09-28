// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fishingdefaulttable {
    #[serde(rename = "actionPointCount")]
    pub action_point_count: i32,
    #[serde(rename = "aquariumBgmPath")]
    pub aquarium_bgm_path: String,
    #[serde(rename = "aquariumMapGroupId")]
    pub aquarium_map_group_id: i32,
    #[serde(rename = "aquariumTrackingSound")]
    pub aquarium_tracking_sound: String,
    #[serde(rename = "autoBiteTime")]
    pub auto_bite_time: i32,
    #[serde(rename = "autoFishingButtonOff")]
    pub auto_fishing_button_off: String,
    #[serde(rename = "autoFishingButtonOn")]
    pub auto_fishing_button_on: String,
    #[serde(rename = "autoFishingOpenCondition")]
    pub auto_fishing_open_condition: i32,
    #[serde(rename = "autoGradePoolId")]
    pub auto_grade_pool_id: i32,
    #[serde(rename = "autoStartCount")]
    pub auto_start_count: i32,
    #[serde(rename = "criticalMultiplier")]
    pub critical_multiplier: i32,
    #[serde(rename = "defaultItemCount")]
    pub default_item_count: Vec<i32>,
    #[serde(rename = "defaultItemId")]
    pub default_item_id: Vec<i32>,
    #[serde(rename = "defaultItemType")]
    pub default_item_type: Vec<i32>,
    #[serde(rename = "defaultMapId")]
    pub default_map_id: i32,
    #[serde(rename = "finishAlert")]
    pub finish_alert: i32,
    #[serde(rename = "fishTrapFishPoolId")]
    pub fish_trap_fish_pool_id: i32,
    #[serde(rename = "fishTrapGradePoolId")]
    pub fish_trap_grade_pool_id: i32,
    #[serde(rename = "fishTrapMaxTime")]
    pub fish_trap_max_time: i32,
    #[serde(rename = "fishTrapOpenCondition")]
    pub fish_trap_open_condition: i32,
    #[serde(rename = "fishTrapOpenContentTicket")]
    pub fish_trap_open_content_ticket: i32,
    #[serde(rename = "fishingBaseTime")]
    pub fishing_base_time: i32,
    #[serde(rename = "gaugeCharge")]
    pub gauge_charge: String,
    #[serde(rename = "guideEventMissionGroupId")]
    pub guide_event_mission_group_id: i32,
    #[serde(rename = "loadingLimit")]
    pub loading_limit: i32,
    #[serde(rename = "lobbyAmbienceName")]
    pub lobby_ambience_name: String,
    #[serde(rename = "lobbySceneBgm")]
    pub lobby_scene_bgm: String,
    #[serde(rename = "lobbySceneName")]
    pub lobby_scene_name: String,
    #[serde(rename = "maxBaitTime")]
    pub max_bait_time: i32,
    #[serde(rename = "maxBigLuck")]
    pub max_big_luck: i32,
    #[serde(rename = "maxCharLevel")]
    pub max_char_level: i32,
    #[serde(rename = "maxRareLuck")]
    pub max_rare_luck: i32,
    #[serde(rename = "minBaitTime")]
    pub min_bait_time: i32,
    #[serde(rename = "multiOpenCondition")]
    pub multi_open_condition: i32,
    #[serde(rename = "multiSearchTime")]
    pub multi_search_time: i32,
    #[serde(rename = "openMapUiObjectId")]
    pub open_map_ui_object_id: i32,
    #[serde(rename = "questNameTextId")]
    pub quest_name_text_id: i32,
    #[serde(rename = "questSkipTextId")]
    pub quest_skip_text_id: i32,
    #[serde(rename = "questTitleQuestTextId")]
    pub quest_title_quest_text_id: i32,
    #[serde(rename = "reconnectTimeout")]
    pub reconnect_timeout: i32,
    #[serde(rename = "roomDuration")]
    pub room_duration: i32,
    #[serde(rename = "roomListCount")]
    pub room_list_count: i32,
    #[serde(rename = "roomRerollInterval")]
    pub room_reroll_interval: i32,
    #[serde(rename = "skillHitzoneSize")]
    pub skill_hitzone_size: i32,
    #[serde(rename = "skinOpenCondition")]
    pub skin_open_condition: i32,
    #[serde(rename = "staminaDevider")]
    pub stamina_devider: i32,
    #[serde(rename = "startTimelineName")]
    pub start_timeline_name: String,
    #[serde(rename = "startVisualNovelDialogId")]
    pub start_visual_novel_dialog_id: i32,
    #[serde(rename = "weekEventMissionGroupId")]
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
