use anyhow::{bail, Context, Result};
use std::process::Command;

fn run_and_print(program: &str, args: &[&str], description: &str) -> Result<()> {
    let output = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("Could not run {program}; is it available on PATH?"))?;

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

/// Inspect the existing firmware boot configuration. Never writes boot entries.
pub fn inspect_boot_configuration() -> Result<()> {
    println!("\n=== Firmware boot configuration (read-only) ===");

    #[cfg(target_os = "linux")]
    {
        if !std::path::Path::new("/sys/firmware/efi").exists() {
            println!("This session does not appear to be booted in UEFI mode.");
            println!("No boot configuration was changed.");
            return Ok(());
        }
        run_and_print("efibootmgr", &["-v"], "UEFI boot-entry inspection")?;
    }

    #[cfg(target_os = "windows")]
    {
        run_and_print(
            "bcdedit",
            &["/enum", "firmware"],
            "Windows firmware boot-entry inspection",
        )?;
    }

    #[cfg(target_os = "macos")]
    {
        run_and_print("bless", &["--getBoot"], "macOS boot target inspection")?;
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        bail!("Boot configuration inspection is not yet implemented for this host operating system.");
    }

    println!("\nNo boot settings were changed.");
    Ok(())
}
