# Development roadmap

## Stage 1 — Safe inspection core (current PR)
- [x] Replace the misleading “installation complete” message with honest read-only behavior.
- [x] Basic image path and non-empty file checks.
- [x] Read-only storage inspection for Linux, Windows, and macOS.
- [x] Read-only boot configuration inspection for supported hosts.
- [x] Document that host platform and target OS image are separate concepts.
- [ ] Verify builds and tests on Linux, Windows, and macOS CI.
- [ ] Add structured storage models and unit tests for parsing platform output.
- [ ] Add image-format identification, checksums, and optional signature verification.

## Stage 2 — Architecture before installation
- [ ] Define a host capability model (firmware mode, privileges, available tools, filesystem support).
- [ ] Define target-image/installer adapters; do not assume every ISO is a Linux live image.
- [ ] Add a dry-run plan and machine-readable report format.
- [ ] Design a safety model, target identity checks, cancellation, logs, and recovery strategy.
- [ ] Test only in disposable virtual machines and virtual disks initially.

## Stage 3 — Carefully scoped installation backends
- [ ] Select one target OS/image family and document its supported combinations.
- [ ] Implement a non-destructive preparation plan before enabling writes.
- [ ] Add explicit, repeated confirmation for destructive operations.
- [ ] Verify boot files and recovery paths before changing firmware configuration.
- [ ] Never claim success until post-operation checks actually pass.

## Stage 4 — GUI
- [ ] Build a lightweight native GUI over the tested core APIs.
- [ ] Show host platform, image checks, detected devices, warnings, and dry-run plan.
- [ ] Keep all destructive actions disabled until a backend is implemented and tested.
