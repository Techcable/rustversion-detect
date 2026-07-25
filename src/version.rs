//! Defines the rust version types.

use core::fmt::{self, Display, Formatter};
use core::num::ParseIntError;
use core::str::FromStr;

use crate::date::Date;

/// Specifies a specific stable version, like `1.48`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    ///
    /// # Panics
    /// Panics if the major version is not `1`.
    #[inline]
    #[must_use]
    pub fn minor(major: u32, minor: u32) -> Self {
        check_major_version(major);
        StableVersionSpec {
            major,
            minor,
            patch: None,
        }
    }

    /// Specify a patch version like `1.32.4`.
    ///
    /// # Panics
    /// Panics if the major version is not `1`.
    #[inline]
    #[must_use]
    pub fn patch(major: u32, minor: u32, patch: u32) -> Self {
        check_major_version(major);
        StableVersionSpec {
            major,
            minor,
            patch: Some(patch),
        }
    }

    /// Convert this specification into a concrete [`RustVersion`].
    ///
    /// If the patch version is not specified,
    /// it is assumed to be zero.
    #[inline]
    #[must_use]
    pub fn to_version(&self) -> RustVersion {
        RustVersion::stable(self.major, self.minor, self.patch.unwrap_or(0))
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
pub enum StableVersionParseError {
    #[doc(hidden)]
    InvalidNumber(ParseIntError),
    #[doc(hidden)]
    BadNumberParts,
    #[doc(hidden)]
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
    /// Create a stable version with the specified combination of major, minor, and patch.
    ///
    /// The major version must be 1.0.
    #[inline]
    #[must_use]
    pub fn stable(major: u32, minor: u32, patch: u32) -> RustVersion {
        check_major_version(major);
        RustVersion {
            major,
            minor,
            patch,
            channel: Channel::Stable,
        }
    }

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
    #[inline]
    #[must_use]
    pub fn is_since_minor_version(&self, major: u32, minor: u32) -> bool {
        self.is_since_stable(StableVersionSpec::minor(major, minor))
    }

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
    #[inline]
    #[must_use]
    pub fn is_since_patch_version(&self, major: u32, minor: u32, patch: u32) -> bool {
        self.is_since_stable(StableVersionSpec::patch(major, minor, patch))
    }

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
    #[inline]
    #[must_use]
    pub fn is_since_stable(&self, spec: StableVersionSpec) -> bool {
        self.major > spec.major
            || (self.major == spec.major
                && (self.minor > spec.minor
                    || (self.minor == spec.minor
                        && match spec.patch {
                            None => true, // missing spec always matches
                            Some(patch_spec) => self.patch >= patch_spec,
                        })))
    }

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
    #[inline]
    #[must_use]
    pub fn is_before_stable(&self, spec: StableVersionSpec) -> bool {
        !self.is_since_stable(spec)
    }

    /// Check if this version is before the specified stable minor version.
    ///
    /// The patch version is unspecified and will be ignored.
    ///
    /// This is a shorthand for calling [`Self::is_before_stable`] with a minor version
    /// spec created with [`StableVersionSpec::minor`].
    ///
    /// The major version must always be one, or a panic could happen.
    #[inline]
    #[must_use]
    pub fn is_before_minor_version(&self, major: u32, minor: u32) -> bool {
        self.is_before_stable(StableVersionSpec::minor(major, minor))
    }

    /// Check if this version is before the specified stable patch version.
    ///
    /// This is a shorthand for calling [`Self::is_before_stable`] with a patch version
    /// spec created with [`StableVersionSpec::patch`].
    ///
    /// The major version must always be one, or a panic could happen.
    #[inline]
    #[must_use]
    pub fn is_before_patch_version(&self, major: u32, minor: u32, patch: u32) -> bool {
        self.is_before_stable(StableVersionSpec::patch(major, minor, patch))
    }

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
    #[inline]
    #[must_use]
    pub fn is_since_nightly(&self, start: Date) -> bool {
        match self.channel {
            Channel::Nightly { date } => date.is_since(start),
            Channel::Stable | Channel::Beta => false, // before every nightly
            Channel::Development => true,             // after every nightly version
            Channel::__NonExhaustive => unreachable!(),
        }
    }

    /// If this version comes before the nightly version with the specified start date.
    ///
    /// Stable and beta versions are always considered before every nightly versions.
    /// Development versions are considered after every nightly version.
    ///
    /// The negation of [`Self::is_since_nightly`].
    ///
    /// See also [`Date::is_before`].
    #[inline]
    #[must_use]
    pub fn is_before_nightly(&self, start: Date) -> bool {
        match self.channel {
            Channel::Nightly { date } => date <= start,
            Channel::Stable | Channel::Beta => false, // before every nightly
            Channel::Development => true,             // after every nightly version
            Channel::__NonExhaustive => unreachable!(),
        }
    }

    /// Check if this is a nightly compiler version.
    #[inline]
    #[must_use]
    pub fn is_nightly(&self) -> bool {
        self.channel.is_nightly()
    }

    /// Check if this is a stable compiler version.
    #[inline]
    #[must_use]
    pub fn is_stable(&self) -> bool {
        self.channel.is_stable()
    }

    /// Check if this is a beta compiler version.
    #[inline]
    #[must_use]
    pub fn is_beta(&self) -> bool {
        self.channel.is_beta()
    }

    /// Check if this is a development compiler version.
    #[inline]
    #[must_use]
    pub fn is_development(&self) -> bool {
        self.channel.is_development()
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
            Channel::Nightly { ref date } => {
                write!(f, "-nightly ({})", date)
            }
            Channel::Development => f.write_str("-dev"),
            Channel::__NonExhaustive => unreachable!(),
        }
    }
}

