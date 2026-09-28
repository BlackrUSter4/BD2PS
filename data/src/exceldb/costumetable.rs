// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Costumetable {
    #[serde(rename = "attackMoveType")]
    pub attack_move_type: Option<i32>,
    #[serde(rename = "attackRange")]
    pub attack_range: Option<i32>,
    #[serde(rename = "attackRangeCount")]
    pub attack_range_count: i32,
    #[serde(rename = "attackType")]
    pub attack_type: Option<i32>,
    #[serde(rename = "buffImmuneGroupID")]
    pub buff_immune_group_i_d: Option<i32>,
    #[serde(rename = "connectedCostumeDesignId")]
    pub connected_costume_design_id: Vec<i32>,
    #[serde(rename = "costumeDescNameTextId")]
    pub costume_desc_name_text_id: i32,
    #[serde(rename = "costumeDialog")]
    pub costume_dialog: i32,
    #[serde(rename = "costumeNameTextId")]
    pub costume_name_text_id: i32,
    #[serde(rename = "growthGroupId")]
    pub growth_group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "japaneseCharacterVoiceActorNameTextId")]
    pub japanese_character_voice_actor_name_text_id: Option<i32>,
    #[serde(rename = "koreanCharacterVoiceActorNameTextId")]
    pub korean_character_voice_actor_name_text_id: Option<i32>,
    #[serde(rename = "maxLevel")]
    pub max_level: Option<i32>,
    #[serde(rename = "norSubAttackBuffId")]
    pub nor_sub_attack_buff_id: Option<i32>,
    #[serde(rename = "norSubAttackDescSkillTextId")]
    pub nor_sub_attack_desc_skill_text_id: Option<i32>,
    #[serde(rename = "norSubAttackNameSkillTextId")]
    pub nor_sub_attack_name_skill_text_id: i32,
    #[serde(rename = "normalAttackDescSkillTextId")]
    pub normal_attack_desc_skill_text_id: i32,
    #[serde(rename = "normalAttackNameSkillTextId")]
    pub normal_attack_name_skill_text_id: i32,
    #[serde(rename = "notTrash")]
    pub not_trash: i32,
    #[serde(rename = "packId")]
    pub pack_id: Option<i32>,
    #[serde(rename = "skillGroupId")]
    pub skill_group_id: Option<i32>,
    #[serde(rename = "spAttackAddCount")]
    pub sp_attack_add_count: i32,
    #[serde(rename = "targetType")]
    pub target_type: Option<i32>,
    #[serde(rename = "useRoguelike")]
    pub use_roguelike: Option<i32>,
    #[serde(rename = "useUniqueCharId")]
    pub use_unique_char_id: i32,
}

pub struct CostumetableTable {
    records: Vec<Costumetable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl CostumetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Costumetable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.growth_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Costumetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Costumetable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Costumetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Costumetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
