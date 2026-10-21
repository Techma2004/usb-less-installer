# USB-less Installer (ULLI-Rust)

A fast, reliable, cross-platform tool written in Rust to install Linux distributions (like Omarchy, Ubuntu, Linux Mint, Fedora, etc.) directly to internal hard drives without needing a physical USB flash drive or manual BIOS/UEFI configuration headaches.

---

## Features

- **Zero USB Required:** Bootstrap and install Linux directly from a running environment (Windows or Linux).
- **Safety Checks:** Root/Administrator privilege verification and disk health safety warnings.
- **Partition Management:** Cross-platform storage inspection (`lsblk` on Linux, PowerShell/`diskpart` on Windows).
- **ISO Validation:** Verifies ISO integrity, size, and boot artifacts.
- **Bootloader Integration:** Configures UEFI boot entries automatically (`efibootmgr` on Linux / `bcdboot` on Windows).

---

## Prerequisites

- **Linux:** `sudo` privileges, `lsblk`, and `efibootmgr`.
- **Windows:** Administrator privileges, PowerShell, and UEFI firmware with Secure Boot settings configured as needed.
- **Rust:** Edition 2024 (Cargo).

---

## Building from Source

Clone the repository and build in release mode for maximum performance:

```bash
git clone https://github.com/Techma2004/usb-less-installer.git
cd usb-less-installer
cargo build --release
```

---

## Usage

Run the installer with interactive menu prompts:

```bash
sudo ./target/release/usb-less-installer
```

Or pass arguments directly:

```bash
sudo ./target/release/usb-less-installer --iso /path/to/omarchy.iso
```

### CLI Options

- `-i, --iso <PATH>`: Path to the Linux ISO file to install.
- `-f, --force`: Skip non-interactive safety checks (useful for automated scripts).

---

## License

Released under the **GNU General Public License v3.0**.
