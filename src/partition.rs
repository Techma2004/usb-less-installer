use anyhow::{Context, Result};
use std::process::Command;
use tracing::info;

#[derive(Debug, Clone)]
pub struct PartitionTarget {
    pub device_path: String,
    pub size_gb: u64,
}

pub fn verify_partition_tools() -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        let which_parted = Command::new("which").arg("parted").output();
        if which_parted.is_err() || !which_parted.unwrap().status.success() {
            println!("⚠️ Warning: 'parted' tool not found. Partition modification will require it later.");
        } else {
            info!("Partitioning tools verified successfully.");
        }
    }
    Ok(())
}

pub fn list_targetable_partitions() -> Result<()> {
    println!("\n=== Phase 2: Partition & Space Management ===");
    verify_partition_tools()?;
    
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("lsblk")
            .args(["-o", "NAME,SIZE,FSTYPE,MOUNTPOINT,TYPE", "-J"])
            .output()
            .context("Failed to execute lsblk")?;
        
        let stdout = String::from_utf8(output.stdout)?;
        println!("Raw storage layout fetched successfully ({} bytes).", stdout.len());
    }

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("powershell")
            .args(["-Command", "Get-Partition | Select-Object DiskNumber, PartitionNumber, Size, DriveLetter"])
            .output()
            .context("Failed to query partitions via PowerShell")?;
        let stdout = String::from_utf8(output.stdout)?;
        print!("{}", stdout);
    }

    Ok(())
}
