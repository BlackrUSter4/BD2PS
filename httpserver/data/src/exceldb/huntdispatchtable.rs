// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Huntdispatchtable {
    #[serde(rename = "apPerTime")]
    pub ap_per_time: i32,
    #[serde(rename = "battleCountBoss")]
    pub battle_count_boss: i32,
    #[serde(rename = "battleCountMonster")]
    pub battle_count_monster: i32,
    #[serde(rename = "clearTime")]
    pub clear_time: i32,
    #[serde(rename = "difficulty")]
    pub difficulty: Option<i32>,
    #[serde(rename = "dispatchNameTextId")]
    pub dispatch_name_text_id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mapUiObjectId")]
    pub map_ui_object_id: Option<i32>,
    #[serde(rename = "packId")]
    pub pack_id: i32,
    #[serde(rename = "packNameTextId")]
    pub pack_name_text_id: i32,
    #[serde(rename = "rewardGrowthRate")]
    pub reward_growth_rate: Option<i32>,
    #[serde(rename = "typeGroupId")]
    pub type_group_id: Option<i32>,
    #[serde(rename = "typeId")]
    pub type_id: i32,
    #[serde(rename = "visualItemId")]
    pub visual_item_id: Vec<i32>,
    #[serde(rename = "visualItemType")]
    pub visual_item_type: Vec<i32>,
}

pub struct HuntdispatchtableTable {
    records: Vec<Huntdispatchtable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl HuntdispatchtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Huntdispatchtable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Huntdispatchtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Huntdispatchtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Huntdispatchtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Huntdispatchtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
