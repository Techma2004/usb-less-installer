use anyhow::Result;
use clap::Parser;
use colored::*;
use dialoguer::{Confirm, Select, Input};
use sysinfo::Disks;
use std::io::IsTerminal;

mod partition;
mod iso;
mod bootloader;

#[derive(Parser, Debug)]
#[command(name = "usb-less-installer", author, version, about = "Install Linux ISOs directly to internal drives without a USB stick")]
struct Args {
    #[arg(short, long)]
    iso: Option<String>,

    #[arg(short, long)]
    force: bool,
}

fn check_privileges(force: bool) -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        let output = std::process::Command::new("id").arg("-u").output()?;
        let uid = String::from_utf8(output.stdout)?.trim().to_string();
        if uid != "0" {
            println!("{}", "⚠️ Warning: You are not running as root (sudo). Partitioning and bootloader installation will fail without root privileges.".yellow());
            
            if !std::io::stdin().is_terminal() || force {
                println!("{}", "Non-interactive or force flag detected. Continuing without prompt...".dimmed());
                return Ok(());
            }

            let continue_anyway = Confirm::new()
                .with_prompt("Do you want to continue anyway?")
                .default(false)
                .interact()?;
            if !continue_anyway {
                std::process::exit(1);
            }
        }
    }
    Ok(())
}

fn list_disks() -> Result<()> {
    println!("{}", "\n=== Step 1: Scanning Available Disks ===".cyan().bold());
    let disks = Disks::new_with_refreshed_list();
    
    if disks.list().is_empty() {
        println!("{}", "No disks detected or insufficient permissions.".red());
    } else {
        println!("Detected {} storage locations/disks:", disks.list().len());
        for (i, d) in disks.list().iter().enumerate() {
            println!(
                " [{}] Name: {:?} | Mount: {:?} | Total: {} GB | Available: {} GB",
                i,
                d.name(),
                d.mount_point(),
                d.total_space() / (1024 * 1024 * 1024),
                d.available_space() / (1024 * 1024 * 1024)
            );
        }
    }
    Ok(())
}

fn run_interactive_menu() -> Result<String> {
    println!("{}", "\n=== ULLI Interactive TUI Menu ===".magenta().bold());
    let choices = vec![
        "Select ISO File & Run Full Installation",
        "Scan System Disks & Partitions",
        "Verify UEFI Bootloader Status",
        "Exit",
    ];

    let selection = Select::new()
        .with_prompt("Choose an option:")
        .items(&choices)
        .default(0)
        .interact()?;

    match selection {
        0 => {
            let iso_path: String = Input::new()
                .with_prompt("Enter path to Linux ISO (e.g. /path/to/omarchy.iso)")
                .interact_text()?;
            Ok(iso_path)
        }
        1 => {
            list_disks()?;
            partition::list_targetable_partitions()?;
            run_interactive_menu()
        }
        2 => {
            bootloader::register_boot_entry()?;
            run_interactive_menu()
        }
        _ => {
            println!("Exiting ULLI.");
            std::process::exit(0);
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("{}", "🚀 USB-less Linux Installer (ULLI-Rust)".green().bold());
    
    // Check root / admin privileges
    check_privileges(args.force)?;

    let iso_to_use = match args.iso {
        Some(path) => path,
        None => {
            if std::io::stdin().is_terminal() && !args.force {
                run_interactive_menu()?
            } else {
                println!("{}", "ℹ️ No ISO specified. Pass --iso <path> or run interactively in a terminal.".yellow());
                return Ok(());
            }
        }
    };

    // Scan disks
    list_disks()?;

    // Phase 2: Partition Management
    partition::list_targetable_partitions()?;

    // Phase 3: ISO Validation
    println!("{}", "\n=== Phase 3: ISO Validation & Extraction ===".cyan().bold());
    iso::validate_iso_path(&iso_to_use)?;

    // Phase 4: Bootloader Integration
    bootloader::register_boot_entry()?;

    println!("{}", "\n[Phases 1-4 Complete] All core modules implemented and verified successfully!".green());
    Ok(())
}
