# Research guide: cross-platform architecture and safety

This document is an architecture guide, not production-ready installation code. The current project intentionally performs read-only inspection only.

## Separate the host from the target

The host is the operating system running `usb-less-installer` (initial targets: Linux, Windows, macOS). The target is the operating system represented by the selected image. These are separate dimensions: running on Windows does not imply the target is Windows, and an ISO is not necessarily a Linux live image.

There is no universal procedure for installing arbitrary OS images. Different image families may use different partition layouts, boot methods, installers, signing requirements, and firmware assumptions. Use explicit backend adapters and publish a compatibility matrix instead of promising that every ISO works.

## Safe implementation sequence

1. **Inventory only:** collect storage metadata using native tools, parse it into a common read-only model, and clearly identify the source of each field.
2. **Image identification:** inspect file metadata and signatures; optionally verify a trusted checksum or signature supplied by the image publisher. File existence and size alone do not prove validity.
3. **Capability report:** detect host OS, firmware mode where available, privileges, available tools, and known limitations.
4. **Dry-run plan:** show exactly what a future backend intends to do, which disk/partition identifiers it would use, and what could be overwritten. A dry run must not write to storage or firmware.
5. **Virtual-machine testing:** test with disposable virtual disks and snapshots. Never start by experimenting on the user's only internal drive.
6. **Installation backend:** implement one specific target-image family at a time, with explicit preconditions, cancellation, error handling, verification, and a recovery strategy.

## Safety requirements before writes are enabled

- Never hard-code device paths such as `/dev/sda`, disk numbers, or EFI partition numbers.
- Resolve and re-check a target by stable identifiers immediately before any write.
- Refuse ambiguous targets, mounted/system disks that are not explicitly intended, and unsupported firmware modes.
- Never treat a `--force` flag as permission to skip critical safety checks.
- Do not format, resize, or rewrite partitions until the user has seen a complete plan and explicitly confirmed the exact target.
- Never assume that a kernel/initrd/squashfs layout is shared by every OS image.
- Never register boot entries unless the required EFI files have been installed and verified.
- Report failure accurately; never print a success message when an operation was only inspected or planned.

## Platform-specific tools

Native tools such as `lsblk`, PowerShell storage cmdlets, `diskutil`, `efibootmgr`, and `bcdedit` are useful for inspection. Their output and privileges vary by platform and version. Invoke commands with argument arrays, avoid interpolating untrusted input into shell scripts, check exit codes, and surface stderr to the user.

## GUI direction

The GUI should call the same core APIs as the CLI. It should display image validation, detected storage, compatibility limitations, and a dry-run summary. Installation controls must remain unavailable until the corresponding backend is implemented and tested. Keep platform-specific code behind small adapters so the core remains portable.
