# Ventoy upstream source and integration notes

This repository tracks Ventoy as a Git submodule at `vendor/ventoy`, pinned to a specific upstream commit. The upstream project is maintained at https://github.com/ventoy/Ventoy.

## Why a submodule

Ventoy is a substantial bootloader/tooling project with its own build system and third-party components. Keeping it as a pinned upstream submodule preserves its source and history, makes the upstream revision explicit, and avoids copying files while accidentally dropping copyright or licence notices. It is **not yet integrated into the Rust executable**.

To fetch the source after cloning this repository:

```sh
git submodule update --init --recursive
```

To inspect the exact pinned revision:

```sh
git -C vendor/ventoy rev-parse HEAD
```

## Licence obligations

Ventoy's repository identifies GPL-3.0 as its licence. Preserve the upstream copyright notices and `COPYING` file, review its bundled third-party components and their licences before packaging or redistributing binaries, and clearly mark any modifications to upstream source. This project is also marked GPL-3.0-only. This note is engineering documentation, not legal advice.

## Integration boundaries

- The submodule is research/source only; the Rust CLI does not invoke Ventoy yet.
- Ventoy's ability to boot images from local disks does not guarantee that every image or host firmware combination works.
- First supported host release target is Windows x86-64, matching the original Windows 10 Home test environment; Linux and macOS builds are also produced for the read-only prototype.
- Do not enable partition, formatting, EFI/BCD, or other boot configuration writes until the exact operation has a dry-run plan, explicit target revalidation, cancellation/recovery behavior, and disposable-VM tests.
- Keep the release marked prerelease until the boot workflow is implemented and tested. These binaries are inspection prototypes, not installers.
