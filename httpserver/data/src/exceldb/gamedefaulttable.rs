// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gamedefaulttable {
    #[serde(rename = "DefaultEquipmentUpgradeCost")]
    pub default_equipment_upgrade_cost: i32,
    #[serde(rename = "EncountDelayTime")]
    pub encount_delay_time: i32,
    #[serde(rename = "MaxEquipmentDismentleCount")]
    pub max_equipment_dismentle_count: i32,
    #[serde(rename = "MaxEquipmentRefineStreak")]
    pub max_equipment_refine_streak: i32,
    #[serde(rename = "MaxEquipmentUpgradeCount")]
    pub max_equipment_upgrade_count: i32,
    #[serde(rename = "MaxEquipmentUpgradeStreak")]
    pub max_equipment_upgrade_streak: i32,
    #[serde(rename = "addEquipInvenPriceCount")]
    pub add_equip_inven_price_count: i32,
    #[serde(rename = "addEquipInvenPriceType")]
    pub add_equip_inven_price_type: i32,
    #[serde(rename = "addEquipStoragePriceCount")]
    pub add_equip_storage_price_count: i32,
    #[serde(rename = "addEquipStoragePriceType")]
    pub add_equip_storage_price_type: i32,
    #[serde(rename = "addInvenPriceCount")]
    pub add_inven_price_count: i32,
    #[serde(rename = "addInvenPriceType")]
    pub add_inven_price_type: i32,
    #[serde(rename = "addStoragePriceCount")]
    pub add_storage_price_count: i32,
    #[serde(rename = "addStoragePriceType")]
    pub add_storage_price_type: i32,
    #[serde(rename = "chargeTimePerPvpTicket")]
    pub charge_time_per_pvp_ticket: i32,
    #[serde(rename = "checkActiveOnEnterCompletePackId")]
    pub check_active_on_enter_complete_pack_id: Vec<i32>,
    #[serde(rename = "checkActiveOnEnterCompleteTutorialId")]
    pub check_active_on_enter_complete_tutorial_id: i32,
    #[serde(rename = "dailyResetTime")]
    pub daily_reset_time: String,
    #[serde(rename = "dashCooltime")]
    pub dash_cooltime: f32,
    #[serde(rename = "dashCount")]
    pub dash_count: i32,
    #[serde(rename = "dashSpeed")]
    pub dash_speed: f32,
    #[serde(rename = "dashTime")]
    pub dash_time: f32,
    #[serde(rename = "defaultCharGroupId")]
    pub default_char_group_id: i32,
    #[serde(rename = "defaultCostumeId")]
    pub default_costume_id: i32,
    #[serde(rename = "defaultEquipInvenSlotCount")]
    pub default_equip_inven_slot_count: i32,
    #[serde(rename = "defaultEquipStorageSlotCount")]
    pub default_equip_storage_slot_count: i32,
    #[serde(rename = "defaultInvenSlotCount")]
    pub default_inven_slot_count: i32,
    #[serde(rename = "defaultStorageSlotCount")]
    pub default_storage_slot_count: i32,
    #[serde(rename = "defaultTalkCharCostumeId")]
    pub default_talk_char_costume_id: i32,
    #[serde(rename = "equipFilterOptCount")]
    pub equip_filter_opt_count: i32,
    #[serde(rename = "equipFilterSubOptCount")]
    pub equip_filter_sub_opt_count: i32,
    #[serde(rename = "equipmentPresetBaseCount")]
    pub equipment_preset_base_count: i32,
    #[serde(rename = "eventApMax")]
    pub event_ap_max: i32,
    #[serde(rename = "evilCastleRewardObjectName")]
    pub evil_castle_reward_object_name: String,
    #[serde(rename = "fieldQuestUsableTalentSkill")]
    pub field_quest_usable_talent_skill: Vec<i32>,
    #[serde(rename = "firstLimitGachaId")]
    pub first_limit_gacha_id: i32,
    #[serde(rename = "gachaEventAddDailyPayGachaCount")]
    pub gacha_event_add_daily_pay_gacha_count: i32,
    #[serde(rename = "gachaEventAddFreeCount")]
    pub gacha_event_add_free_count: i32,
    #[serde(rename = "gachaPointEndMailId")]
    pub gacha_point_end_mail_id: i32,
    #[serde(rename = "giveRecipeMaxSkillLevel")]
    pub give_recipe_max_skill_level: i32,
    #[serde(rename = "grPresetBaseCount")]
    pub gr_preset_base_count: i32,
    #[serde(rename = "grPresetMaxCount")]
    pub gr_preset_max_count: i32,
    #[serde(rename = "growUpGuideAchieveLevel")]
    pub grow_up_guide_achieve_level: i32,
    #[serde(rename = "homeDefaultBackgroundPath")]
    pub home_default_background_path: i32,
    #[serde(rename = "huntDispatchLimitCount")]
    pub hunt_dispatch_limit_count: i32,
    #[serde(rename = "huntingApMax")]
    pub hunting_ap_max: i32,
    #[serde(rename = "initPackId")]
    pub init_pack_id: i32,
    #[serde(rename = "linkFeedbackUrlCn")]
    pub link_feedback_url_cn: String,
    #[serde(rename = "linkFeedbackUrlEn")]
    pub link_feedback_url_en: String,
    #[serde(rename = "linkFeedbackUrlJp")]
    pub link_feedback_url_jp: String,
    #[serde(rename = "linkFeedbackUrlKr")]
    pub link_feedback_url_kr: String,
    #[serde(rename = "linkFeedbackUrlTw")]
    pub link_feedback_url_tw: String,
    #[serde(rename = "loseLikabilityPoint")]
    pub lose_likability_point: i32,
    #[serde(rename = "mailHistoryPeriodDate")]
    pub mail_history_period_date: i32,
    #[serde(rename = "mailViewLimitCount")]
    pub mail_view_limit_count: i32,
    #[serde(rename = "maxAtkCollectionBuff")]
    pub max_atk_collection_buff: f32,
    #[serde(rename = "maxDeckNormalTypeCount")]
    pub max_deck_normal_type_count: i32,
    #[serde(rename = "maxDeckTempTypeCount")]
    pub max_deck_temp_type_count: i32,
    #[serde(rename = "maxEquipInvenSlotCount")]
    pub max_equip_inven_slot_count: i32,
    #[serde(rename = "maxEquipStorageSlotCount")]
    pub max_equip_storage_slot_count: i32,
    #[serde(rename = "maxFreePvpTicket")]
    pub max_free_pvp_ticket: i32,
    #[serde(rename = "maxHpCollectionBuff")]
    pub max_hp_collection_buff: f32,
    #[serde(rename = "maxInvenCost")]
    pub max_inven_cost: i32,
    #[serde(rename = "maxInvenSlotCount")]
    pub max_inven_slot_count: i32,
    #[serde(rename = "maxMgAtkCollectionBuff")]
    pub max_mg_atk_collection_buff: f32,
    #[serde(rename = "maxStorageSlotCount")]
    pub max_storage_slot_count: i32,
    #[serde(rename = "mhPresetBaseCount")]
    pub mh_preset_base_count: i32,
    #[serde(rename = "mhPresetMaxCount")]
    pub mh_preset_max_count: i32,
    #[serde(rename = "moreSixteenUnderTwentyBillingLimit")]
    pub more_sixteen_under_twenty_billing_limit: i32,
    #[serde(rename = "partnerLikabilityMax")]
    pub partner_likability_max: i32,
    #[serde(rename = "popularEquipConditionPvpRanking")]
    pub popular_equip_condition_pvp_ranking: i32,
    #[serde(rename = "popularEquipValidity")]
    pub popular_equip_validity: i32,
    #[serde(rename = "popupDisabledDuration")]
    pub popup_disabled_duration: i32,
    #[serde(rename = "presetBaseCount")]
    pub preset_base_count: i32,
    #[serde(rename = "presetBuyCount")]
    pub preset_buy_count: i32,
    #[serde(rename = "presetBuyType")]
    pub preset_buy_type: i32,
    #[serde(rename = "presetMaxCount")]
    pub preset_max_count: i32,
    #[serde(rename = "returnDayCount")]
    pub return_day_count: i32,
    #[serde(rename = "returnUserPeriod")]
    pub return_user_period: i32,
    #[serde(rename = "reviewPopupConditionPackId")]
    pub review_popup_condition_pack_id: i32,
    #[serde(rename = "reviewPopupConditionQuestId")]
    pub review_popup_condition_quest_id: i32,
    #[serde(rename = "roguelikeApMax")]
    pub roguelike_ap_max: i32,
    #[serde(rename = "shopBuyMaxCount")]
    pub shop_buy_max_count: i32,
    #[serde(rename = "silhouetteModeDelayTime")]
    pub silhouette_mode_delay_time: i32,
    #[serde(rename = "silhouetteModeIllustCharId")]
    pub silhouette_mode_illust_char_id: i32,
    #[serde(rename = "speechBubbleGapTime")]
    pub speech_bubble_gap_time: i32,
    #[serde(rename = "speechBubbleRemind")]
    pub speech_bubble_remind: i32,
    #[serde(rename = "talentSkillMakingDevideValue")]
    pub talent_skill_making_devide_value: i32,
    #[serde(rename = "talentSkillPinMaxCount")]
    pub talent_skill_pin_max_count: i32,
    #[serde(rename = "todayQuestLimitCount")]
    pub today_quest_limit_count: i32,
    #[serde(rename = "todayQuestPackPostCount")]
    pub today_quest_pack_post_count: i32,
    #[serde(rename = "torchLightApMax")]
    pub torch_light_ap_max: i32,
    #[serde(rename = "twPresetBaseCount")]
    pub tw_preset_base_count: i32,
    #[serde(rename = "twPresetMaxCount")]
    pub tw_preset_max_count: i32,
    #[serde(rename = "underSixteenBillingLimit")]
    pub under_sixteen_billing_limit: i32,
    #[serde(rename = "unnormalBattlePower")]
    pub unnormal_battle_power: i32,
    #[serde(rename = "useBossHuntingAp")]
    pub use_boss_hunting_ap: i32,
    #[serde(rename = "useNormalHuntingAp")]
    pub use_normal_hunting_ap: i32,
    #[serde(rename = "winLikabilityPoint")]
    pub win_likability_point: i32,
}

pub struct GamedefaulttableTable {
    records: Vec<Gamedefaulttable>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl GamedefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Gamedefaulttable> = serde_json::from_str(&json)?;
        
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_group.entry(record.default_char_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Gamedefaulttable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Gamedefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Gamedefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
