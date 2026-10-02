// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Communityweblinktable {
    #[serde(rename = "communityRewardCount", default)]
    pub community_reward_count: Vec<i32>,
    #[serde(rename = "communityRewardId", default)]
    pub community_reward_id: Vec<i32>,
    #[serde(rename = "communityRewardType", default)]
    pub community_reward_type: Vec<i32>,
    #[serde(rename = "linkIconCn1", default)]
    pub link_icon_cn1: String,
    #[serde(rename = "linkIconCn2", default)]
    pub link_icon_cn2: String,
    #[serde(rename = "linkIconCn3", default)]
    pub link_icon_cn3: String,
    #[serde(rename = "linkIconEn1", default)]
    pub link_icon_en1: String,
    #[serde(rename = "linkIconEn2", default)]
    pub link_icon_en2: String,
    #[serde(rename = "linkIconEn3", default)]
    pub link_icon_en3: String,
    #[serde(rename = "linkIconJp1", default)]
    pub link_icon_jp1: String,
    #[serde(rename = "linkIconJp2", default)]
    pub link_icon_jp2: String,
    #[serde(rename = "linkIconJp3", default)]
    pub link_icon_jp3: String,
    #[serde(rename = "linkIconKr1", default)]
    pub link_icon_kr1: String,
    #[serde(rename = "linkIconKr2", default)]
    pub link_icon_kr2: String,
    #[serde(rename = "linkIconKr3", default)]
    pub link_icon_kr3: String,
    #[serde(rename = "linkIconTw1", default)]
    pub link_icon_tw1: String,
    #[serde(rename = "linkIconTw2", default)]
    pub link_icon_tw2: String,
    #[serde(rename = "linkIconTw3", default)]
    pub link_icon_tw3: String,
    #[serde(rename = "linkUrlPathCn1", default)]
    pub link_url_path_cn1: String,
    #[serde(rename = "linkUrlPathCn2", default)]
    pub link_url_path_cn2: String,
    #[serde(rename = "linkUrlPathCn3", default)]
    pub link_url_path_cn3: String,
    #[serde(rename = "linkUrlPathEn1", default)]
    pub link_url_path_en1: String,
    #[serde(rename = "linkUrlPathEn2", default)]
    pub link_url_path_en2: String,
    #[serde(rename = "linkUrlPathEn3", default)]
    pub link_url_path_en3: String,
    #[serde(rename = "linkUrlPathJp1", default)]
    pub link_url_path_jp1: String,
    #[serde(rename = "linkUrlPathJp2", default)]
    pub link_url_path_jp2: String,
    #[serde(rename = "linkUrlPathJp3", default)]
    pub link_url_path_jp3: String,
    #[serde(rename = "linkUrlPathKr1", default)]
    pub link_url_path_kr1: String,
    #[serde(rename = "linkUrlPathKr2", default)]
    pub link_url_path_kr2: String,
    #[serde(rename = "linkUrlPathKr3", default)]
    pub link_url_path_kr3: String,
    #[serde(rename = "linkUrlPathTw1", default)]
    pub link_url_path_tw1: String,
    #[serde(rename = "linkUrlPathTw2", default)]
    pub link_url_path_tw2: String,
    #[serde(rename = "linkUrlPathTw3", default)]
    pub link_url_path_tw3: String,
    #[serde(rename = "surveyRewardCount", default)]
    pub survey_reward_count: Vec<i32>,
    #[serde(rename = "surveyRewardId", default)]
    pub survey_reward_id: Vec<i32>,
    #[serde(rename = "surveyRewardType", default)]
    pub survey_reward_type: Vec<i32>,
}

pub struct CommunityweblinktableTable {
    records: Vec<Communityweblinktable>,
}

impl CommunityweblinktableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Communityweblinktable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Communityweblinktable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Communityweblinktable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
