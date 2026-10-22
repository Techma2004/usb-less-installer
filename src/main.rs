use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use dialoguer::Select;
use std::io::IsTerminal;

mod bootloader;
mod iso;
mod partition;

#[derive(Debug, Parser)]
#[command(
    name = "usb-less-installer",
    version,
    about = "Inspect OS installation images and storage before a USB-free install"
)]
struct Args {
    /// Inspect an OS image file without modifying it.
    #[arg(short, long, value_name = "PATH")]
    iso: Option<String>,

    /// Print read-only storage information.
    #[arg(long)]
    inspect_storage: bool,

    /// Inspect the current firmware boot configuration (read-only).
    #[arg(long)]
    inspect_boot: bool,
}

fn interactive_menu() -> Result<()> {
    loop {
        let choices = [
            "Validate an OS image file",
            "Inspect storage devices (read-only)",
            "Inspect firmware boot configuration (read-only)",
            "Exit",
        ];
        let selection = Select::new()
            .with_prompt("USB-less Installer — safe inspection tools")
            .items(choices)
            .default(0)
            .interact()?;

        match selection {
            0 => {
                let path: String = dialoguer::Input::new()
                    .with_prompt("Path to an OS image (ISO, IMG, or another image file)")
                    .interact_text()?;
                iso::inspect_image(&path)?;
            }
            1 => partition::inspect_storage()?,
            2 => bootloader::inspect_boot_configuration()?,
            _ => break,
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{}", "USB-less Installer".cyan().bold());
    println!("{}", "Read-only prototype: no partitions or boot settings will be changed.".yellow());

    let mut did_work = false;
    if let Some(path) = args.iso.as_deref() {
        iso::inspect_image(path)?;
        did_work = true;
    }
    if args.inspect_storage {
        partition::inspect_storage()?;
        did_work = true;
    }
    if args.inspect_boot {
        bootloader::inspect_boot_configuration()?;
        did_work = true;
    }

    if !did_work && std::io::stdin().is_terminal() {
        interactive_menu()?;
    } else if !did_work {
        println!("Nothing to do. Use --iso <PATH>, --inspect-storage, or --inspect-boot.");
    }

    Ok(())
}
