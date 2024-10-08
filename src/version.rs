//! Defines the rust version types.

use core::fmt::{self, Display, Formatter};
use core::num::ParseIntError;
use core::str::FromStr;

use crate::date::Date;

/// Specify a stable version (a [`StableVersionSpec`]).
///
/// Unfortunately, this does not work in a `const` setting.
/// If that is required, please use [`StableVersionSpec::minor`] or [`StableVersionSpec::major`] constructors.
#[deprecated(note = "Please use helper methods (is_since_minor/is_since_patch)")]
#[macro_export]
macro_rules! spec {
    ($($items:tt)+) => {{
        let text = stringify!($($items)*);
        match text.parse::<$crate::version::StableVersionSpec>() {
            Ok(val) => val,
            Err(e) => panic!("Invalid version spec {:?} ({:?})", text, e),
        }
    }};
}

/// Specifies a specific stable version, like `1.48`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(has_non_exhaustive, non_exhaustive)]
pub struct StableVersionSpec {
    /// The major version
    pub major: u32,
    /// The minor version
    pub minor: u32,
    /// The patch version.
    ///
    /// If this is `None`, it will match any patch version.
    pub patch: Option<u32>,
}
impl StableVersionSpec {
    /// Specify a minor version like `1.32`.
    #[inline]
    #[cfg_attr(has_track_caller, track_caller)]
    pub const fn minor(major: u32, minor: u32) -> Self {
        #[cfg(has_const_panic)]
        check_major_version(major);
        StableVersionSpec {
            major,
            minor,
            patch: None,
        }
    }

    /// Specify a patch version like `1.32.4`.
    #[inline]
    #[cfg_attr(has_track_caller, track_caller)]
    pub const fn patch(major: u32, minor: u32, patch: u32) -> Self {
        #[cfg(has_const_panic)]
        check_major_version(major);
        StableVersionSpec {
            major,
            minor,
            patch: Some(patch),
        }
    }

    maybe_const_fn! {
        #[cfg_const(has_const_match)]
        /// Convert this specification into a concrete [`RustVersion`].
        ///
        /// If the patch version is not specified,
        /// it is assumed to be zero.
        #[inline]
        pub const fn to_version(&self) -> RustVersion {
            RustVersion::stable(
                self.major,
                self.minor,
                match self.patch {
                    None => 0,
                    Some(val) => val,
                }
            )
        }
    }
}
impl FromStr for StableVersionSpec {
    type Err = StableVersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('.');
        let major = iter
            .next()
            .ok_or(StableVersionParseError::BadNumberParts)?
            .parse::<u32>()?;
        let minor = iter
            .next()
            .ok_or(StableVersionParseError::BadNumberParts)?
            .parse::<u32>()?;
        let patch = match iter.next() {
            Some(patch_text) => Some(patch_text.parse::<u32>()?),
            None => None,
        };
        if iter.next().is_some() {
            return Err(StableVersionParseError::BadNumberParts);
        }
        if major != 1 {
            return Err(StableVersionParseError::InvalidMajorVersion);
        }
        Ok(StableVersionSpec {
            major,
            minor,
            patch,
        })
    }
}

/// An error while parsing a [`StableVersionSpec`].
///
/// The specifics of this error are implementation-dependent.
#[derive(Clone, Debug)]
#[cfg_attr(has_non_exhaustive, non_exhaustive)]
#[doc(hidden)]
pub enum StableVersionParseError {
    InvalidNumber(ParseIntError),
    BadNumberParts,
    InvalidMajorVersion,
}
impl From<ParseIntError> for StableVersionParseError {
    #[inline]
    fn from(cause: ParseIntError) -> Self {
        StableVersionParseError::InvalidNumber(cause)
    }
}

/// Show the specification in a manner consistent with the `spec!` macro.
impl Display for StableVersionSpec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)?;
        if let Some(patch) = self.patch {
            write!(f, ".{}", patch)?;
        }
        Ok(())
    }
}

