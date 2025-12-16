use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GostVariant {
    CryptoPro,
    Test,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TigerVariant {
    Tiger,
    Tiger2,
}

impl GostVariant {
    pub fn all() -> Vec<GostVariant> {
        vec![GostVariant::CryptoPro, GostVariant::Test]
    }
    
    pub fn to_string(&self) -> &'static str {
        match self {
            GostVariant::CryptoPro => "GOST R 34.11-94 (CryptoPro S-box)",
            GostVariant::Test => "GOST R 34.11-94 (Test S-box)",
        }
    }
}

impl fmt::Display for GostVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl TigerVariant {
    pub fn all() -> Vec<TigerVariant> {
        vec![TigerVariant::Tiger, TigerVariant::Tiger2]
    }
    
    pub fn to_string(&self) -> &'static str {
        match self {
            TigerVariant::Tiger => "Tiger (original padding)",
            TigerVariant::Tiger2 => "Tiger2 (alternate padding)",
        }
    }
}

impl fmt::Display for TigerVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Algorithm {
    MD4,
    MD5,
    SHA1,
    SHA256,
    SHA384,
    SHA512,
    SHA3_224,
    SHA3_256,
    SHA3_384,
    SHA3_512,
    BLAKE2b,
    BLAKE2s,
    RIPEMD160,
    TIGER192,
    WHIRLPOOL,
    GOST,
    CRC32,
    ADLER32,
}

impl Algorithm {
    pub fn all() -> Vec<Algorithm> {
        vec![
            Algorithm::MD4,
            Algorithm::MD5,
            Algorithm::SHA1,
            Algorithm::SHA256,
            Algorithm::SHA384,
            Algorithm::SHA512,
            Algorithm::SHA3_224,
            Algorithm::SHA3_256,
            Algorithm::SHA3_384,
            Algorithm::SHA3_512,
            Algorithm::BLAKE2b,
            Algorithm::BLAKE2s,
            Algorithm::RIPEMD160,
            Algorithm::TIGER192,
            Algorithm::WHIRLPOOL,
            Algorithm::GOST,
            Algorithm::CRC32,
            Algorithm::ADLER32,
        ]
    }

    pub fn to_string(&self) -> String {
        match self {
            Algorithm::MD4 => "MD4",
            Algorithm::MD5 => "MD5",
            Algorithm::SHA1 => "SHA1",
            Algorithm::SHA256 => "SHA256",
            Algorithm::SHA384 => "SHA384",
            Algorithm::SHA512 => "SHA512",
            Algorithm::SHA3_224 => "SHA3-224",
            Algorithm::SHA3_256 => "SHA3-256",
            Algorithm::SHA3_384 => "SHA3-384",
            Algorithm::SHA3_512 => "SHA3-512",
            Algorithm::BLAKE2b => "BLAKE2b",
            Algorithm::BLAKE2s => "BLAKE2s",
            Algorithm::RIPEMD160 => "RIPEMD160",
            Algorithm::TIGER192 => "TIGER192",
            Algorithm::WHIRLPOOL => "WHIRLPOOL",
            Algorithm::GOST => "GOST",
            Algorithm::CRC32 => "CRC32",
            Algorithm::ADLER32 => "ADLER32",
        }
        .to_string()
    }
}

