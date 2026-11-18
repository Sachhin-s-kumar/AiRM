use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use zstd::stream::encode_all;

/// Serializes telemetry, hashes the original JSON,
/// compresses it, and stores it locally.
pub fn store_telemetry<T: serde::Serialize>(
    records: &[T],
    output_path: &Path,
) -> Result<String, String> {
    let json_data =
        serde_json::to_vec(records).map_err(|e| format!("JSON serialization failed: {e}"))?;

    let hash = Sha256::digest(&json_data);
    let fingerprint = format!("{:x}", hash);

    let compressed_data =
        encode_all(json_data.as_slice(), 3).map_err(|e| format!("Compression failed: {e}"))?;

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Directory creation failed: {e}"))?;
    }

    fs::write(output_path, compressed_data)
        .map_err(|e| format!("Telemetry storage failed: {e}"))?;

    Ok(fingerprint)
}
