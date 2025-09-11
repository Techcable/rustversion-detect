//! This crate provides a simple API for detecting the rustc
//! compiler version.
//!
//! It is only intended for use at build time,
//! because it requires executing the `rustc` compiler.
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
#![deny(missing_docs)]
use std::error::Error;
use std::fmt::{self, Display};

#[macro_use]
mod macros;
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
        match &*lock {
            Some(cached) => return Ok(*cached),
            _ => {
                // fallthrough to detection
            }
        }
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
