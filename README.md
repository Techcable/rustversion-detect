# rustversion-detect
<!-- cargo-rdme start -->

Parses the output of `rustc --version` for use in build scripts.

The parsed output is cached in a static variable,
so `rustc` will only be invoked once no matter how many build scripts use this crate.
This gives an advantage over using [`autocfg` crate][`autocfg`] or performing ad-hoc detection,
as these require re-running `rustc` for each build scripts.

This crate originated as a fork of [dtolnay's `rustversion` crate][`rustversion`],
but with the proc-macro code removed and version detection moved from the build script to runtime.
The core version detection logic has been kept up to date with upstream changes.
It currently mirrors rustversion v1.0.23.

Moving the version detection logic to runtime means that this crate
does not need its own build script,
reducing compile times compared to using the [`rustversion`] macros in a build script.

[`rustversion`]: https://github.com/dtolnay/rustversion
[`autocfg`]: https://github.com/cuviper/autocfg

## Dependency
Add the following to your build script:
```toml
[build-dependencies]
rustversion-detect = "0.3"
```

## Examples
```rust
pub fn main() {
    // by default rust re-runs the build script if any source file changes
    // this directive indicates that the build script only needs
    // to be rerun if the compiler flags change (or if the build script itself changes)
    println!("cargo:rerun-if-changed=build.rs");

    // Rust 1.80 requires listing all possibilities using this directive
    println!("cargo:rustc-check-cfg=cfg(use_nightly)");

    let version = rustversion_detect::detect_version().unwrap();
    if version.is_nightly() {
        println!("cargo:rustc-cfg=use_nightly");
    }
}
```

<!-- cargo-rdme end -->

## License
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