/// Indicates the rust version.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RustVersion {
    /// The major version.
    ///
    /// Should always be one.
    pub major: u32,
    /// The minor version of rust.
    pub minor: u32,
    /// The patch version of the rust compiler.
    pub patch: u32,
    /// The channel of the rust compiler.
    pub channel: Channel,
}
impl RustVersion {
    /// The current rust version.
    ///
    /// This is an alias for [`rustversion_detect::RUST_VERSION`](`crate::RUST_VERSION`).
    pub const CURRENT: Self = crate::RUST_VERSION;

    /// Create a stable version with the specified combination of major, minor, and patch.
    ///
    /// The major version must be 1.0.
    #[inline]
    #[cfg_attr(has_track_caller, track_caller)]
    pub const fn stable(major: u32, minor: u32, patch: u32) -> RustVersion {
        #[cfg(has_const_panic)]
        check_major_version(major);
        RustVersion {
            major,
            minor,
            patch,
            channel: Channel::Stable,
        }
    }

    maybe_const_fn! {
        #[cfg_const(has_const_match)]
        #[cfg_attr(has_track_caller, track_caller)]
        #[inline]
        /// Check if this version is after the specified stable minor version.
        ///
        /// The patch version is unspecified and will be ignored.
        ///
        /// This is a shorthand for calling [`Self::is_since_stable`] with a minor version
        /// spec created with [`StableVersionSpec::minor`].
        ///
        /// The major version must always be one, or a panic could happen.
        ///
        /// ## Example
        /// ```
        /// # use rustversion_detect::RustVersion;
        ///
        /// assert!(RustVersion::stable(1, 32, 2).is_since_minor_version(1, 32));
        /// assert!(RustVersion::stable(1, 48, 0).is_since_minor_version(1, 40));
        /// ```
        pub const fn is_since_minor_version(&self, major: u32, minor: u32) -> bool {
            self.is_since_stable(StableVersionSpec::minor(major, minor))
        }

        #[cfg_const(has_const_match)]
        #[cfg_attr(has_track_caller, track_caller)]
        #[inline]
        /// Check if this version is after the specified stable patch version.
        ///
        /// This is a shorthand for calling [`Self::is_since_stable`] with a patch version
        /// spec created with [`StableVersionSpec::patch`].
        ///
        /// The major version must always be one, or a panic could happen.
        ///
        /// ## Example
        /// ```
        /// # use rustversion_detect::RustVersion;
        ///
        /// assert!(RustVersion::stable(1, 32, 2).is_since_patch_version(1, 32, 1));
        /// assert!(RustVersion::stable(1, 48, 0).is_since_patch_version(1, 40, 5));
        /// ```
        pub const fn is_since_patch_version(&self, major: u32, minor: u32, patch: u32) -> bool {
            self.is_since_stable(StableVersionSpec::patch(major, minor, patch))
        }

        #[cfg_const(has_const_match)]
        #[inline]
        /// Check if this version is after the given [stable version spec](StableVersionSpec).
        ///
        /// In general, the [`Self::is_since_minor_version`] and [`Self::is_since_patch_version`]
        /// helper methods are preferable.
        ///
        /// This ignores the channel.
        ///
        /// The negation of [`Self::is_before_stable`].
        ///
        /// Behavior is (mostly) equivalent to `#[rustversion::since($spec)]`
        ///
        /// ## Example
        /// ```
        /// # use rustversion_detect::{RustVersion, StableVersionSpec};
        ///
        /// assert!(RustVersion::stable(1, 32, 2).is_since_stable(StableVersionSpec::minor(1, 32)));
        /// assert!(RustVersion::stable(1, 48, 0).is_since_stable(StableVersionSpec::patch(1, 32, 7)))
        /// ```
        pub const fn is_since_stable(&self, spec: StableVersionSpec) -> bool {
            self.major > spec.major
                || (self.major == spec.major
                    && (self.minor > spec.minor
                        || (self.minor == spec.minor && match spec.patch {
                            None => true, // missing spec always matches
                            Some(patch_spec) => self.patch >= patch_spec,
                        })))
        }

        #[cfg_const(has_const_match)]
        #[inline]
        /// Check if the version is less than the given [stable version spec](StableVersionSpec).
        ///
        /// This ignores the channel.
        ///
        /// In general, the [`Self::is_before_minor_version`] and [`Self::is_before_patch_version`]
        /// helper methods are preferable.
        ///
        /// The negation of [`Self::is_since_stable`].
        ///
        /// Behavior is (mostly) equivalent to `#[rustversion::before($spec)]`
        pub const fn is_before_stable(&self, spec: StableVersionSpec) -> bool {
            !self.is_since_stable(spec)
        }


        #[cfg_const(has_const_match)]
        #[cfg_attr(has_track_caller, track_caller)]
        #[inline]
        /// Check if this version is before the specified stable minor version.
        ///
        /// The patch version is unspecified and will be ignored.
        ///
        /// This is a shorthand for calling [`Self::is_before_stable`] with a minor version
        /// spec created with [`StableVersionSpec::minor`].
        ///
        /// The major version must always be one, or a panic could happen.
        pub const fn is_before_minor_version(&self, major: u32, minor: u32) -> bool {
            self.is_before_stable(StableVersionSpec::minor(major, minor))
        }

        #[cfg_const(has_const_match)]
        #[cfg_attr(has_track_caller, track_caller)]
        #[inline]
        /// Check if this version is before the specified stable patch version.
        ///
        /// This is a shorthand for calling [`Self::is_before_stable`] with a patch version
        /// spec created with [`StableVersionSpec::patch`].
        ///
        /// The major version must always be one, or a panic could happen.
        pub const fn is_before_patch_version(&self, major: u32, minor: u32, patch: u32) -> bool {
            self.is_before_stable(StableVersionSpec::patch(major, minor, patch))
        }

        #[cfg_const(has_const_match)]
        #[deprecated(note = "Please use `is_since_stable` or the helper methods")]
        #[inline]
        /// Old name of [`Self::is_since_stable`].
        ///
        /// Deprecated due to unclear naming and preference for
        /// helper methods [`Self::is_since_minor_version`].
        pub const fn is_since(&self, spec: StableVersionSpec) -> bool {
            self.is_since_stable(spec)
        }

        #[cfg_const(has_const_match)]
        #[deprecated(note = "Please use `is_before_stable` or the helper methods")]
        #[inline]
        /// Old name of [`Self::is_before_stable`]
        ///
        /// Deprecated due to unclear naming and preference for
        /// helper methods like [`Self::is_before_minor_version`].
        pub const fn is_before(&self, spec: StableVersionSpec) -> bool {
            self.is_before_stable(spec)
        }

        #[cfg_const(has_const_match)]
        #[inline]
        /// If this version is a nightly version after the specified start date.
        ///
        /// Stable and beta versions are always considered before every nightly versions.
        /// Development versions are considered after every nightly version.
        ///
        /// The negation of [`Self::is_before_nightly`].
        ///
        /// Behavior is (mostly) equivalent to `#[rustversion::since($date)]`
        ///
        /// See also [`Date::is_since`].
        pub const fn is_since_nightly(&self, start: Date) -> bool {
            match self.channel {
                Channel::Nightly(date) => date.is_since(start),
                Channel::Stable | Channel::Beta => false, // before every nightly
                Channel::Dev => true, // after every nightly version
            }
        }


        #[cfg_const(has_const_match)]
        #[inline]
        /// If this version comes before the nightly version with the specified start date.
        ///
        /// Stable and beta versions are always considered before every nightly versions.
        /// Development versions are considered after every nightly version.
        ///
        /// The negation of [`Self::is_since_nightly`].
        ///
        /// See also [`Date::is_before`].
        pub const fn is_before_nightly(&self, start: Date) -> bool {
            match self.channel {
                Channel::Nightly(date) => date.is_before(start),
                Channel::Stable | Channel::Beta => false, // before every nightly
                Channel::Dev => true, // after every nightly version
            }
        }
    }

