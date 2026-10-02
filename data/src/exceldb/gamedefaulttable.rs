// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gamedefaulttable {
    #[serde(rename = "DefaultEquipmentUpgradeCost", default)]
    pub default_equipment_upgrade_cost: i32,
    #[serde(rename = "EncountDelayTime", default)]
    pub encount_delay_time: i32,
    #[serde(rename = "MaxEquipmentDismentleCount", default)]
    pub max_equipment_dismentle_count: i32,
    #[serde(rename = "MaxEquipmentRefineStreak", default)]
    pub max_equipment_refine_streak: i32,
    #[serde(rename = "MaxEquipmentUpgradeCount", default)]
    pub max_equipment_upgrade_count: i32,
    #[serde(rename = "MaxEquipmentUpgradeStreak", default)]
    pub max_equipment_upgrade_streak: i32,
    #[serde(rename = "addEquipInvenPriceCount", default)]
    pub add_equip_inven_price_count: i32,
    #[serde(rename = "addEquipInvenPriceType", default)]
    pub add_equip_inven_price_type: i32,
    #[serde(rename = "addEquipStoragePriceCount", default)]
    pub add_equip_storage_price_count: i32,
    #[serde(rename = "addEquipStoragePriceType", default)]
    pub add_equip_storage_price_type: i32,
    #[serde(rename = "addInvenPriceCount", default)]
    pub add_inven_price_count: i32,
    #[serde(rename = "addInvenPriceType", default)]
    pub add_inven_price_type: i32,
    #[serde(rename = "addStoragePriceCount", default)]
    pub add_storage_price_count: i32,
    #[serde(rename = "addStoragePriceType", default)]
    pub add_storage_price_type: i32,
    #[serde(rename = "chargeTimePerPvpTicket", default)]
    pub charge_time_per_pvp_ticket: i32,
    #[serde(rename = "checkActiveOnEnterCompletePackId", default)]
    pub check_active_on_enter_complete_pack_id: Vec<i32>,
    #[serde(rename = "checkActiveOnEnterCompleteTutorialId", default)]
    pub check_active_on_enter_complete_tutorial_id: i32,
    #[serde(rename = "dailyResetTime", default)]
    pub daily_reset_time: String,
    #[serde(rename = "dashCooltime", default)]
    pub dash_cooltime: f32,
    #[serde(rename = "dashCount", default)]
    pub dash_count: i32,
    #[serde(rename = "dashSpeed", default)]
    pub dash_speed: f32,
    #[serde(rename = "dashTime", default)]
    pub dash_time: f32,
    #[serde(rename = "defaultCharGroupId", default)]
    pub default_char_group_id: i32,
    #[serde(rename = "defaultCostumeId", default)]
    pub default_costume_id: i32,
    #[serde(rename = "defaultEquipInvenSlotCount", default)]
    pub default_equip_inven_slot_count: i32,
    #[serde(rename = "defaultEquipStorageSlotCount", default)]
    pub default_equip_storage_slot_count: i32,
    #[serde(rename = "defaultInvenSlotCount", default)]
    pub default_inven_slot_count: i32,
    #[serde(rename = "defaultStorageSlotCount", default)]
    pub default_storage_slot_count: i32,
    #[serde(rename = "defaultTalkCharCostumeId", default)]
    pub default_talk_char_costume_id: i32,
    #[serde(rename = "equipFilterOptCount", default)]
    pub equip_filter_opt_count: i32,
    #[serde(rename = "equipFilterSubOptCount", default)]
    pub equip_filter_sub_opt_count: i32,
    #[serde(rename = "equipmentPresetBaseCount", default)]
    pub equipment_preset_base_count: i32,
    #[serde(rename = "eventApMax", default)]
    pub event_ap_max: i32,
    #[serde(rename = "evilCastleRewardObjectName", default)]
    pub evil_castle_reward_object_name: String,
    #[serde(rename = "fieldQuestUsableTalentSkill", default)]
    pub field_quest_usable_talent_skill: Vec<i32>,
    #[serde(rename = "firstLimitGachaId", default)]
    pub first_limit_gacha_id: i32,
    #[serde(rename = "gachaEventAddDailyPayGachaCount", default)]
    pub gacha_event_add_daily_pay_gacha_count: i32,
    #[serde(rename = "gachaEventAddFreeCount", default)]
    pub gacha_event_add_free_count: i32,
    #[serde(rename = "gachaPointEndMailId", default)]
    pub gacha_point_end_mail_id: i32,
    #[serde(rename = "giveRecipeMaxSkillLevel", default)]
    pub give_recipe_max_skill_level: i32,
    #[serde(rename = "grPresetBaseCount", default)]
    pub gr_preset_base_count: i32,
    #[serde(rename = "grPresetMaxCount", default)]
    pub gr_preset_max_count: i32,
    #[serde(rename = "growUpGuideAchieveLevel", default)]
    pub grow_up_guide_achieve_level: i32,
    #[serde(rename = "homeDefaultBackgroundPath", default)]
    pub home_default_background_path: i32,
    #[serde(rename = "huntDispatchLimitCount", default)]
    pub hunt_dispatch_limit_count: i32,
    #[serde(rename = "huntingApMax", default)]
    pub hunting_ap_max: i32,
    #[serde(rename = "initPackId", default)]
    pub init_pack_id: i32,
    #[serde(rename = "linkFeedbackUrlCn", default)]
    pub link_feedback_url_cn: String,
    #[serde(rename = "linkFeedbackUrlEn", default)]
    pub link_feedback_url_en: String,
    #[serde(rename = "linkFeedbackUrlJp", default)]
    pub link_feedback_url_jp: String,
    #[serde(rename = "linkFeedbackUrlKr", default)]
    pub link_feedback_url_kr: String,
    #[serde(rename = "linkFeedbackUrlTw", default)]
    pub link_feedback_url_tw: String,
    #[serde(rename = "loseLikabilityPoint", default)]
    pub lose_likability_point: i32,
    #[serde(rename = "mailHistoryPeriodDate", default)]
    pub mail_history_period_date: i32,
    #[serde(rename = "mailViewLimitCount", default)]
    pub mail_view_limit_count: i32,
    #[serde(rename = "maxAtkCollectionBuff", default)]
    pub max_atk_collection_buff: f32,
    #[serde(rename = "maxDeckNormalTypeCount", default)]
    pub max_deck_normal_type_count: i32,
    #[serde(rename = "maxDeckTempTypeCount", default)]
    pub max_deck_temp_type_count: i32,
    #[serde(rename = "maxEquipInvenSlotCount", default)]
    pub max_equip_inven_slot_count: i32,
    #[serde(rename = "maxEquipStorageSlotCount", default)]
    pub max_equip_storage_slot_count: i32,
    #[serde(rename = "maxFreePvpTicket", default)]
    pub max_free_pvp_ticket: i32,
    #[serde(rename = "maxHpCollectionBuff", default)]
    pub max_hp_collection_buff: f32,
    #[serde(rename = "maxInvenCost", default)]
    pub max_inven_cost: i32,
    #[serde(rename = "maxInvenSlotCount", default)]
    pub max_inven_slot_count: i32,
    #[serde(rename = "maxMgAtkCollectionBuff", default)]
    pub max_mg_atk_collection_buff: f32,
    #[serde(rename = "maxStorageSlotCount", default)]
    pub max_storage_slot_count: i32,
    #[serde(rename = "mhPresetBaseCount", default)]
    pub mh_preset_base_count: i32,
    #[serde(rename = "mhPresetMaxCount", default)]
    pub mh_preset_max_count: i32,
    #[serde(rename = "moreSixteenUnderTwentyBillingLimit", default)]
    pub more_sixteen_under_twenty_billing_limit: i32,
    #[serde(rename = "partnerLikabilityMax", default)]
    pub partner_likability_max: i32,
    #[serde(rename = "popularEquipConditionPvpRanking", default)]
    pub popular_equip_condition_pvp_ranking: i32,
    #[serde(rename = "popularEquipValidity", default)]
    pub popular_equip_validity: i32,
    #[serde(rename = "popupDisabledDuration", default)]
    pub popup_disabled_duration: i32,
    #[serde(rename = "presetBaseCount", default)]
    pub preset_base_count: i32,
    #[serde(rename = "presetBuyCount", default)]
    pub preset_buy_count: i32,
    #[serde(rename = "presetBuyType", default)]
    pub preset_buy_type: i32,
    #[serde(rename = "presetMaxCount", default)]
    pub preset_max_count: i32,
    #[serde(rename = "returnDayCount", default)]
    pub return_day_count: i32,
    #[serde(rename = "returnUserPeriod", default)]
    pub return_user_period: i32,
    #[serde(rename = "reviewPopupConditionPackId", default)]
    pub review_popup_condition_pack_id: i32,
    #[serde(rename = "reviewPopupConditionQuestId", default)]
    pub review_popup_condition_quest_id: i32,
    #[serde(rename = "roguelikeApMax", default)]
    pub roguelike_ap_max: i32,
    #[serde(rename = "shopBuyMaxCount", default)]
    pub shop_buy_max_count: i32,
    #[serde(rename = "silhouetteModeDelayTime", default)]
    pub silhouette_mode_delay_time: i32,
    #[serde(rename = "silhouetteModeIllustCharId", default)]
    pub silhouette_mode_illust_char_id: i32,
    #[serde(rename = "speechBubbleGapTime", default)]
    pub speech_bubble_gap_time: i32,
    #[serde(rename = "speechBubbleRemind", default)]
    pub speech_bubble_remind: i32,
    #[serde(rename = "talentSkillMakingDevideValue", default)]
    pub talent_skill_making_devide_value: i32,
    #[serde(rename = "talentSkillPinMaxCount", default)]
    pub talent_skill_pin_max_count: i32,
    #[serde(rename = "todayQuestLimitCount", default)]
    pub today_quest_limit_count: i32,
    #[serde(rename = "todayQuestPackPostCount", default)]
    pub today_quest_pack_post_count: i32,
    #[serde(rename = "torchLightApMax", default)]
    pub torch_light_ap_max: i32,
    #[serde(rename = "twPresetBaseCount", default)]
    pub tw_preset_base_count: i32,
    #[serde(rename = "twPresetMaxCount", default)]
    pub tw_preset_max_count: i32,
    #[serde(rename = "underSixteenBillingLimit", default)]
    pub under_sixteen_billing_limit: i32,
    #[serde(rename = "unnormalBattlePower", default)]
    pub unnormal_battle_power: i32,
    #[serde(rename = "useBossHuntingAp", default)]
    pub use_boss_hunting_ap: i32,
    #[serde(rename = "useNormalHuntingAp", default)]
    pub use_normal_hunting_ap: i32,
    #[serde(rename = "winLikabilityPoint", default)]
    pub win_likability_point: i32,
    #[serde(rename = "EquipmentTryConut", default)]
    pub equipment_try_conut: Option<i32>,
    #[serde(rename = "acivementFavorites", default)]
    pub acivement_favorites: Option<i32>,
    #[serde(rename = "colosseumPresetBaseCount", default)]
    pub colosseum_preset_base_count: Option<i32>,
    #[serde(rename = "colosseumPresetMaxCount", default)]
    pub colosseum_preset_max_count: Option<i32>,
    #[serde(rename = "costumeMail", default)]
    pub costume_mail: Option<i32>,
    #[serde(rename = "defaultFishInvenSlotCount", default)]
    pub default_fish_inven_slot_count: Option<i32>,
    #[serde(rename = "defaultFishingItemInvenSlotCount", default)]
    pub default_fishing_item_inven_slot_count: Option<i32>,
    #[serde(rename = "defaultRodInvenSlotCount", default)]
    pub default_rod_inven_slot_count: Option<i32>,
    #[serde(rename = "defaultSettingHomeLobby", default)]
    pub default_setting_home_lobby: Option<i32>,
    #[serde(rename = "equipmentOnceClear", default)]
    pub equipment_once_clear: Option<i32>,
    #[serde(rename = "lifeContentCategoryPackId", default)]
    pub life_content_category_pack_id: Option<Vec<i32>>,
    #[serde(rename = "maxBoxOpenCount", default)]
    pub max_box_open_count: Option<i32>,
    #[serde(rename = "maxCritRateCounselBuff", default)]
    pub max_crit_rate_counsel_buff: Option<f32>,
    #[serde(rename = "maxFishInvenSlotCount", default)]
    pub max_fish_inven_slot_count: Option<i32>,
    #[serde(rename = "maxFishingItemInvenSlotCount", default)]
    pub max_fishing_item_inven_slot_count: Option<i32>,
    #[serde(rename = "maxRodInvenSlotCount", default)]
    pub max_rod_inven_slot_count: Option<i32>,
    #[serde(rename = "rankingRange", default)]
    pub ranking_range: Option<i32>,
    #[serde(rename = "resetDayOfWeek", default)]
    pub reset_day_of_week: Option<i32>,
    #[serde(rename = "todayQuestAchievementScore", default)]
    pub today_quest_achievement_score: Option<i32>,
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
    pub fn iter(&self) -> std::slice::Iter<'_, Gamedefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
