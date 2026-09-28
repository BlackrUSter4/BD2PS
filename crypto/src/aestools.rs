use aes::Aes256;
use base64::{engine::general_purpose, Engine as _};
use cbc::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use once_cell::sync::Lazy;
use sha1::{Digest, Sha1};
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

type Aes256CbcEnc = cbc::Encryptor<Aes256>;
type Aes256CbcDec = cbc::Decryptor<Aes256>;

pub const ENCRYPT_FIX_KEY: &str = "abcdefghijkrstuv024680wxyzlmnopq";
pub const DEFAULT_KEY: &str = "688370f00a38e21a6ca65ec2d7c38a7c";
const SALT: &str = "salt_alone";

static ZERO_IV: [u8; 16] = [0u8; 16];

static ENCRYPT_FIX_PACKET_LIST: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    HashSet::from(["LoginUser", "JoinUser"]) // double-Base64 + AES
});

static NON_ENCRYPT_PACKET_LIST: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    HashSet::from([
        "MaintenanceInfo",
        "ServerInfo",
        "ServerNowTime",
        "NoticeInfo",
    ])
});

pub enum EncryptionKey {
    Fixed,   // LoginUser / JoinUser
    Default, // other encrypted routes
}

pub struct AesTools;

impl AesTools {
    pub fn get_key_for_route(route_name: &str) -> Option<EncryptionKey> {
        let route = route_name.trim_start_matches('/').to_string();
        if ENCRYPT_FIX_PACKET_LIST.contains(route.as_str()) {
            Some(EncryptionKey::Fixed)
        } else if NON_ENCRYPT_PACKET_LIST.contains(route.as_str()) {
            None
        } else {
            Some(EncryptionKey::Default)
        }
    }

    /// Encode outbound protobuf payload for a route
    pub fn encode_packet(route_name: &str, protobuf_bytes: &[u8]) -> String {
        match Self::get_key_for_route(route_name) {
            Some(EncryptionKey::Fixed) => {
                // Base64( AES( Base64(proto) ) )
                let inner_b64 = general_purpose::STANDARD.encode(protobuf_bytes);
                Self::aes_encrypt_raw_b64(inner_b64.as_bytes(), ENCRYPT_FIX_KEY)
            }
            Some(EncryptionKey::Default) => {
                // Base64( AES(proto) )
                //Self::aes_encrypt_raw_b64(protobuf_bytes, DEFAULT_KEY)

                // Base64( AES( Base64(proto) ) )
                let inner_b64 = general_purpose::STANDARD.encode(protobuf_bytes);
                Self::aes_encrypt_raw_b64(inner_b64.as_bytes(), DEFAULT_KEY)
            }
            None => {
                // Base64(proto)
                general_purpose::STANDARD.encode(protobuf_bytes)
            }
        }
    }

    /// Decode inbound body → protobuf bytes
    pub fn decrypt_packet(route_name: &str, body_b64: &str) -> Result<Vec<u8>, ()> {
        match Self::get_key_for_route(route_name) {
            Some(EncryptionKey::Fixed) => {
                let decrypted = Self::aes_decrypt_raw(body_b64, ENCRYPT_FIX_KEY).map_err(|_| ())?;
                let inner_b64 = String::from_utf8(decrypted).map_err(|_| ())?;
                general_purpose::STANDARD.decode(inner_b64).map_err(|_| ())
            }
            Some(EncryptionKey::Default) => {
                //Self::aes_decrypt_raw(body_b64, DEFAULT_KEY)
                let decrypted = Self::aes_decrypt_raw(body_b64, DEFAULT_KEY).map_err(|_| ())?;
                let inner_b64 = String::from_utf8(decrypted).map_err(|_| ())?;
                general_purpose::STANDARD.decode(inner_b64).map_err(|_| ())
            }
            None => general_purpose::STANDARD.decode(body_b64).map_err(|_| ()),
        }
    }

    pub fn aes_encrypt_raw_b64(plaintext: &[u8], key_str: &str) -> String {
        let cipher = Aes256CbcEnc::new_from_slices(key_str.as_bytes(), &ZERO_IV).unwrap();
        let encrypted = cipher.encrypt_padded_vec_mut::<Pkcs7>(plaintext);
        general_purpose::STANDARD.encode(encrypted)
    }

    pub fn aes_decrypt_raw(cipher_b64: &str, key_str: &str) -> Result<Vec<u8>, ()> {
        let decoded = general_purpose::STANDARD
            .decode(cipher_b64)
            .map_err(|_| ())?;
        let cipher = Aes256CbcDec::new_from_slices(key_str.as_bytes(), &ZERO_IV).map_err(|_| ())?;
        cipher
            .decrypt_padded_vec_mut::<Pkcs7>(&decoded)
            .map_err(|_| ())
    }

    /// SHA-1 hash with static salt suffix
    pub fn sha1_hash(input: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.update(format!("{input}{SALT}"));
        format!("{:x}", hasher.finalize())
    }

    /// Millisecond-precision timestamp
    pub fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64
    }
}
