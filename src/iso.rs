use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

/// Perform basic, read-only checks on an image file.
/// This does not prove that the image is bootable or safe to install.
pub fn inspect_image(image_path: &str) -> Result<()> {
    let path = Path::new(image_path);

    if !path.exists() {
        bail!("Image file does not exist: {}", path.display());
    }
    if !path.is_file() {
        bail!("Image path is not a regular file: {}", path.display());
    }

    let metadata = fs::metadata(path)
        .with_context(|| format!("Could not read metadata for {}", path.display()))?;
    if metadata.len() == 0 {
        bail!("Image file is empty: {}", path.display());
    }

    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("(no extension)");

    println!("Image file: {}", path.display());
    println!("Size: {} bytes ({:.2} MiB)", metadata.len(), metadata.len() as f64 / 1_048_576.0);
    println!("Extension: {extension}");
    println!("Basic file checks passed.");
    println!("Note: format, checksum, signature, and bootability have not been verified.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::inspect_image;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn rejects_missing_file() {
        let path = std::env::temp_dir().join(format!(
            "usb-less-installer-missing-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        assert!(inspect_image(path.to_str().unwrap()).is_err());
    }

    #[test]
    fn accepts_nonempty_regular_file_without_assuming_iso_extension() {
        let path = std::env::temp_dir().join(format!(
            "usb-less-installer-test-{}.img",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::write(&path, b"test image").unwrap();
        assert!(inspect_image(path.to_str().unwrap()).is_ok());
        fs::remove_file(path).unwrap();
    }
}
