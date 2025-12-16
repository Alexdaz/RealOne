pub mod algo;
pub mod io;
pub mod wrappers;
pub mod compute;

pub use algo::{Algorithm, GostVariant, TigerVariant};
pub use compute::calculate_hashes_parallel_streaming;

