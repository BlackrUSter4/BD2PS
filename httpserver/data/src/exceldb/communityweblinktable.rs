// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Communityweblinktable {
    #[serde(rename = "communityRewardCount")]
    pub community_reward_count: Vec<i32>,
    #[serde(rename = "communityRewardId")]
    pub community_reward_id: Vec<i32>,
    #[serde(rename = "communityRewardType")]
    pub community_reward_type: Vec<i32>,
    #[serde(rename = "linkIconCn1")]
    pub link_icon_cn1: String,
    #[serde(rename = "linkIconCn2")]
    pub link_icon_cn2: String,
    #[serde(rename = "linkIconCn3")]
    pub link_icon_cn3: String,
    #[serde(rename = "linkIconEn1")]
    pub link_icon_en1: String,
    #[serde(rename = "linkIconEn2")]
    pub link_icon_en2: String,
    #[serde(rename = "linkIconEn3")]
    pub link_icon_en3: String,
    #[serde(rename = "linkIconJp1")]
    pub link_icon_jp1: String,
    #[serde(rename = "linkIconJp2")]
    pub link_icon_jp2: String,
    #[serde(rename = "linkIconJp3")]
    pub link_icon_jp3: String,
    #[serde(rename = "linkIconKr1")]
    pub link_icon_kr1: String,
    #[serde(rename = "linkIconKr2")]
    pub link_icon_kr2: String,
    #[serde(rename = "linkIconKr3")]
    pub link_icon_kr3: String,
    #[serde(rename = "linkIconTw1")]
    pub link_icon_tw1: String,
    #[serde(rename = "linkIconTw2")]
    pub link_icon_tw2: String,
    #[serde(rename = "linkIconTw3")]
    pub link_icon_tw3: String,
    #[serde(rename = "linkUrlPathCn1")]
    pub link_url_path_cn1: String,
    #[serde(rename = "linkUrlPathCn2")]
    pub link_url_path_cn2: String,
    #[serde(rename = "linkUrlPathCn3")]
    pub link_url_path_cn3: String,
    #[serde(rename = "linkUrlPathEn1")]
    pub link_url_path_en1: String,
    #[serde(rename = "linkUrlPathEn2")]
    pub link_url_path_en2: String,
    #[serde(rename = "linkUrlPathEn3")]
    pub link_url_path_en3: String,
    #[serde(rename = "linkUrlPathJp1")]
    pub link_url_path_jp1: String,
    #[serde(rename = "linkUrlPathJp2")]
    pub link_url_path_jp2: String,
    #[serde(rename = "linkUrlPathJp3")]
    pub link_url_path_jp3: String,
    #[serde(rename = "linkUrlPathKr1")]
    pub link_url_path_kr1: String,
    #[serde(rename = "linkUrlPathKr2")]
    pub link_url_path_kr2: String,
    #[serde(rename = "linkUrlPathKr3")]
    pub link_url_path_kr3: String,
    #[serde(rename = "linkUrlPathTw1")]
    pub link_url_path_tw1: String,
    #[serde(rename = "linkUrlPathTw2")]
    pub link_url_path_tw2: String,
    #[serde(rename = "linkUrlPathTw3")]
    pub link_url_path_tw3: String,
    #[serde(rename = "surveyRewardCount")]
    pub survey_reward_count: Vec<i32>,
    #[serde(rename = "surveyRewardId")]
    pub survey_reward_id: Vec<i32>,
    #[serde(rename = "surveyRewardType")]
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
    pub fn iter(&self) -> std::slice::Iter<Communityweblinktable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
