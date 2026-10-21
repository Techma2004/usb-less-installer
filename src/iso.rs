use anyhow::Result;
use std::path::Path;
use tracing::info;

pub fn validate_iso_path(iso_path: &str) -> Result<()> {
    let path = Path::new(iso_path);
    if !path.exists() {
        anyhow::bail!("Specified ISO file does not exist: {}", iso_path);
    }
    
    let metadata = std::fs::metadata(path)?;
    let size_mb = metadata.len() / (1024 * 1024);
    info!("Validated ISO file: {} (Size: {} MB)", iso_path, size_mb);
    println!("📦 Selected ISO: {} ({} MB)", iso_path, size_mb);
    
    Ok(())
}
