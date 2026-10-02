// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chartable {
    #[serde(rename = "charGrowthId", default)]
    pub char_growth_id: i32,
    #[serde(rename = "charNameTextId", default)]
    pub char_name_text_id: i32,
    #[serde(rename = "criticalChanceValue", default)]
    pub critical_chance_value: Option<f32>,
    #[serde(rename = "criticalDamageRateValue", default)]
    pub critical_damage_rate_value: Option<f32>,
    #[serde(rename = "defaultCostumeId", default)]
    pub default_costume_id: i32,
    #[serde(rename = "element", default)]
    pub element: Option<i32>,
    #[serde(rename = "elementDefenseValue", default)]
    pub element_defense_value: Option<f32>,
    #[serde(rename = "elementPowerValue", default)]
    pub element_power_value: Option<f32>,
    #[serde(rename = "grade", default)]
    pub grade: i32,
    #[serde(rename = "growthgrade", default)]
    pub growthgrade: i32,
    #[serde(rename = "healthValue", default)]
    pub health_value: f32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "magicDefenseValue", default)]
    pub magic_defense_value: Option<f32>,
    #[serde(rename = "magicPowerValue", default)]
    pub magic_power_value: Option<f32>,
    #[serde(rename = "nextCharId", default)]
    pub next_char_id: Option<i32>,
    #[serde(rename = "physicalDefenseValue", default)]
    pub physical_defense_value: Option<f32>,
    #[serde(rename = "physicalPowerValue", default)]
    pub physical_power_value: Option<f32>,
    #[serde(rename = "talentId", default)]
    pub talent_id: Option<i32>,
    #[serde(rename = "type", default)]
    pub r#type: Option<i32>,
    #[serde(rename = "uniqueCharId", default)]
    pub unique_char_id: i32,
    #[serde(rename = "usePackTemporary", default)]
    pub use_pack_temporary: Option<i32>,
}

pub struct ChartableTable {
    records: Vec<Chartable>,
    by_id: HashMap<i32, usize>,
}

impl ChartableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Chartable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Chartable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Chartable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Chartable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
