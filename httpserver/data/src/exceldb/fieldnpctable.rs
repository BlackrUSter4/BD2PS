// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldnpctable {
    #[serde(rename = "animatorType")]
    pub animator_type: Option<i32>,
    #[serde(rename = "bundleType")]
    pub bundle_type: Option<i32>,
    #[serde(rename = "directionType")]
    pub direction_type: i32,
    #[serde(rename = "faceIllustName")]
    pub face_illust_name: Option<String>,
    #[serde(rename = "holdingCollectionId")]
    pub holding_collection_id: Option<i32>,
    #[serde(rename = "holdingCollectionTalkGroupIdList")]
    pub holding_collection_talk_group_id_list: Option<Vec<i32>>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "interactionList")]
    pub interaction_list: Vec<i32>,
    #[serde(rename = "interactionSpriteName")]
    pub interaction_sprite_name: Vec<String>,
    #[serde(rename = "interactionValue")]
    pub interaction_value: Vec<i32>,
    #[serde(rename = "isEnableTimeline")]
    pub is_enable_timeline: Option<i32>,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "npcDefaultStoryTextId")]
    pub npc_default_story_text_id: Option<i32>,
    #[serde(rename = "npcNameTextId")]
    pub npc_name_text_id: i32,
    #[serde(rename = "packId")]
    pub pack_id: Option<i32>,
    #[serde(rename = "questEnableType")]
    pub quest_enable_type: Vec<i32>,
    #[serde(rename = "questRange")]
    pub quest_range: Vec<i32>,
    #[serde(rename = "rangeType")]
    pub range_type: i32,
    #[serde(rename = "recoveryId")]
    pub recovery_id: Option<i32>,
    #[serde(rename = "reputationBadMessageLocalTextId")]
    pub reputation_bad_message_local_text_id: Option<i32>,
    #[serde(rename = "reputationGoodMessageLocalTextId")]
    pub reputation_good_message_local_text_id: Option<i32>,
    #[serde(rename = "resourceName")]
    pub resource_name: Option<String>,
    #[serde(rename = "resourceType")]
    pub resource_type: i32,
    #[serde(rename = "talentSkillFailTalkGroupId")]
    pub talent_skill_fail_talk_group_id: Option<Vec<i32>>,
    #[serde(rename = "talentSkillGroupList")]
    pub talent_skill_group_list: Option<Vec<i32>>,
    #[serde(rename = "talentSkillRewardGroupList")]
    pub talent_skill_reward_group_list: Option<Vec<i32>>,
    #[serde(rename = "talentSkillSuccessTalkGroupId")]
    pub talent_skill_success_talk_group_id: Option<Vec<i32>>,
    #[serde(rename = "talkGroupIdList")]
    pub talk_group_id_list: Option<Vec<i32>>,
    #[serde(rename = "voiceResourceName")]
    pub voice_resource_name: Option<String>,
}

pub struct FieldnpctableTable {
    records: Vec<Fieldnpctable>,
    by_id: HashMap<i32, usize>,
}

impl FieldnpctableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldnpctable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldnpctable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldnpctable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Fieldnpctable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
