# Omarchy 4.0.4 USB-less installation incident

This report records the user's previous Windows 10 Home attempt. It is evidence for compatibility research, not proof that any single component caused the failure.

## Goal and safety constraints

- Host: Windows 10 Home.
- Intended target: native Omarchy 4.0.4 installation on the internal 500 GB drive, replacing Windows only after backup verification and explicit approval.
- No USB drive was available.
- Windows and recovery partitions were to remain intact until the final approved installation.

## Observed history

1. ULLI failed twice with: `Failed to copy files! Exit code: 11`.
2. The report states ULLI supported a limited set of distributions and did not recognize Omarchy's Arch-based layout. Two empty staging volumes, `LINUX_LIVE (D:)` and `LINUX_LIVE (E:)`, were created and later deleted in Windows Disk Management.
3. The disk was reported to have 67 GB unallocated space afterwards; Windows, EFI, and recovery partitions remained intact.
4. Secure Boot and Legacy Support were reported disabled in firmware.
5. The user then tried Grub2Win with a loopback configuration to boot `omarchy-4.0.4.iso`.
6. The ISO exposed `/arch/boot/x86_64/vmlinuz-linux-t2` and `initramfs-linux-t2`. GRUB could access the ISO, but later boot attempts reported missing kernel/initrd-related files, including microcode and initramfs paths.
7. Windows continued to boot normally. No successful Omarchy installation was completed.

## Evidence versus hypothesis

Observed from the report: ULLI's copy step failed; GRUB loopback reached the ISO but failed to locate/load all required boot assets.

Not established: that NTFS alone caused the problem, that ULLI's exact parser logic was the only cause, or that every internal-disk loopback method is impossible. Plausible factors to investigate include unsupported image layout, GRUB loopback limitations, boot parameters, initramfs hooks, microcode paths, and how the live environment locates its root image.

## Engineering requirements derived from this incident

- Never infer compatibility solely from a distribution label.
- Inspect ISO contents and boot metadata read-only before preparing storage.
- Discover kernel, initramfs, microcode, and boot parameters from the image's own boot configuration rather than hardcoding guessed paths.
- Report exactly which boot files and parameters were found; flag unsupported or unresolved references.
- Treat "ISO file readable", "kernel loadable", "live environment bootable", and "installer works" as separate validation stages.
- Do not create or format staging partitions or modify Windows BCD/EFI entries until a backend has an explicit dry-run plan and recovery path.
- Test on disposable virtual disks/VMs first. The user's real internal drive must not be used for destructive testing.
- Preserve a verified backup and require explicit approval before any future Windows replacement operation.

## Next investigation

1. Reproduce metadata inspection of the exact Omarchy 4.0.4 ISO without writing to disk.
2. Compare its boot configuration with the GRUB loopback script and the missing-path errors.
3. Determine whether a supported chainload or dedicated boot entry is viable for this specific host/firmware/image combination.
4. Only after successful VM validation, design an opt-in Windows-host preparation backend.

This project remains a read-only prototype and cannot install Omarchy yet.
