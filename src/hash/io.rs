use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub const BUFFER_SIZE: usize = 2 * 1024 * 1024; // 2MB
pub const MMAP_THRESHOLD: u64 = 4 * 1024 * 1024 * 1024; // 4GB
pub const LARGE_FILE_THRESHOLD: u64 = 100 * 1024 * 1024; // 100MB
pub const LARGE_BUFFER_SIZE: usize = 4 * 1024 * 1024; // 4MB

// Helper function to read file in chunks and update a hasher that implements Digest
// Optimized for better performance with large buffers and efficient reading
// Uses memory-mapped I/O for small files and streaming for large files
#[allow(dead_code)]
pub fn hash_file_streaming<H>(
    path: &PathBuf,
    mut hasher: H,
    update_fn: fn(&mut H, &[u8]),
    finalize_fn: fn(H) -> String,
) -> Result<String, String> {
    // Try to use memory-mapped I/O for files that aren't too large
    let metadata = std::fs::metadata(path).map_err(|e| format!("Error: {}", e))?;
    let file_size = metadata.len();
    
    if file_size <= MMAP_THRESHOLD && file_size > 0 {
        // For small/medium files, use mmap for better performance
        let file = File::open(path).map_err(|e| format!("Error: {}", e))?;
        unsafe {
            match memmap2::MmapOptions::new().map(&file) {
                Ok(mmap) => {
                    // Process the entire file at once
                    update_fn(&mut hasher, &mmap);
                    return Ok(finalize_fn(hasher));
                }
                Err(_) => {
                    // If mmap fails, fall back to normal reading
                }
            }
        }
    }
    
    // For large files or if mmap fails, use streaming with optimized buffer
    let file = File::open(path).map_err(|e| format!("Error: {}", e))?;
    // Use larger buffer for large files
    let buffer_size = if file_size > LARGE_FILE_THRESHOLD {
        LARGE_BUFFER_SIZE
    } else {
        BUFFER_SIZE
    };
    let mut reader = BufReader::with_capacity(buffer_size, file);
    let mut buffer = vec![0u8; buffer_size];

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => {
                update_fn(&mut hasher, &buffer[..n]);
            }
            Err(e) => return Err(format!("Error reading file: {}", e)),
        }
    }

    Ok(finalize_fn(hasher))
}

// Helper function specific for MD5 using Context
// Optimized for better performance with memory-mapped I/O for small files
#[allow(dead_code)]
pub fn hash_md5_streaming(path: &PathBuf) -> Result<String, String> {
    let metadata = std::fs::metadata(path).map_err(|e| format!("Error: {}", e))?;
    let file_size = metadata.len();
    
    // Use memory-mapped I/O for files that aren't too large
    if file_size <= MMAP_THRESHOLD && file_size > 0 {
        let file = File::open(path).map_err(|e| format!("Error: {}", e))?;
        unsafe {
            match memmap2::MmapOptions::new().map(&file) {
                Ok(mmap) => {
                    let mut context = md5::Context::new();
                    context.consume(&mmap);
                    let digest = context.finalize();
                    return Ok(hex::encode(digest.as_slice()));
                }
                Err(_) => {
                    // If mmap fails, fall back to normal reading
                }
            }
        }
    }
    
    // For large files, use streaming with optimized buffer
    let file = File::open(path).map_err(|e| format!("Error: {}", e))?;
    let buffer_size = if file_size > LARGE_FILE_THRESHOLD {
        LARGE_BUFFER_SIZE
    } else {
        BUFFER_SIZE
    };
    let mut reader = BufReader::with_capacity(buffer_size, file);
    let mut buffer = vec![0u8; buffer_size];
    let mut context = md5::Context::new();

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => {
                context.consume(&buffer[..n]);
            }
            Err(e) => return Err(format!("Error reading file: {}", e)),
        }
    }

    let digest = context.finalize();
    Ok(hex::encode(digest.as_slice()))
}

