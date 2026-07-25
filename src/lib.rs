//! Parses the output of `rustc --version` for use in build scripts.
//!
//! The parsed output is cached in a static variable,
//! so `rustc` will only be invoked once no matter how many build scripts use this crate.
//! This gives an advantage over using [`autocfg` crate][`autocfg`] or performing ad-hoc detection,
//! as these require re-running `rustc` for each build scripts.
//!
//! This crate originated as a fork of [dtolnay's `rustversion` crate][`rustversion`],
//! but with the proc-macro code removed and version detection moved from the build script to runtime.
//! The core version detection logic has been kept up to date with upstream changes.
//! It currently mirrors rustversion v1.0.23.
//!
//! Moving the version detection logic to runtime means that this crate
//! does not need its own build script,
//! reducing compile times compared to using the [`rustversion`] macros in a build script.
//!
//! [`rustversion`]: https://github.com/dtolnay/rustversion
//! [`autocfg`]: https://github.com/cuviper/autocfg
//!
//! # Dependency
//! Add the following to your build script:
//! ```toml
//! [build-dependencies]
//! rustversion-detect = "0.2"
//! ```
//!
//! # Examples
//! ```
//! pub fn main() {
//!     // by default rust re-runs the build script if any source file changes
//!     // this directive indicates that the build script only needs
//!     // to be rerun if the compiler flags change (or if the build script itself changes)
//!     println!("cargo:rerun-if-changed=build.rs");
//!
//!     // Rust 1.80 requires listing all possibilities using this directive
//!     println!("cargo:rustc-check-cfg=cfg(use_nightly)");
//!
//!     let version = rustversion_detect::detect_version().unwrap();
//!     if version.is_nightly() {
//!         println!("cargo:rustc-cfg=use_nightly");
//!     }
//! }
//! ```
// These lints indicate serious problems which I would normally mark as #[deny(...)].
// However, failing the build could cause problems for users of this library.
#![warn(missing_docs)]
use std::error::Error;
use std::fmt::{self, Display};

mod build;
pub mod date;
pub mod version;

pub use crate::date::Date;
pub use crate::version::{Channel, RustVersion, StableVersionSpec};

/// Detect the current version by executing `rustc`.
///
/// This should only be called at build time (usually a build script),
/// since the rust compiler is likely unavailable at runtime.
/// It will execute whatever command is present in the `RUSTC` environment variable,
/// so should not be run in an untrusted environment.
///
/// Once the version is successfully detected,
/// it will be cached for future runs.
///
/// # Errors
/// Returns an error if unable to execute the result compiler
/// or unable to parse the result.
pub fn detect_version() -> Result<crate::RustVersion, VersionDetectionError> {
    {
        let lock = state::state_mutex()
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(cached) = &*lock {
            return Ok(*cached);
        }
        // release the lock & fallthrough to detection
    }
    match build::determine_version() {
        Ok(success) => {
            {
                let mut lock = state::state_mutex()
                    .write()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                *lock = Some(success);
            }
            Ok(success)
        }
        Err(failure) => Err(failure),
    }
}

/// Indicates failure to detect the compiler's rust version.
#[derive(Debug)]
pub struct VersionDetectionError {
    desc: String,
    cause: Option<std::io::Error>,
}
impl VersionDetectionError {
    pub(crate) fn new(desc: String) -> Self {
        VersionDetectionError { desc, cause: None }
    }

    pub(crate) fn with_cause(desc: String, cause: std::io::Error) -> Self {
        VersionDetectionError {
            desc,
            cause: Some(cause),
        }
    }
}
impl Display for VersionDetectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.desc)?;
        if let Some(ref cause) = self.cause {
            write!(f, ": {}", cause)?;
        }
        Ok(())
    }
}
impl Error for VersionDetectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_ref().map(|x| x as _)
    }
}

/// Caches the detected rust version.
#[allow(unused_imports)]
mod state {
    use std::sync::{Once, RwLock};

    #[allow(deprecated)] // Only available since 1.32
    static CACHED_STATE_INIT: Once = std::sync::ONCE_INIT;
    static mut CACHED_STATE: Option<RwLock<Option<crate::RustVersion>>> = None;

    pub fn state_mutex() -> &'static RwLock<Option<crate::RustVersion>> {
        CACHED_STATE_INIT.call_once(|| {
            // SAFETY: Will only be called once
            unsafe {
                CACHED_STATE = Some(RwLock::new(None));
            }
        });
        // SAFETY: After completion of `Once::call_once`,
        // the mutex is fully initialized and not `None`
        unsafe {
            match CACHED_STATE {
                Some(ref mutex) => mutex,
                None => std::hint::unreachable_unchecked(),
            }
        }
    }
}
