use bd2::prost::Message;
use bd2::proto::proto_net::{ChatSettingUpdateRequest, ChatSettingUpdateResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::chat::chat_setting_info;
use database::models::game::chat::chat_setting_info::ChatSettingInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account chat preference persistence.
pub async fn handle(pool: &SqlitePool, uid: i64, req: ChatSettingUpdateRequest) -> GameResponse {
    info!("Handling ChatSettingUpdateRequest: {:?}", req);

    let info = ChatSettingInfo {
        uid,
        auto_translate_flag: req.auto_translate_flag.unwrap_or(0),
        global_chat_flag: req.global_chat_flag.unwrap_or(0),
        visual_flag: req.visual_flag.unwrap_or(0),
    };
    let _ = chat_setting_info::save(pool, &info).await;

    let response = ChatSettingUpdateResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ChatSettingUpdate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
