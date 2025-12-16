use digest::Digest;
use gost94::{Gost94CryptoPro, Gost94Test};
use tiger::{Tiger, Tiger2};

use crate::hash::algo::{GostVariant, TigerVariant};

pub enum TigerHasher {
    Tiger(Tiger),
    Tiger2(Tiger2),
}

pub enum GostHasher {
    Crypto(Gost94CryptoPro),
    Test(Gost94Test),
}

fn reverse_words8(bytes: &mut [u8]) {
    for chunk in bytes.chunks_mut(8) {
        chunk.reverse();
    }
}

impl TigerHasher {
    pub fn new(variant: TigerVariant) -> Self {
        match variant {
            TigerVariant::Tiger => TigerHasher::Tiger(Tiger::new()),
            TigerVariant::Tiger2 => TigerHasher::Tiger2(Tiger2::new()),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        match self {
            TigerHasher::Tiger(h) => h.update(data),
            TigerHasher::Tiger2(h) => h.update(data),
        }
    }
    
    pub fn finalize(self) -> Vec<u8> {
        let mut out = match self {
            TigerHasher::Tiger(h) => h.finalize().to_vec(),
            TigerHasher::Tiger2(h) => h.finalize().to_vec(),
        };
        
        reverse_words8(&mut out);
        out
    }
}

impl GostHasher {
    pub fn new(variant: GostVariant) -> Self {
        match variant {
            GostVariant::CryptoPro => GostHasher::Crypto(Gost94CryptoPro::new()),
            GostVariant::Test => GostHasher::Test(Gost94Test::new()),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        match self {
            GostHasher::Crypto(h) => h.update(data),
            GostHasher::Test(h) => h.update(data),
        }
    }
    
    pub fn finalize(self) -> Vec<u8> {
        match self {
            GostHasher::Crypto(h) => h.finalize().to_vec(),
            GostHasher::Test(h) => h.finalize().to_vec(),
        }
    }
}

