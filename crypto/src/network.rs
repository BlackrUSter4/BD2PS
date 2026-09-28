use crate::aestools::{AesTools, EncryptionKey, DEFAULT_KEY, ENCRYPT_FIX_KEY};
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use prost::Message;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Debug, thiserror::Error)]
pub enum PacketError {
    #[error("empty request body")]
    EmptyBody,
    #[error("decryption failed")]
    DecryptionFailed,
    #[error("base64 decode failed: {0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("protobuf decode failed: {0}")]
    ProtobufError(#[from] prost::DecodeError),
    #[error("utf8 decode failed")]
    Utf8Error,
}

/// Parse and decrypt incoming packet data
pub fn parse_packet<M: Message + Default>(route_name: &str, body: &str) -> Result<M, PacketError> {
    if body.trim().is_empty() {
        error!("Empty request body for route {}", route_name);
        return Err(PacketError::EmptyBody);
    }

    let key_type = AesTools::get_key_for_route(route_name);

    // Try decryption first, fall back to plain base64 if it fails (for batch requests)
    let proto_bytes = match key_type {
        Some(EncryptionKey::Fixed) => {
            info!("Decrypting using ENCRYPT_FIX_KEY for route {}", route_name);
            // Try decrypt: Base64(AES(Base64(proto)))
            match try_decrypt_double_base64(body, ENCRYPT_FIX_KEY) {
                Ok(bytes) => bytes,
                Err(_) => {
                    // Decryption failed, try plain base64 (batch request)
                    info!("trying plain base64 for route {}", route_name);
                    general_purpose::STANDARD
                        .decode(body)
                        .map_err(|e| PacketError::Base64Error(e))?
                }
            }
        }
        Some(EncryptionKey::Default) => {
            info!("Decrypting using DEFAULT_KEY for route {}", route_name);
            // Try decrypt: Base64(AES(Base64(proto)))
            match try_decrypt_double_base64(body, DEFAULT_KEY) {
                Ok(bytes) => bytes,
                Err(_) => {
                    // Decryption failed, try plain base64 (batch request)
                    info!("trying plain base64 for route {}", route_name);
                    general_purpose::STANDARD
                        .decode(body)
                        .map_err(|e| PacketError::Base64Error(e))?
                }
            }
        }
        None => {
            info!("Non-encrypted route: {}", route_name);
            // Direct base64 decode
            general_purpose::STANDARD.decode(body)?
        }
    };

    if proto_bytes.is_empty() {
        return Err(PacketError::DecryptionFailed);
    }

    Ok(M::decode(&*proto_bytes)?)
}

/// Helper function to try double base64 + AES decryption
fn try_decrypt_double_base64(body: &str, key: &str) -> Result<Vec<u8>, PacketError> {
    // Decrypt: Base64(AES(Base64(proto)))
    let decrypted =
        AesTools::aes_decrypt_raw(body, key).map_err(|_| PacketError::DecryptionFailed)?;

    // Inner layer is base64
    let inner_b64 = String::from_utf8(decrypted).map_err(|_| PacketError::Utf8Error)?;

    general_purpose::STANDARD
        .decode(&inner_b64)
        .map_err(|e| PacketError::Base64Error(e))
}

/// Encode and encrypt a protobuf message
pub fn encode_packet<M: Message>(route_name: &str, msg: &M) -> String {
    let bytes = msg.encode_to_vec();
    AesTools::encode_packet(route_name, &bytes)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseResponse {
    pub error_type: i32,
    pub error_message: String,
    pub data: String,
    pub length: i32,
    pub ip: String,
}

impl BaseResponse {
    pub fn success(data: &[u8]) -> Self {
        let encoded = general_purpose::STANDARD.encode(data);
        Self {
            error_type: 0,
            error_message: String::new(),
            data: encoded,
            length: 0, // Game hardcodes this
            ip: "127.0.0.1".to_string(),
        }
    }

    pub fn error(error_type: i32) -> Self {
        Self {
            error_type,
            error_message: String::new(),
            data: String::new(),
            length: 0,
            ip: "127.0.0.1".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameResponse {
    pub error_type: i32,
    pub packet_code: i32,
    pub error_message: String,
    pub length: i32,
    pub data: String,
    pub server_now_time: i64,
    pub notify: String,
}

impl GameResponse {
    pub fn success(route_name: &str, proto_bytes: &[u8], packet_code: i32) -> Self {
        let inner_b64 = general_purpose::STANDARD.encode(proto_bytes);
        let encrypted = AesTools::encode_packet(route_name, proto_bytes);

        Self {
            error_type: 0,
            packet_code,
            error_message: String::new(),
            length: inner_b64.len() as i32,
            data: encrypted,
            server_now_time: Utc::now().timestamp_millis(),
            notify: String::new(),
        }
    }

    pub fn error(error_type: i32) -> Self {
        Self {
            error_type,
            packet_code: 0,
            error_message: String::new(),
            length: 0,
            data: String::new(),
            server_now_time: 0,
            notify: String::new(),
        }
    }

    /// Attach a notify message (plain base64, not encrypted)
    pub fn with_notify<M: Message>(mut self, notify: &M) -> Self {
        let bytes = notify.encode_to_vec();
        self.notify = general_purpose::STANDARD.encode(bytes);
        self
    }
}
