// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packtable {
    #[serde(rename = "bgmName", default)]
    pub bgm_name: String,
    #[serde(rename = "bgmSoundId", default)]
    pub bgm_sound_id: i32,
    #[serde(rename = "bgmTitleLocalTextId", default)]
    pub bgm_title_local_text_id: i32,
    #[serde(rename = "bundleLabel", default)]
    pub bundle_label: String,
    #[serde(rename = "buyDescLocalTextId", default)]
    pub buy_desc_local_text_id: i32,
    #[serde(rename = "buyRewardCount", default)]
    pub buy_reward_count: Option<Vec<i32>>,
    #[serde(rename = "buyRewardId", default)]
    pub buy_reward_id: Option<Vec<i32>>,
    #[serde(rename = "buyRewardType", default)]
    pub buy_reward_type: Option<Vec<i32>>,
    #[serde(rename = "caseBackTextureName", default)]
    pub case_back_texture_name: String,
    #[serde(rename = "caseBackThumbnailTextureName", default)]
    pub case_back_thumbnail_texture_name: String,
    #[serde(rename = "caseFrontPrefabName", default)]
    pub case_front_prefab_name: String,
    #[serde(rename = "contentGenreId", default)]
    pub content_genre_id: Option<Vec<i32>>,
    #[serde(rename = "contentTypeTextId", default)]
    pub content_type_text_id: Option<i32>,
    #[serde(rename = "dockingOnce", default)]
    pub docking_once: i32,
    #[serde(rename = "epilogLocalTextId", default)]
    pub epilog_local_text_id: i32,
    #[serde(rename = "fieldMapId", default)]
    pub field_map_id: Option<Vec<i32>>,
    #[serde(rename = "fieldMonsterGroupId", default)]
    pub field_monster_group_id: Option<Vec<i32>>,
    #[serde(rename = "fieldObjectGroupId", default)]
    pub field_object_group_id: Option<Vec<i32>>,
    #[serde(rename = "fieldTrapGroupId", default)]
    pub field_trap_group_id: Option<Vec<i32>>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "isSkipPackInsertDirection", default)]
    pub is_skip_pack_insert_direction: Option<i32>,
    #[serde(rename = "mainQuestRewardCount", default)]
    pub main_quest_reward_count: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardCount1", default)]
    pub main_quest_reward_count1: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardCount2", default)]
    pub main_quest_reward_count2: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardCount3", default)]
    pub main_quest_reward_count3: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardCount4", default)]
    pub main_quest_reward_count4: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardId", default)]
    pub main_quest_reward_id: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardId1", default)]
    pub main_quest_reward_id1: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardId2", default)]
    pub main_quest_reward_id2: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardId3", default)]
    pub main_quest_reward_id3: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardId4", default)]
    pub main_quest_reward_id4: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardType", default)]
    pub main_quest_reward_type: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardType1", default)]
    pub main_quest_reward_type1: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardType2", default)]
    pub main_quest_reward_type2: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardType3", default)]
    pub main_quest_reward_type3: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardType4", default)]
    pub main_quest_reward_type4: Option<Vec<i32>>,
    #[serde(rename = "nextPackId", default)]
    pub next_pack_id: Option<i32>,
    #[serde(rename = "packDescNameTextId", default)]
    pub pack_desc_name_text_id: i32,
    #[serde(rename = "packHide", default)]
    pub pack_hide: Option<i32>,
    #[serde(rename = "packLoadingPrefabName", default)]
    pub pack_loading_prefab_name: String,
    #[serde(rename = "packNameTextId", default)]
    pub pack_name_text_id: i32,
    #[serde(rename = "packPeriod", default)]
    pub pack_period: Option<i32>,
    #[serde(rename = "packPreviewName", default)]
    pub pack_preview_name: Vec<String>,
    #[serde(rename = "packSpriteName", default)]
    pub pack_sprite_name: String,
    #[serde(rename = "packType", default)]
    pub pack_type: Option<i32>,
    #[serde(rename = "packUnopenedResourceName", default)]
    pub pack_unopened_resource_name: String,
    #[serde(rename = "prologLocalTextId", default)]
    pub prolog_local_text_id: i32,
    #[serde(rename = "startPositionPath", default)]
    pub start_position_path: String,
    #[serde(rename = "storySynopsisQuestTextId", default)]
    pub story_synopsis_quest_text_id: Option<i32>,
    #[serde(rename = "useSchedule", default)]
    pub use_schedule: Option<i32>,
    #[serde(rename = "waypointPriceType", default)]
    pub waypoint_price_type: i32,
    #[serde(rename = "packDisplayNumber", default)]
    pub pack_display_number: Option<i32>,
    #[serde(rename = "shopMapId", default)]
    pub shop_map_id: Option<i32>,
}

pub struct PacktableTable {
    records: Vec<Packtable>,
    by_id: HashMap<i32, usize>,
}

impl PacktableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Packtable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
        }
        
        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Packtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Packtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Packtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