    maybe_const_fn! {
        #[cfg_const(has_const_match)]
        #[inline]
        /// Check if a nightly compiler version is used.
        pub const fn is_nightly(&self) -> bool {
            self.channel.is_nightly()
        }

        #[cfg_const(has_const_match)]
        #[inline]
        /// Check if a stable compiler version is used
        pub const fn is_stable(&self) -> bool {
            self.channel.is_stable()
        }

        #[cfg_const(has_const_match)]
        #[inline]
        /// Check if a beta compiler version is used
        pub const fn is_beta(&self) -> bool {
            self.channel.is_beta()
        }

        #[cfg_const(has_const_match)]
        #[inline]
        /// Check if a development compiler version is used
        pub const fn is_development(&self) -> bool {
            self.channel.is_development()
        }
    }
}

impl From<StableVersionSpec> for RustVersion {
    #[inline]
    fn from(value: StableVersionSpec) -> Self {
        value.to_version()
    }
}

/// Displays the version in a manner similar to `rustc --version`.
///
/// The format here is not stable and may change in the future.
impl Display for RustVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        match self.channel {
            Channel::Stable => Ok(()), // nothing
            Channel::Beta => f.write_str("-beta"),
            Channel::Nightly(ref date) => {
                write!(f, "-nightly ({})", date)
            }
            Channel::Dev => f.write_str("-dev"),
        }
    }
}

