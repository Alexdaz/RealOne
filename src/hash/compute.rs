use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use digest::Digest;
use whirlpool::Whirlpool;

use crate::hash::algo::{Algorithm, GostVariant, TigerVariant};
use crate::hash::io::{BUFFER_SIZE, LARGE_BUFFER_SIZE, LARGE_FILE_THRESHOLD, MMAP_THRESHOLD};
use crate::hash::wrappers::{GostHasher, TigerHasher};

pub fn calculate_hash_from_data(
    data: &[u8],
    algorithm: &Algorithm,
    gost_variant: GostVariant,
    tiger_variant: TigerVariant,
) -> String {
    match algorithm {
        Algorithm::MD5 => {
            let mut context = md5::Context::new();
            context.consume(data);
            let digest = context.finalize();
            hex::encode(digest.as_slice())
        }
        Algorithm::SHA1 => {
            use sha1::{Sha1, Digest};
            let mut hasher = Sha1::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::SHA256 => {
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::SHA384 => {
            use sha2::{Sha384, Digest};
            let mut hasher = Sha384::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::SHA512 => {
            use sha2::{Sha512, Digest};
            let mut hasher = Sha512::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::SHA3_224 => {
            use sha3::{Sha3_224, Digest};
            let mut hasher = Sha3_224::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::SHA3_256 => {
            use sha3::{Sha3_256, Digest};
            let mut hasher = Sha3_256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::SHA3_384 => {
            use sha3::{Sha3_384, Digest};
            let mut hasher = Sha3_384::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::SHA3_512 => {
            use sha3::{Sha3_512, Digest};
            let mut hasher = Sha3_512::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::BLAKE2b => {
            use blake2::{Blake2b512, Digest};
            let mut hasher = Blake2b512::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::BLAKE2s => {
            use blake2::{Blake2s256, Digest};
            let mut hasher = Blake2s256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::RIPEMD160 => {
            use ripemd::{Ripemd160, Digest};
            let mut hasher = Ripemd160::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::CRC32 => {
            let mut hasher = crc32fast::Hasher::new();
            hasher.update(data);
            format!("{:08x}", hasher.finalize())
        }
        Algorithm::ADLER32 => {
            let mut adler = adler32::RollingAdler32::new();
            for &byte in data {
                adler.update(byte);
            }
            format!("{:08x}", adler.hash())
        }
        Algorithm::MD4 => {
            let mut hasher = md4::Md4::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::TIGER192 => {
            let mut hasher = TigerHasher::new(tiger_variant);
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::WHIRLPOOL => {
            let mut hasher = Whirlpool::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::GOST => {
            let mut hasher = GostHasher::new(gost_variant);
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
    }
}

// Optimized function that processes multiple algorithms in a single file pass
// Reads the file once using streaming and updates all hashers
pub fn calculate_hashes_parallel_streaming(
    path: &PathBuf,
    algorithms: &[Algorithm],
    gost_variant: GostVariant,
    tiger_variant: TigerVariant,
) -> Vec<(Algorithm, String)> {
    use std::io::Read;
    use sha1::Digest;
    
    // Clone algorithms at the beginning to avoid ownership issues
    let algorithms_clone = algorithms.to_vec();
    
    let metadata = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(e) => return algorithms_clone.iter().map(|alg| (alg.clone(), format!("Error: {}", e))).collect(),
    };
    let file_size = metadata.len();
    
    // OPTIMIZED STRATEGY: Try mmap first (more efficient)
    // If mmap fails or is not available, load into memory and process in parallel
    // This is faster than sequential streaming
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return algorithms_clone.iter().map(|alg| (alg.clone(), format!("Error: {}", e))).collect(),
    };
    
    // Try to use mmap (works for large files too)
    let mmap_result = unsafe {
        memmap2::MmapOptions::new().map(&file)
    };
    
    match mmap_result {
        Ok(mmap) => {
            // Use mmap directly - the OS handles memory efficiently
            let mmap_arc = std::sync::Arc::new(mmap);
            
            // Process all algorithms in parallel on the shared mmap
            let mut handles = Vec::new();
            
            for algorithm in &algorithms_clone {
                let alg = algorithm.clone();
                let mmap_clone = mmap_arc.clone();
                let alg_clone = alg.clone();
                let gost_variant_clone = gost_variant;
                let tiger_variant_clone = tiger_variant;
                let handle = std::thread::spawn(move || {
                    let data: &[u8] = &*mmap_clone;
                    (alg_clone.clone(), calculate_hash_from_data(data, &alg_clone, gost_variant_clone, tiger_variant_clone))
                });
                handles.push(handle);
            }
            
            let mut results = Vec::new();
            for handle in handles {
                match handle.join() {
                    Ok(result) => results.push(result),
                    Err(_) => {}
                }
            }
            return results;
        }
        Err(_) => {
            // If mmap fails, load the entire file into memory and process in parallel
            // This is faster than sequential streaming for reasonable files
            if file_size <= MMAP_THRESHOLD {
                let file2 = match File::open(path) {
                    Ok(f) => f,
                    Err(e) => return algorithms_clone.iter().map(|alg| (alg.clone(), format!("Error: {}", e))).collect(),
                };
                let mut data = Vec::with_capacity(file_size as usize);
                if let Ok(_) = std::io::Read::read_to_end(&mut std::io::BufReader::new(file2), &mut data) {
                    let data_arc = std::sync::Arc::new(data);
                    
                    // Process all algorithms in parallel
                    let mut handles = Vec::new();
                    for algorithm in &algorithms_clone {
                        let alg = algorithm.clone();
                        let data_clone = data_arc.clone();
                        let alg_clone = alg.clone();
                        let gost_variant_clone = gost_variant;
                        let tiger_variant_clone = tiger_variant;
                        let handle = std::thread::spawn(move || {
                            (alg_clone.clone(), calculate_hash_from_data(&data_clone, &alg_clone, gost_variant_clone, tiger_variant_clone))
                        });
                        handles.push(handle);
                    }
                    
                    let mut results = Vec::new();
                    for handle in handles {
                        match handle.join() {
                            Ok(result) => results.push(result),
                            Err(_) => {}
                        }
                    }
                    return results;
                }
            }
            // If everything fails, fall back to streaming (last resort)
        }
    }
    
    // For large files, use streaming: read once and update all hashers
    // This avoids loading everything into memory
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return algorithms_clone.iter().map(|alg| (alg.clone(), format!("Error: {}", e))).collect(),
    };
    
    let buffer_size = if file_size > LARGE_FILE_THRESHOLD {
        LARGE_BUFFER_SIZE
    } else {
        BUFFER_SIZE
    };
    
    let mut reader = BufReader::with_capacity(buffer_size, file);
    let mut buffer = vec![0u8; buffer_size];
    
    // Initialize all hashers (only those needed)
    let has_md4 = algorithms_clone.contains(&Algorithm::MD4);
    let has_md5 = algorithms_clone.contains(&Algorithm::MD5);
    let has_sha1 = algorithms_clone.contains(&Algorithm::SHA1);
    let has_sha256 = algorithms_clone.contains(&Algorithm::SHA256);
    let has_sha384 = algorithms_clone.contains(&Algorithm::SHA384);
    let has_sha512 = algorithms_clone.contains(&Algorithm::SHA512);
    let has_sha3_224 = algorithms_clone.contains(&Algorithm::SHA3_224);
    let has_sha3_256 = algorithms_clone.contains(&Algorithm::SHA3_256);
    let has_sha3_384 = algorithms_clone.contains(&Algorithm::SHA3_384);
    let has_sha3_512 = algorithms_clone.contains(&Algorithm::SHA3_512);
    let has_blake2b = algorithms_clone.contains(&Algorithm::BLAKE2b);
    let has_blake2s = algorithms_clone.contains(&Algorithm::BLAKE2s);
    let has_ripemd160 = algorithms_clone.contains(&Algorithm::RIPEMD160);
    let has_tiger192 = algorithms_clone.contains(&Algorithm::TIGER192);
    let has_whirlpool = algorithms_clone.contains(&Algorithm::WHIRLPOOL);
    let has_gost = algorithms_clone.contains(&Algorithm::GOST);
    let has_crc32 = algorithms_clone.contains(&Algorithm::CRC32);
    let has_adler32 = algorithms_clone.contains(&Algorithm::ADLER32);
    
    let mut md4_hasher = if has_md4 { Some(md4::Md4::new()) } else { None };
    let mut md5_ctx = if has_md5 { Some(md5::Context::new()) } else { None };
    let mut sha1_hasher = if has_sha1 { Some(sha1::Sha1::new()) } else { None };
    let mut sha256_hasher = if has_sha256 { Some(sha2::Sha256::new()) } else { None };
    let mut sha384_hasher = if has_sha384 { Some(sha2::Sha384::new()) } else { None };
    let mut sha512_hasher = if has_sha512 { Some(sha2::Sha512::new()) } else { None };
    let mut sha3_224_hasher = if has_sha3_224 { Some(sha3::Sha3_224::new()) } else { None };
    let mut sha3_256_hasher = if has_sha3_256 { Some(sha3::Sha3_256::new()) } else { None };
    let mut sha3_384_hasher = if has_sha3_384 { Some(sha3::Sha3_384::new()) } else { None };
    let mut sha3_512_hasher = if has_sha3_512 { Some(sha3::Sha3_512::new()) } else { None };
    let mut blake2b_hasher = if has_blake2b { Some(blake2::Blake2b512::new()) } else { None };
    let mut blake2s_hasher = if has_blake2s { Some(blake2::Blake2s256::new()) } else { None };
    let mut ripemd160_hasher = if has_ripemd160 { Some(ripemd::Ripemd160::new()) } else { None };
    let mut tiger192_hasher = if has_tiger192 { Some(TigerHasher::new(tiger_variant)) } else { None };
    let mut whirlpool_hasher = if has_whirlpool { Some(Whirlpool::new()) } else { None };
    let mut gost_hasher = if has_gost { Some(GostHasher::new(gost_variant)) } else { None };
    let mut crc32_hasher = if has_crc32 { Some(crc32fast::Hasher::new()) } else { None };
    let mut adler32_hasher = if has_adler32 { Some(adler32::RollingAdler32::new()) } else { None };
    
    // Read the file once and update all active hashers
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let chunk = &buffer[..n];
                if let Some(ref mut h) = md4_hasher { h.update(chunk); }
                if let Some(ref mut ctx) = md5_ctx { ctx.consume(chunk); }
                if let Some(ref mut h) = sha1_hasher { h.update(chunk); }
                if let Some(ref mut h) = sha256_hasher { h.update(chunk); }
                if let Some(ref mut h) = sha384_hasher { h.update(chunk); }
                if let Some(ref mut h) = sha512_hasher { h.update(chunk); }
                if let Some(ref mut h) = sha3_224_hasher { h.update(chunk); }
                if let Some(ref mut h) = sha3_256_hasher { h.update(chunk); }
                if let Some(ref mut h) = sha3_384_hasher { h.update(chunk); }
                if let Some(ref mut h) = sha3_512_hasher { h.update(chunk); }
                if let Some(ref mut h) = blake2b_hasher { h.update(chunk); }
                if let Some(ref mut h) = blake2s_hasher { h.update(chunk); }
                if let Some(ref mut h) = ripemd160_hasher { h.update(chunk); }
                if let Some(ref mut h) = tiger192_hasher { h.update(chunk); }
                if let Some(ref mut h) = whirlpool_hasher { h.update(chunk); }
                if let Some(ref mut h) = gost_hasher { h.update(chunk); }
                if let Some(ref mut h) = crc32_hasher { h.update(chunk); }
                if let Some(ref mut h) = adler32_hasher {
                    for &byte in chunk {
                        h.update(byte);
                    }
                }
            }
            Err(e) => return algorithms_clone.iter().map(|alg| (alg.clone(), format!("Error reading file: {}", e))).collect(),
        }
    }
    
    // Finalize all hashes
    let mut results = Vec::new();
    for alg in &algorithms_clone {
        let hash = match alg {
            Algorithm::MD4 => {
                if let Some(h) = md4_hasher.take() {
                    hex::encode(h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::MD5 => {
                if let Some(ctx) = md5_ctx.take() {
                    let digest = ctx.finalize();
                    hex::encode(digest.as_slice())
                } else {
                    continue;
                }
            }
            Algorithm::SHA1 => {
                if let Some(h) = sha1_hasher.take() {
                    hex::encode(h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::SHA256 => {
                if let Some(h) = sha256_hasher.take() {
                    hex::encode(h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::SHA384 => {
                if let Some(h) = sha384_hasher.take() {
                    hex::encode(h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::SHA512 => {
                if let Some(h) = sha512_hasher.take() {
                    hex::encode(h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::SHA3_224 => {
                if let Some(h) = sha3_224_hasher.take() {
                    hex::encode(h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::SHA3_256 => {
                if let Some(h) = sha3_256_hasher.take() {
                    hex::encode(h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::SHA3_384 => {
                if let Some(h) = sha3_384_hasher.take() {
                    hex::encode(h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::SHA3_512 => {
                if let Some(h) = sha3_512_hasher.take() {
                    hex::encode(h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::BLAKE2b => {
                if let Some(h) = blake2b_hasher.take() {
                    hex::encode(h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::BLAKE2s => {
                if let Some(h) = blake2s_hasher.take() {
                    hex::encode(h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::RIPEMD160 => {
                if let Some(h) = ripemd160_hasher.take() {
                    hex::encode(h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::TIGER192 => {
                if let Some(h) = tiger192_hasher.take() {
                    hex::encode(h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::WHIRLPOOL => {
                if let Some(h) = whirlpool_hasher.take() {
                    hex::encode(h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::GOST => {
                if let Some(h) = gost_hasher.take() {
                    hex::encode(h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::CRC32 => {
                if let Some(h) = crc32_hasher.take() {
                    format!("{:08x}", h.finalize())
                } else {
                    continue;
                }
            }
            Algorithm::ADLER32 => {
                if let Some(h) = adler32_hasher.take() {
                    format!("{:08x}", h.hash())
                } else {
                    continue;
                }
            }
        };
        results.push((alg.clone(), hash));
    }
    
    results
}

