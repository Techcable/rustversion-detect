//! This crate provides a simple API for detecting the rustc
//! compiler version.
//!
//! It is primarily intended for build scripts, but is also usable at runtime.
//!
//! The implementation is forked from the [`rustversion` crate], but with proc-macro code removed.
//!
//! [`rustversion` crate]: https://github.com/dtolnay/rustversion
//!
//! # Dependency
//! Add the following to your build script:
//! ```toml
//! [build-dependencies]
//! rustversion-detect = "0.1"
//! ```
#![no_std]
#![deny(missing_docs)]

#[macro_use]
mod macros;
pub mod date;
pub mod version;

pub use crate::date::Date;
pub use crate::version::{Channel, RustVersion, StableVersionSpec};

/// The detected rust compiler version.
pub const RUST_VERSION: RustVersion = self::detected::DETECTED_VERSION;

/// Version detected by build script.
#[allow(unused_imports)]
mod detected {
    use crate::date::Date;
    use crate::version::Channel::*;
    use crate::version::RustVersion as Version;

    #[cfg(not(host_os = "windows"))]
    pub const DETECTED_VERSION: Version = include!(concat!(env!("OUT_DIR"), "/version.expr"));

    #[cfg(host_os = "windows")]
    pub const DETECTED_VERSION: Version = include!(concat!(env!("OUT_DIR"), "\\version.expr"));
}