/// The [channel] of the rust compiler release.
///
/// [channel]: https://rust-lang.github.io/rustup/concepts/channels.html
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(has_non_exhaustive, non_exhaustive)]
pub enum Channel {
    /// The stable compiler
    Stable,
    /// The beta compiler
    Beta,
    /// The nightly compiler.
    Nightly(Date),
    /// A development compiler version.
    ///
    /// These are compiled directly instead of distributed through [rustup](https://rustup.rs).
    Dev,
}
impl Channel {
    maybe_const_fn! {
        #[cfg_const(has_const_match)]
        #[inline]
        /// Check if this is the nightly channel.
        pub const fn is_nightly(&self) -> bool {
            // NOTE: Can't use matches! because of minimum rust version
            match *self {
                Channel::Nightly(_) => true,
                _ => false,
            }
        }

        #[cfg_const(has_const_match)]
        #[inline]
        /// Check if this is the stable channel.
        pub const fn is_stable(&self) -> bool {
            match *self {
                Channel::Stable => true,
                _ => false,
            }
        }

        #[cfg_const(has_const_match)]
        #[inline]
        /// Check if this is the beta channel.
        pub const fn is_beta(&self) -> bool {
            match *self {
                Channel::Beta => true,
                _ => false,
            }
        }

        #[cfg_const(has_const_match)]
        #[inline]
        /// Check if this is the development channel.
        pub const fn is_development(&self) -> bool {
            match *self {
                Channel::Dev => true,
                _ => false,
            }
        }
    }
}

#[inline]
#[cfg_attr(has_track_caller, track_caller)]
#[cfg(has_const_panic)]
const fn check_major_version(major: u32) {
    assert!(major == 1, "Major version must be 1.*");
}

#[cfg(test)]
mod test {
    use super::{RustVersion, StableVersionSpec};

    // (before, after)
    const VERSIONS: &[(RustVersion, RustVersion)] = &[
        (RustVersion::stable(1, 7, 8), RustVersion::CURRENT),
        (RustVersion::stable(1, 18, 0), RustVersion::stable(1, 80, 3)),
    ];

    #[cfg(test)]
    impl RustVersion {
        #[inline]
        pub fn to_spec(&self) -> StableVersionSpec {
            StableVersionSpec::patch(self.major, self.minor, self.patch)
        }
    }

    // TODO: Remove this test
    #[test]
    #[ignore] // Broken on Rust 1.31
    #[allow(deprecated)] // spec! macro is deprecated
    fn test_spec_macro() {
        assert_eq!(spec!(1.40), StableVersionSpec::minor(1, 40));
        assert_eq!(spec!(1.12.3), StableVersionSpec::patch(1, 12, 3));
        assert!(RustVersion::stable(1, 12, 8).is_since(spec!(1.12.3)));
    }

    #[test]
    fn test_before_after() {
        for (before, after) in VERSIONS {
            assert!(
                before.is_before_stable(after.to_spec()),
                "{} & {}",
                before,
                after
            );
            assert!(
                after.is_since_stable(before.to_spec()),
                "{} & {}",
                before,
                after
            );
        }
    }
}
