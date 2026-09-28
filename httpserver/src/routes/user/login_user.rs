use actix_web::{HttpResponse, Result, http::header, put, web};
use bd2::prost::Message;
use bd2::proto::proto_net::{
    AchievementUpdateInfo, LoginUserRequest, LoginUserResponse, Notify, RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::{GameResponse, parse_packet};
use gameserver::logic::game::account;
use sqlx::SqlitePool;
use tracing::{error, info, warn};

#[put("LoginUser")]
pub async fn login_user_handler(pool: web::Data<SqlitePool>, body: String) -> Result<HttpResponse> {
    // Parse incoming protobuf request
    let req = match parse_packet::<LoginUserRequest>("LoginUser", &body) {
        Ok(r) => r,
        Err(e) => {
            warn!("Failed to parse LoginUser request: {}", e);
            return Ok(HttpResponse::Ok().json(GameResponse::error(400)));
        }
    };

    // Extract and parse UID from token
    let access_token = req.access_token.as_deref().unwrap_or_default();
    info!("Login attempt with token: {}", access_token);

    let uid = match account::parse_uid_from_token(access_token) {
        Ok(uid) => {
            info!("Parsed UID: {}", uid);
            uid
        }
        Err(e) => {
            error!("Invalid access token: {}", e);
            return Ok(HttpResponse::Ok().json(GameResponse::error(401)));
        }
    };

    // Get or create user
    let (_account, user_info) = match account::get_or_create_user(&pool, uid).await {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to get/create user: {}", e);
            return Ok(HttpResponse::Ok().json(GameResponse::error(500)));
        }
    };

    // Update login timestamp
    if let Err(e) = account::update_login_timestamp(&pool, uid).await {
        error!("Failed to update login timestamp: {}", e);
    }

    // Get owner_index from UserMapper table
    let owner_index = match account::get_owner_index_for_uid(&pool, uid).await {
        Ok(idx) => {
            info!("Found owner_index {} for UID {}", idx, uid);
            idx
        }
        Err(e) => {
            error!("Failed to get owner_index for UID {}: {}", uid, e);
            return Ok(HttpResponse::Ok().json(GameResponse::error(500)));
        }
    };

    // Generate a new access token (random 64-character string)
    let new_access_token = generate_access_token();

    // Build protobuf response
    let response = LoginUserResponse {
        user_info: Some(user_info.to_proto()),
        accumulated_payment_amount: Some(0.0),
        date_of_birth: Some(0),
        reward_info_bundle: Some(RewardDbInfoBundle::default()),
        user_guild_info: None,
        guild_base_info: None,
        has_available_supporter_reward: Some(false),
        ..Default::default()
    };

    // Build notify
    let notify = Notify {
        achievement_update_info: vec![AchievementUpdateInfo {
            group_id: Some(349),
            value: Some(1),
            is_set: Some(false),
        }],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: None,
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    // Set authentication cookie in format: access_token|owner_index
    let auth_value = format!("{}|{}", new_access_token, owner_index);
    info!("Setting auth header: {}", auth_value);

    // Build response
    let (route, code) = PacketCodeType::LoginUser.info();
    let result = GameResponse::success(route, &response.encode_to_vec(), code).with_notify(&notify);

    // Return response with raw set-cookie header (no cookie name or attributes!)
    Ok(HttpResponse::Ok()
        .insert_header((header::SET_COOKIE, auth_value))
        .json(result))
}

/// Generate a random access token (64 alphanumeric characters)
fn generate_access_token() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::rng();

    (0..64)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
