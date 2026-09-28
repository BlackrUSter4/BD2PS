use once_cell::sync::Lazy;
use std::path::PathBuf;

pub mod error_type;
pub mod packet_code;

pub const GAME_SERVER_ADDRESS: &str = "0.0.0.0";
pub const GAME_SERVER_PORT: u16 = 10800;

pub static CERT_FILE_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../certs/cert.pem"));

pub static KEY_FILE_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../certs/key.pem"));

pub static DATABASE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
        .join("db")
});

pub fn init_tracing() {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().unwrap();

    tracing_subscriber::fmt().init();
}