/// The [channel] of the rust compiler release.
///
/// [channel]: https://rust-lang.github.io/rustup/concepts/channels.html
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Channel {
    /// A stable compiler version.
    Stable,
    /// A beta compiler version.
    Beta,
    /// A nightly compiler version.
    Nightly {
        /// The date that the compiler was released.
        date: Date,
    },
    /// A development compiler version.
    ///
    /// These are compiled directly instead of distributed through [rustup](https://rustup.rs).
    Development,
    #[doc(hidden)]
    __NonExhaustive,
}
impl Channel {
    /// Check if this is the nightly channel.
    #[inline]
    #[must_use]
    pub fn is_nightly(&self) -> bool {
        // NOTE: Can't use matches! because of minimum rust version
        match *self {
            Channel::Nightly { .. } => true,
            _ => false,
        }
    }

    /// Check if this is the stable channel.
    #[inline]
    #[must_use]
    pub fn is_stable(&self) -> bool {
        match *self {
            Channel::Stable => true,
            _ => false,
        }
    }

    /// Check if this is the beta channel.
    #[inline]
    #[must_use]
    pub fn is_beta(&self) -> bool {
        match *self {
            Channel::Beta => true,
            _ => false,
        }
    }

    /// Check if this is the development channel.
    #[inline]
    #[must_use]
    pub fn is_development(&self) -> bool {
        match *self {
            Channel::Development => true,
            _ => false,
        }
    }
}

#[inline]
fn check_major_version(major: u32) {
    assert_eq!(major, 1, "Major version must be 1.*");
}

#[cfg(test)]
mod test {
    use super::{RustVersion, StableVersionSpec};

    // (before, after)
    fn versions() -> Vec<(RustVersion, RustVersion)> {
        vec![
            (RustVersion::stable(1, 7, 8), RustVersion::stable(1, 89, 0)),
            (RustVersion::stable(1, 18, 0), RustVersion::stable(1, 80, 3)),
        ]
    }

    #[cfg(test)]
    impl RustVersion {
        #[inline]
        pub(crate) fn to_spec(self) -> StableVersionSpec {
            StableVersionSpec::patch(self.major, self.minor, self.patch)
        }
    }

    #[test]
    fn test_before_after() {
        for (before, after) in versions() {
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
