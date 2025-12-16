use std::fs;
use std::path::PathBuf;
use crate::hash::{Algorithm, GostVariant, TigerVariant};
use crate::state::HashFormat;

#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
    selected_algorithms: Vec<Algorithm>,
    hash_format: Option<HashFormat>, // Optional for compatibility with old configs
    gost_variant: Option<GostVariant>, // Optional for compatibility with old configs
    tiger_variant: Option<TigerVariant>, // Optional for compatibility with old configs
}

fn config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("RealOne");
    path.push("config.json");
    path
}

pub fn load_config() -> (Vec<Algorithm>, HashFormat, GostVariant, TigerVariant) {
    let path = config_path();
    
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(config) = serde_json::from_str::<Config>(&content) {
            let format = config.hash_format.unwrap_or(HashFormat::HexLower);
            let gost_variant = config.gost_variant.unwrap_or(GostVariant::CryptoPro);
            let tiger_variant = config.tiger_variant.unwrap_or(TigerVariant::Tiger);
            return (config.selected_algorithms, format, gost_variant, tiger_variant);
        }
    }
    
    // Default values if there's no configuration
    (
        vec![
            Algorithm::MD5,
            Algorithm::SHA256,
            Algorithm::SHA512,
        ],
        HashFormat::HexLower,
        GostVariant::CryptoPro,
        TigerVariant::Tiger,
    )
}

pub fn save_config(algorithms: &[Algorithm], format: HashFormat, gost_variant: GostVariant, tiger_variant: TigerVariant) -> Result<(), Box<dyn std::error::Error>> {
    let path = config_path();
    
    // Create directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let config = Config {
        selected_algorithms: algorithms.to_vec(),
        hash_format: Some(format),
        gost_variant: Some(gost_variant),
        tiger_variant: Some(tiger_variant),
    };
    
    let content = serde_json::to_string_pretty(&config)?;
    fs::write(&path, content)?;
    
    Ok(())
}

