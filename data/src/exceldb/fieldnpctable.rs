// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldnpctable {
    #[serde(rename = "animatorType", default)]
    pub animator_type: Option<i32>,
    #[serde(rename = "bundleType", default)]
    pub bundle_type: Option<i32>,
    #[serde(rename = "directionType", default)]
    pub direction_type: i32,
    #[serde(rename = "faceIllustName", default)]
    pub face_illust_name: Option<String>,
    #[serde(rename = "holdingCollectionId", default)]
    pub holding_collection_id: Option<i32>,
    #[serde(rename = "holdingCollectionTalkGroupIdList", default)]
    pub holding_collection_talk_group_id_list: Option<Vec<i32>>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "interactionList", default)]
    pub interaction_list: Vec<i32>,
    #[serde(rename = "interactionSpriteName", default)]
    pub interaction_sprite_name: Vec<String>,
    #[serde(rename = "interactionValue", default)]
    pub interaction_value: Vec<i32>,
    #[serde(rename = "isEnableTimeline", default)]
    pub is_enable_timeline: Option<i32>,
    #[serde(rename = "mapId", default)]
    pub map_id: i32,
    #[serde(rename = "npcDefaultStoryTextId", default)]
    pub npc_default_story_text_id: Option<i32>,
    #[serde(rename = "npcNameTextId", default)]
    pub npc_name_text_id: i32,
    #[serde(rename = "packId", default)]
    pub pack_id: Option<i32>,
    #[serde(rename = "questEnableType", default)]
    pub quest_enable_type: Vec<i32>,
    #[serde(rename = "questRange", default)]
    pub quest_range: Vec<i32>,
    #[serde(rename = "rangeType", default)]
    pub range_type: i32,
    #[serde(rename = "recoveryId", default)]
    pub recovery_id: Option<i32>,
    #[serde(rename = "reputationBadMessageLocalTextId", default)]
    pub reputation_bad_message_local_text_id: Option<i32>,
    #[serde(rename = "reputationGoodMessageLocalTextId", default)]
    pub reputation_good_message_local_text_id: Option<i32>,
    #[serde(rename = "resourceName", default)]
    pub resource_name: Option<String>,
    #[serde(rename = "resourceType", default)]
    pub resource_type: i32,
    #[serde(rename = "talentSkillFailTalkGroupId", default)]
    pub talent_skill_fail_talk_group_id: Option<Vec<i32>>,
    #[serde(rename = "talentSkillGroupList", default)]
    pub talent_skill_group_list: Option<Vec<i32>>,
    #[serde(rename = "talentSkillRewardGroupList", default)]
    pub talent_skill_reward_group_list: Option<Vec<i32>>,
    #[serde(rename = "talentSkillSuccessTalkGroupId", default)]
    pub talent_skill_success_talk_group_id: Option<Vec<i32>>,
    #[serde(rename = "talkGroupIdList", default)]
    pub talk_group_id_list: Option<Vec<i32>>,
    #[serde(rename = "voiceResourceName", default)]
    pub voice_resource_name: Option<String>,
}

pub struct FieldnpctableTable {
    records: Vec<Fieldnpctable>,
    by_id: HashMap<i32, usize>,
    by_pack: HashMap<(i32, i32), usize>,
}

impl FieldnpctableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldnpctable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_pack = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_pack.insert((record.pack_id.unwrap_or(1), record.id), idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_pack,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Fieldnpctable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    /// Pack-scoped lookup — use this over `get()` for any new call site,
    /// since `id` collides across packs (see `pack_id` field doc).
    #[inline]
    pub fn get_by_pack(&self, pack_id: i32, id: i32) -> Option<&Fieldnpctable> {
        self.by_pack.get(&(pack_id, id)).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldnpctable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldnpctable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
