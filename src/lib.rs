//! Simple API for detecting the rust version.
#![no_std]
#![deny(missing_docs)]

#[macro_use]
mod macros;
pub mod date;
pub mod version;

pub use crate::date::Date;
pub use crate::version::{Channel, RustVersion, StableVersion};

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
