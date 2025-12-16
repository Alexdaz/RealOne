use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::time::Instant;
use iced_multi_window::WindowManager;
use crate::hash::{Algorithm, GostVariant, TigerVariant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HashFormat {
    HexUpper,
    HexLower,
    Base64,
}

impl HashFormat {
    pub fn all() -> Vec<HashFormat> {
        vec![
            HashFormat::HexUpper,
            HashFormat::HexLower,
            HashFormat::Base64,
        ]
    }
    
    pub fn to_string(&self) -> &'static str {
        match self {
            HashFormat::HexUpper => "Hexadecimal (Uppercase)",
            HashFormat::HexLower => "Hexadecimal (Lowercase)",
            HashFormat::Base64 => "Base64",
        }
    }
}

impl HashFormat {
    pub fn format_hash(&self, hash_bytes: &[u8]) -> String {
        match self {
            HashFormat::HexUpper => hex::encode(hash_bytes).to_uppercase(),
            HashFormat::HexLower => hex::encode(hash_bytes),
            HashFormat::Base64 => {
                use base64::Engine;
                base64::engine::general_purpose::STANDARD.encode(hash_bytes)
            }
        }
    }
    
    #[allow(dead_code)]
    pub fn parse_hash(&self, hash_str: &str) -> Result<Vec<u8>, String> {
        match self {
            HashFormat::HexUpper | HashFormat::HexLower => {
                hex::decode(hash_str.trim().replace(' ', ""))
                    .map_err(|e| format!("Error decoding hex: {}", e))
            }
            HashFormat::Base64 => {
                use base64::Engine;
                base64::engine::general_purpose::STANDARD.decode(hash_str.trim().replace(' ', ""))
                    .map_err(|e| format!("Error decoding base64: {}", e))
            }
        }
    }
}

impl fmt::Display for HashFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BrowseFile,
    FileSelected(Option<PathBuf>),
    CheckHashChanged(String),
    CheckButtonPressed,
    SettingsButtonPressed,
    AlgorithmToggled(Algorithm, bool),
    FormatChanged(HashFormat),
    GostVariantChanged(GostVariant),
    TigerVariantChanged(TigerVariant),
    #[allow(dead_code)]
    HashCalculated(Algorithm, String),
    HashesCalculated(Vec<(Algorithm, String)>), // Message for batch results
    #[allow(dead_code)]
    ProgressUpdate, // Update calculation progress
    CopyHash(Algorithm), // Copy hash to clipboard
    WindowClosed(iced::window::Id),
}

impl Default for RealOne {
    fn default() -> Self {
        Self {
            file_path: None,
            check_hash: String::new(),
            selected_algorithms: Vec::new(),
            hash_results: HashMap::new(),
            window_manager: WindowManager::default(),
            calculation_start: None,
            hash_format: HashFormat::HexLower,
            gost_variant: GostVariant::CryptoPro,
            tiger_variant: TigerVariant::Tiger,
            file_error: None,
        }
    }
}

pub struct RealOne {
    pub file_path: Option<PathBuf>,
    pub check_hash: String, // Hash that the user wants to compare
    pub selected_algorithms: Vec<Algorithm>,
    pub hash_results: HashMap<Algorithm, String>,
    pub window_manager: WindowManager<RealOne, iced::Theme, Message>,
    pub calculation_start: Option<Instant>, // Calculation start time
    pub hash_format: HashFormat, // Hash format
    pub gost_variant: GostVariant, // GOST S-box variant
    pub tiger_variant: TigerVariant, // TIGER padding variant
    pub file_error: Option<String>, // Error message for the file
}

