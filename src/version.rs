//! Defines the rust version types.

use core::fmt::{self, Display, Formatter};

use crate::date::Date;

/// Specifies a specific stable version, like `1.48`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(has_non_exhaustive, non_exhaustive)]
pub struct StableVersion {
    /// The major version
    pub major: u32,
    /// The minor version
    pub minor: u32,
    /// The patch version.
    ///
    /// If this is `None`, it will match any patch version.
    pub patch: Option<u32>,
}
impl StableVersion {
    /// Specify a minor version like `1.32`.
    #[inline]
    #[cfg_attr(has_track_caller, track_caller)]
    pub const fn minor(major: u32, minor: u32) -> Self {
        check_major_version(major);
        StableVersion {
            major,
            minor,
            patch: None,
        }
    }

    /// Specify a patch version like `1.32.4`.
    #[inline]
    #[cfg_attr(has_track_caller, track_caller)]
    pub const fn patch(major: u32, minor: u32, patch: u32) -> Self {
        check_major_version(major);
        StableVersion {
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

/// Show the specification in a manner consistent with the `spec!` macro.
impl Display for StableVersion {
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
        #[inline]
        /// Check if this version is after the specified stable version,
        ///
        /// Ignores the channel.
        /// The negation of [`Self::is_before`].
        ///
        /// Behavior is (mostly) equivalent to `#[rustversion::since($spec)]`
        ///
        /// ## Example
        /// ```
        /// # use rustversion_detect::{RustVersion, StableVersion};
        ///
        /// assert!(RustVersion::stable(1, 32, 2).is_since(StableVersion::minor(1, 32)));
        /// assert!(RustVersion::stable(1, 48, 0).is_since(StableVersion::patch(1, 32, 7)))
        /// ```
        pub const fn is_since(&self, spec: StableVersion) -> bool {
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
        /// Check if the version is greater than or equal to the specified version specification.
        ///
        /// Ignores the channel.
        /// The negation of [`Self::is_since`].
        ///
        /// Behavior is (mostly) equivalent to `#[rustversion::before($spec)]`
        pub const fn is_before(&self, spec: StableVersion) -> bool {
            !self.is_since(spec)
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

impl From<StableVersion> for RustVersion {
    #[inline]
    fn from(value: StableVersion) -> Self {
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

//noinspection RsAssertEqual
#[inline]
#[cfg_attr(has_track_caller, track_caller)]
const fn check_major_version(major: u32) {
    #[cfg(has_const_panic)]
    {
        assert!(major == 1, "Major version must be 1.*");
    }
    let _ = major;
}

#[cfg(test)]
mod test {
    use super::{RustVersion, StableVersion};

    // (before, after)
    const VERSIONS: &[(RustVersion, RustVersion)] = &[
        (RustVersion::stable(1, 7, 8), RustVersion::CURRENT),
        (RustVersion::stable(1, 18, 0), RustVersion::stable(1, 80, 3)),
    ];

    #[cfg(test)]
    impl RustVersion {
        #[inline]
        pub fn to_spec(&self) -> StableVersion {
            StableVersion::patch(self.major, self.minor, self.patch)
        }
    }

    #[test]
    fn test_before_after() {
        for (before, after) in VERSIONS {
            assert!(before.is_before(after.to_spec()), "{} & {}", before, after);
            assert!(after.is_since(before.to_spec()), "{} & {}", before, after);
        }
    }
}
