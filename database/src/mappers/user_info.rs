use crate::models::game::user::user_info::UserInfo;
use bd2::proto::proto_net::{OptimizeBaseInfo, UserDbInfo};
use crypto::aestools::DEFAULT_KEY;

impl UserInfo {
    pub fn to_proto(&self) -> UserDbInfo {
        let optimize_info = vec![
            OptimizeBaseInfo {
                optimize_index: Some(0),
                optimize_value: Some(true),
                optimize_property: None,
            },
            OptimizeBaseInfo {
                optimize_index: Some(1),
                optimize_value: Some(true),
                optimize_property: None,
            },
            OptimizeBaseInfo {
                optimize_index: Some(2),
                optimize_value: Some(true),
                optimize_property: Some("10,1.5".into()),
            },
            OptimizeBaseInfo {
                optimize_index: Some(3),
                optimize_value: Some(true),
                optimize_property: None,
            },
            OptimizeBaseInfo {
                optimize_index: Some(4),
                optimize_value: Some(true),
                optimize_property: None,
            },
            OptimizeBaseInfo {
                optimize_index: Some(5),
                optimize_value: Some(true),
                optimize_property: Some("30,3".into()),
            },
            OptimizeBaseInfo {
                optimize_index: Some(6),
                optimize_value: Some(true),
                optimize_property: Some("5".into()),
            },
        ];

        UserDbInfo {
            optimize_info,
            owner_index: self.owner_index,
            user_id: self.user_id.clone(),
            user_key: Some(DEFAULT_KEY.to_string()),
            last_play_pack_id: self.last_play_pack_id,
            inven_slot: self.inven_slot,
            storage_slot: self.storage_slot,
            gold: self.gold,
            free_jewelry: self.free_jewelry,
            jewelry: self.jewelry,
            equip_slot: self.equip_slot,
            catalyst: self.catalyst,
            exp: self.exp,
            level_reward: self.level_reward,
            portrait_costume_id: self.portrait_costume_id,
            equip_storage_slot: self.equip_storage_slot,
            pvp_ticket: self.pvp_ticket,
            medal: self.medal,
            evil_castle_coin: self.evil_castle_coin,
            user_type: self.user_type,
            free_hunting_ap: self.free_hunting_ap,
            bonus_hunting_ap: self.bonus_hunting_ap,
            pvp_ticket_stack: self.pvp_ticket_stack,
            mileage: self.mileage,
            hope_powder: self.hope_powder,
            unreg_date: self.unreg_date,
            is_first_gacha: self.is_first_gacha.map(|v| v != true),
            preset_slot: self.preset_slot,
            block_date: self.block_date,
            return_status_end_time: self.return_status_end_time,
            my_room_slot: self.my_room_slot,
            total_war_preset_slot: self.total_war_preset_slot,
            event_ap_free: self.event_ap_free,
            event_ap_stack: self.event_ap_stack,
            free_torch_light_ap: self.free_torch_light_ap,
            torch_light_ap: self.torch_light_ap,
            join_time: self.join_time,
            newbie_pass_step: self.newbie_pass_step,
            block_reason: self.block_reason,
            rogue_like_ap: self.rogue_like_ap,
            rogue_like_ap_stack: self.rogue_like_ap_stack,
            login_date: self.login_date,

            ..Default::default()
        }
    }
}
