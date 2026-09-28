// Auto-generated from JSON data
// Do not edit manually

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packtable {
    #[serde(rename = "bgmName")]
    pub bgm_name: String,
    #[serde(rename = "bgmSoundId")]
    pub bgm_sound_id: i32,
    #[serde(rename = "bgmTitleLocalTextId")]
    pub bgm_title_local_text_id: i32,
    #[serde(rename = "bundleLabel")]
    pub bundle_label: String,
    #[serde(rename = "buyDescLocalTextId")]
    pub buy_desc_local_text_id: i32,
    #[serde(rename = "buyRewardCount")]
    pub buy_reward_count: Option<Vec<i32>>,
    #[serde(rename = "buyRewardId")]
    pub buy_reward_id: Option<Vec<i32>>,
    #[serde(rename = "buyRewardType")]
    pub buy_reward_type: Option<Vec<i32>>,
    #[serde(rename = "caseBackTextureName")]
    pub case_back_texture_name: String,
    #[serde(rename = "caseBackThumbnailTextureName")]
    pub case_back_thumbnail_texture_name: String,
    #[serde(rename = "caseFrontPrefabName")]
    pub case_front_prefab_name: String,
    #[serde(rename = "contentGenreId")]
    pub content_genre_id: Option<Vec<i32>>,
    #[serde(rename = "contentTypeTextId")]
    pub content_type_text_id: Option<i32>,
    #[serde(rename = "dockingOnce")]
    pub docking_once: i32,
    #[serde(rename = "epilogLocalTextId")]
    pub epilog_local_text_id: i32,
    #[serde(rename = "fieldMapId")]
    pub field_map_id: Option<Vec<i32>>,
    #[serde(rename = "fieldMonsterGroupId")]
    pub field_monster_group_id: Option<Vec<i32>>,
    #[serde(rename = "fieldObjectGroupId")]
    pub field_object_group_id: Option<Vec<i32>>,
    #[serde(rename = "fieldTrapGroupId")]
    pub field_trap_group_id: Option<Vec<i32>>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isSkipPackInsertDirection")]
    pub is_skip_pack_insert_direction: Option<i32>,
    #[serde(rename = "mainQuestRewardCount")]
    pub main_quest_reward_count: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardCount1")]
    pub main_quest_reward_count1: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardCount2")]
    pub main_quest_reward_count2: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardCount3")]
    pub main_quest_reward_count3: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardCount4")]
    pub main_quest_reward_count4: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardId")]
    pub main_quest_reward_id: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardId1")]
    pub main_quest_reward_id1: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardId2")]
    pub main_quest_reward_id2: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardId3")]
    pub main_quest_reward_id3: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardId4")]
    pub main_quest_reward_id4: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardType")]
    pub main_quest_reward_type: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardType1")]
    pub main_quest_reward_type1: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardType2")]
    pub main_quest_reward_type2: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardType3")]
    pub main_quest_reward_type3: Option<Vec<i32>>,
    #[serde(rename = "mainQuestRewardType4")]
    pub main_quest_reward_type4: Option<Vec<i32>>,
    #[serde(rename = "nextPackId")]
    pub next_pack_id: Option<i32>,
    #[serde(rename = "packDescNameTextId")]
    pub pack_desc_name_text_id: i32,
    #[serde(rename = "packHide")]
    pub pack_hide: Option<i32>,
    #[serde(rename = "packLoadingPrefabName")]
    pub pack_loading_prefab_name: String,
    #[serde(rename = "packNameTextId")]
    pub pack_name_text_id: i32,
    #[serde(rename = "packPeriod")]
    pub pack_period: Option<i32>,
    #[serde(rename = "packPreviewName")]
    pub pack_preview_name: Vec<String>,
    #[serde(rename = "packSpriteName")]
    pub pack_sprite_name: String,
    #[serde(rename = "packType")]
    pub pack_type: Option<i32>,
    #[serde(rename = "packUnopenedResourceName")]
    pub pack_unopened_resource_name: String,
    #[serde(rename = "prologLocalTextId")]
    pub prolog_local_text_id: i32,
    #[serde(rename = "startPositionPath")]
    pub start_position_path: String,
    #[serde(rename = "storySynopsisQuestTextId")]
    pub story_synopsis_quest_text_id: Option<i32>,
    #[serde(rename = "useSchedule")]
    pub use_schedule: Option<i32>,
    #[serde(rename = "waypointPriceType")]
    pub waypoint_price_type: i32,
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

        Ok(Self { records, by_id })
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
    pub fn iter(&self) -> std::slice::Iter<Packtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
