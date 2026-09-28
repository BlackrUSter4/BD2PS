// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Costumedesignconceptinfotable {
    #[serde(rename = "ageProfileTextId")]
    pub age_profile_text_id: i32,
    #[serde(rename = "associationProfileTextId")]
    pub association_profile_text_id: i32,
    #[serde(rename = "birthDayProfileTextId")]
    pub birth_day_profile_text_id: i32,
    #[serde(rename = "cutSceneDialogProfileTextId")]
    pub cut_scene_dialog_profile_text_id: Option<Vec<i32>>,
    #[serde(rename = "cutSceneDialogVoiceResourceName")]
    pub cut_scene_dialog_voice_resource_name: Option<Vec<String>>,
    #[serde(rename = "dislikeProfileTextId")]
    pub dislike_profile_text_id: i32,
    #[serde(rename = "favoriteProfileTextId")]
    pub favorite_profile_text_id: i32,
    #[serde(rename = "generalDialogProfileTextId")]
    pub general_dialog_profile_text_id: Vec<i32>,
    #[serde(rename = "generalDialogVoiceResourceName")]
    pub general_dialog_voice_resource_name: Vec<String>,
    #[serde(rename = "heightProfileTextId")]
    pub height_profile_text_id: i32,
    #[serde(rename = "hobbyProfileTextId")]
    pub hobby_profile_text_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "preciousProfileTextId")]
    pub precious_profile_text_id: i32,
    #[serde(rename = "rumor1ProfileTextId")]
    pub rumor1_profile_text_id: i32,
    #[serde(rename = "rumor2ProfileTextId")]
    pub rumor2_profile_text_id: i32,
    #[serde(rename = "skillDialogProfileTextId")]
    pub skill_dialog_profile_text_id: Option<Vec<i32>>,
    #[serde(rename = "skillDialogVoiceResourceName")]
    pub skill_dialog_voice_resource_name: Option<Vec<String>>,
    #[serde(rename = "summaryProfileTextId")]
    pub summary_profile_text_id: i32,
    #[serde(rename = "talentDialogProfileTextId")]
    pub talent_dialog_profile_text_id: Option<Vec<i32>>,
    #[serde(rename = "talentDialogVoiceResourceName")]
    pub talent_dialog_voice_resource_name: Option<Vec<String>>,
    #[serde(rename = "victoryDialogProfileTextId")]
    pub victory_dialog_profile_text_id: Vec<i32>,
    #[serde(rename = "victoryDialogVoiceResourceName")]
    pub victory_dialog_voice_resource_name: Vec<String>,
    #[serde(rename = "contentGenreId")]
    pub content_genre_id: Option<i32>,
    #[serde(rename = "costumeBgResoureceName")]
    pub costume_bg_resourece_name: Option<String>,
}

pub struct CostumedesignconceptinfotableTable {
    records: Vec<Costumedesignconceptinfotable>,
    by_id: HashMap<i32, usize>,
}

impl CostumedesignconceptinfotableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Costumedesignconceptinfotable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Costumedesignconceptinfotable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Costumedesignconceptinfotable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Costumedesignconceptinfotable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
