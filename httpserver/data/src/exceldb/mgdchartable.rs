// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mgdchartable {
    #[serde(rename = "attackRange")]
    pub attack_range: f32,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: f32,
    #[serde(rename = "attackType")]
    pub attack_type: Option<i32>,
    #[serde(rename = "attackValue")]
    pub attack_value: i32,
    #[serde(rename = "charNameTextId")]
    pub char_name_text_id: i32,
    #[serde(rename = "charScale")]
    pub char_scale: f32,
    #[serde(rename = "element")]
    pub element: Option<i32>,
    #[serde(rename = "faceIconName")]
    pub face_icon_name: String,
    #[serde(rename = "grade")]
    pub grade: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "prefabName")]
    pub prefab_name: i32,
    #[serde(rename = "sellCost")]
    pub sell_cost: i32,
    #[serde(rename = "splash")]
    pub splash: Option<i32>,
    #[serde(rename = "splashRange")]
    pub splash_range: Option<f32>,
    #[serde(rename = "summonRatio")]
    pub summon_ratio: i32,
    #[serde(rename = "upAttackValue")]
    pub up_attack_value: i32,
}

pub struct MgdchartableTable {
    records: Vec<Mgdchartable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl MgdchartableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Mgdchartable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Mgdchartable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Mgdchartable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Mgdchartable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Mgdchartable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
