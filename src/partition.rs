use anyhow::{bail, Context, Result};
use std::process::Command;

fn run_and_print(program: &str, args: &[&str], description: &str) -> Result<()> {
    let output = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("Could not run {program}; is it installed and available on PATH?"))?;

    if !output.status.success() {
        bail!(
            "{description} failed ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    print!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

/// Display storage information only. This function never partitions or formats disks.
pub fn inspect_storage() -> Result<()> {
    println!("\n=== Storage inspection (read-only) ===");

    #[cfg(target_os = "linux")]
    {
        run_and_print(
            "lsblk",
            &["--bytes", "--json", "--output", "NAME,SIZE,TYPE,FSTYPE,LABEL,UUID,MOUNTPOINTS,RM,RO,MODEL"],
            "lsblk storage inspection",
        )?;
    }

    #[cfg(target_os = "windows")]
    {
        run_and_print(
            "powershell",
            &[
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                "Get-Disk | Select-Object Number,FriendlyName,BusType,PartitionStyle,OperationalStatus,IsBoot,IsSystem,IsReadOnly,Size | ConvertTo-Json -Depth 3",
            ],
            "PowerShell storage inspection",
        )?;
    }

    #[cfg(target_os = "macos")]
    {
        run_and_print("diskutil", &["list"], "diskutil storage inspection")?;
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        bail!("Storage inspection is not yet implemented for this host operating system.");
    }

    println!("\nNo storage changes were made.");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn supported_host_platforms_are_explicit() {
        assert!(cfg!(any(target_os = "linux", target_os = "windows", target_os = "macos")));
    }
}
