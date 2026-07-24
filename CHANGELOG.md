# Changelog

Notable changes to this project should be documented in this file.
Make sure it is up to date before performing a release.

This file should follow the [Keep a Changelog](https://keepachangelog.com/en/2.0.0/) format where possible.
Descriptions for older versions are mostly copied from github releases notes.

The "title" of each release should be its first line.
A title is required for publishing a github release, so all versions should have one.

## Unreleased

### Changes
- The `maybe_const_fn!` macro has been moved to its own crate.
  For compatibility reasons, this macro is re-exported from the `rustversion-detect` crate (but still deprecated).
  - It is possible to disable this re-exporting and remove the dependency by turning off the default feature `compat-maybe-const-fn`.

### Removed
- Hide documentation for deprecated `maybe_const_fn!` macro.

## 0.2.0 - 2025-09-10
Move version detection to runtime.

Avoids the need to wait for compilation of a build script.

### Changes
- Rewrite API to focus on use on runtime detection used in build scripts.
- Caches the result of running rustc so that multiple build scripts will use the same result.

### Removed
- The `const` keyword has been removed from all functions.
  Their availability is version-dependent and so would require
  this crate to have its own build script, increasing build times.
- The `date!` macro has been removed.

### Deprecated
- The `maybe_const_fn!` macro has been deprecated.
  It belongs in a separate crate and is no longer used internally.

## 0.1.3 - 2024-09-05
Support Rust 1.31

### Changes
- Reduce the MSRV from 1.32 to 1.31.
- Use [cargo-rdme] to sync README.md with crate docs.
- Use github actions to run [lychee] link-checker on README.md

[cargo-rdme]: https://github.com/orium/cargo-rdme
[lychee]: https://github.com/lycheeverse/lychee

### Fixed
- Run tests on MSRV (previously only ran cargo check)

### Deprecated
- Deprecate `date!` macro, because it can't be implemented on Rust 1.31

## 0.1.2 - 2024-06-15
Deprecate `spec!` macro in favor of helper methods

Use the `RustVersion.is_since_major_version` and `RustVersion.is_since_patch_version` methods instead of `is_since(spec!(...))`.

This avoids a macro import, is simpler, faster, and allows const evaluation.

The macro will be removed in a future version.

## 0.1.1 - 2024-06-15
Fix compilation on MSRV

## 0.1.0 - 2024-06-16
Initial release.

Forked from [dtolnay/rustversion] ([v1.0.17][rustversion v1.0.17]) with all procedural macro code removed.

[dtolnay/rustversion]: https://github.com/dtolnay/rustversion
[rustversion v1.0.17]: https://github.com/dtolnay/rustversion/releases/tag/1.0.17
