# USB-less Linux Installer (Rust) — Development Roadmap

## Phase 1: Core CLI & Safety Checks
- [ ] Root / Administrator privilege validation.
- [ ] Disk health & S.M.A.R.T. warning checks (to catch failing drives like Error 301 early).
- [ ] Interactive target disk selection and confirmation prompts.

## Phase 2: Partition & Space Management
- [ ] Enumerate available disks and partitions.
- [ ] Calculate unallocated space or shrink existing partitions safely.
- [ ] Create new target Linux partition (ext4/btrfs).

## Phase 3: ISO Parsing & Extraction
- [ ] Support local ISO file selection.
- [ ] Extract kernel (`vmlinuz`), initramfs (`initrd`), and root filesystem (`squashfs.img`).
- [ ] Copy extracted boot files to the target boot partition.

## Phase 4: Bootloader & UEFI Integration
- [ ] Configure GRUB or systemd-boot / rEFInd payload.
- [ ] Register UEFI boot entry (`efibootmgr` on Linux, `bcdboot` on Windows).
- [ ] Set default boot priority.

## Phase 5: Cleanup, Verification & Polish
- [ ] Clean up temporary staging files.
- [ ] Provide clear post-installation instructions.
- [ ] End-to-end testing and error handling.
