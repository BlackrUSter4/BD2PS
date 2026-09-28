pub mod mini_event_hub_info;
pub mod mini_game_action_end;
pub mod mini_game_action_info;
pub mod mini_game_action_ranking;
pub mod mini_game_action_start;
pub mod mini_game_action_user_record_info;
pub mod mini_game_bingo_info;
pub mod mini_game_bingo_play;
pub mod mini_game_board_info;
pub mod mini_game_board_play;
pub mod mini_game_defense_end;
pub mod mini_game_defense_info;
pub mod mini_game_defense_matching;
pub mod mini_game_defense_start;
pub mod mini_game_field_end;
pub mod mini_game_field_info;
pub mod mini_game_field_quick_reward;
pub mod mini_game_field_reward;
pub mod mini_game_field_score;
pub mod mini_game_field_start;
pub mod mini_game_hopscotch_game_end;
pub mod mini_game_hopscotch_game_start;
pub mod mini_game_hopscotch_info;
pub mod mini_game_hopscotch_ranking_info;
pub mod mini_game_hopscotch_user_record_info;
pub mod mini_game_hub_info;
pub mod mini_game_ranking;
pub mod mini_game_relay_server_info;
pub mod mini_game_rhythm_info;
pub mod mini_game_rhythm_play_end;
pub mod mini_game_rhythm_play_start;
pub mod mini_game_rhythm_ranking;
pub mod mini_game_rhythm_user_record_info;
pub mod mini_game_roulette_draw;
pub mod mini_game_roulette_info;
pub mod mini_game_run_end;
pub mod mini_game_run_info;
pub mod mini_game_run_quick_reward;
pub mod mini_game_run_start;
pub mod mini_game_sichuan_end;
pub mod mini_game_sichuan_info;
pub mod mini_game_sichuan_ranking;
pub mod mini_game_sichuan_start;
pub mod mini_game_sichuan_user_record_info;
pub mod mini_game_survival_char_upgrade;
pub mod mini_game_survival_char_upgrade_reset;
pub mod mini_game_survival_end;
pub mod mini_game_survival_info;
pub mod mini_game_survival_play;
pub mod mini_game_survival_ranking;
pub mod mini_game_survival_skill_select_list;
pub mod mini_game_survival_skill_up;
pub mod mini_game_survival_start;
pub mod mini_game_survival_user_record_info;
pub mod mini_game_user_record_info;
pub mod mini_puzzle_all_open;
pub mod mini_puzzle_info;
pub mod mini_puzzle_open;
pub mod mini_puzzle_renew;

use bd2::proto::proto_net::Notify;

/// No `MiniGameHopscotch*Table` master data was captured (that screen was never visited during
/// capture). These are documented, reasonable placeholders standing in for stage layout/rewards;
/// revisit if real data is ever captured. See tools/BD2DataExtractor.
pub const HOPSCOTCH_STAGE_IDS: [i32; 3] = [1, 2, 3];
pub const HOPSCOTCH_CLEAR_REWARD_IDS: [i32; 1] = [1];
pub const HOPSCOTCH_RANKING_LIMIT: i64 = 100;

pub fn default_notify() -> Notify {
    Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    }
}
