# USB-less Installer

A Rust project exploring how to start operating-system installations from an existing computer without a USB stick. The goal is to support multiple host platforms and different OS installation images—not just Linux distributions.

**Current status: safe, read-only prototype.** This version inspects image files, storage devices, and existing firmware boot configuration. It does not partition, format, extract, install, or change boot settings.

## Cross-platform direction

The project separates the **host platform** (where the program runs) from the **target image** (the OS the user wants to install).

- **Host platforms:** Linux, Windows, and macOS builds are being prepared incrementally.
- **Target images:** different operating-system installer images, not only Linux ISOs. The current image check accepts any non-empty regular file; it does not yet identify or verify image formats.
- **Important limitation:** there is no single universal way to install every OS image. Future support will use image/installer-specific backends, with explicit host capabilities and safety checks. A successful file check does not mean an image is bootable or installable.

## Ventoy source

Ventoy is tracked as a pinned upstream Git submodule under `vendor/ventoy`. It is research/source only at this stage; the Rust executable does **not** invoke Ventoy yet.

After cloning, fetch the upstream source with:

```sh
git submodule update --init --recursive
```

See [VENTOY_INTEGRATION.md](VENTOY_INTEGRATION.md) for integration boundaries and licence notes.

## Current features

- Basic, read-only image file checks.
- Read-only storage inspection using `lsblk` on Linux, PowerShell on Windows, and `diskutil` on macOS.
- Read-only firmware boot configuration inspection using `efibootmgr` on UEFI-booted Linux, `bcdedit` on Windows, and `bless` on macOS.
- Interactive CLI menu and command-line options.
- No destructive disk operations in this prototype.

## Requirements

- Rust toolchain with Cargo (Rust edition 2024).
- Linux: `lsblk`; `efibootmgr` is needed to inspect UEFI entries when booted in UEFI mode.
- Windows: PowerShell and `bcdedit`.
- macOS: `diskutil` and `bless`.

The inspection commands do not normally require administrator/root privileges, though host tools and system policy can affect the output.

## Build

```bash
git clone --recurse-submodules https://github.com/Techma2004/usb-less-installer.git
cd usb-less-installer
cargo build --release
cargo test
```

## Usage

Interactive menu:

```bash
./target/release/usb-less-installer
```

Check an image file (read-only):

```bash
./target/release/usb-less-installer --iso /path/to/installer.iso
```

Inspect storage:

```bash
./target/release/usb-less-installer --inspect-storage
```

Inspect firmware boot configuration:

```bash
./target/release/usb-less-installer --inspect-boot
```

Use the actual path to your image. `--iso` is retained as a familiar option name, but the check is not restricted to `.iso` extensions.

## Releases

The release workflow is configured to build prototype binaries for Linux x86-64, Windows x86-64, macOS x86-64, and macOS ARM64 when a `v*` tag is pushed. Releases are marked as **pre-releases** because this project is not yet an OS installer. Do not treat these binaries as capable of installing an operating system.

## Safety roadmap

Before any disk-writing feature is considered, the project needs a threat model, target-device identity checks, explicit destructive-operation confirmation, dry-run plans, robust error handling, recovery strategy, and VM-based tests. A future GUI will call the same tested core APIs rather than implement storage logic separately.

**Do not use this prototype to install an OS yet.** It is not an installer and cannot safely prepare a bootable installation.

## License

GNU General Public License v3.0 (GPL-3.0-only). Ventoy source is separately maintained upstream and included as a pinned submodule; see its own licence and third-party notices in `vendor/ventoy`.
