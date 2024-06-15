//! Contains a basic [`Date`] type used for differentiating nightly versions of rust.
//!
//! Intentionally ignores timezone information, making it simpler than the [`time` crate]
//!
//! [`time` crate]: https://github.com/time-rs/time

use core::fmt::{self, Display};

/// Indicates the date.
///
/// Used for the nightly versions of rust.
///
/// The timezone is not explicitly specified here,
/// and matches whatever one the rust team uses for nightly releases.
///
/// Can be created by the [`date!`](crate::date!) macro.
/// For example `date!(2014-12-31)`
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    /// The year
    pub year: u16,
    /// The month
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}
impl Date {
    /// Create a date, using YYYY-MM-DD format (ISO 8601).
    ///
    /// For example, `Date::from_iso(2018-
    ///
    /// If possible, perform some basic validation.
    #[inline]
    #[cfg_attr(has_track_caller, track_caller)]
    pub const fn new(year: u16, month: u8, day: u8) -> Self {
        #[cfg(has_const_panic)]
        {
            assert!(year >= 1, "Invalid year");
            assert!(month >= 1 && month <= 12, "Invalid month");
            assert!(day >= 1 && day <= 31, "Invalid day of month");
        }
        Date { year, month, day }
    }

    /// Check if this date is equal to or after the specified start.
    ///
    /// Equivalent to `self >= start`,
    /// but available as a `const` function.
    ///
    /// ## Example
    /// ```
    /// # use rustversion_detect::date;;
    ///
    /// assert!(date!(2024-11-16).is_since(date!(2024-7-28)));
    /// ```
    #[inline]
    pub const fn is_since(&self, start: Date) -> bool {
        self.year > start.year
            || (self.year == start.year
                && (self.month > start.month
                    || (self.month == start.month && self.day >= start.day)))
    }

    /// Check if this date is before the specified end.
    ///
    /// Equivalent to `self < end`,
    /// but available as a `const` function.
    ///
    /// ## Example
    /// ```
    /// # use rustversion_detect::date;
    ///
    /// assert!(date!(2018-12-14).is_before(date!(2022-8-16)));
    /// assert!(date!(2024-11-14).is_before(date!(2024-12-7)));
    /// assert!(date!(2024-11-14).is_before(date!(2024-11-17)));
    /// ```
    #[inline]
    pub const fn is_before(&self, end: Date) -> bool {
        !self.is_since(end)
    }
}

/// Declare a [`Date`] using the YYYY-MM-DD format (ISO 8601).
#[macro_export]
macro_rules! date {
    ($year:literal - $month:literal - $day:literal) => {{
        // NOTE: The Date::new function sometimes perfroms validation
        // It only validates if `const_panic` is stablized.
        const DTE: $crate::date::Date = $crate::date::Date::new($year, $month, $day);
        DTE
    }};
}

/// Displays the date consistent with the ISO 8601 standard.
impl Display for Date {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{:04}-{:02}-{:02}",
            self.year, self.month, self.day,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::Date;

    // (before, after)
    const TEST_DATES: &[(Date, Date)] = &[
        (date!(2018 - 12 - 14), date!(2022 - 8 - 16)),
        (date!(2024 - 11 - 14), date!(2024 - 12 - 7)),
        (date!(2024 - 11 - 14), date!(2024 - 11 - 17)),
    ];

    #[test]
    fn test_before_after() {
        for &(before, after) in TEST_DATES {
            assert!(before.is_before(after), "{} & {}", before, after);
            assert!(after.is_since(before), "{} & {}", before, after);
            // check equal dates
            for date in [before, after] {
                assert!(date.is_since(date), "{}", date);
                assert!(!date.is_before(date), "{}", date);
            }
        }
    }

    #[test]
    #[cfg_attr(has_const_panic, should_panic(expected = "Invalid year"))]
    fn test_invalid_year() {
        Date::new(0, 7, 18);
    }

    #[test]
    #[cfg_attr(has_const_panic, should_panic(expected = "Invalid month"))]
    fn test_invalid_month() {
        Date::new(2014, 13, 18);
    }

    #[test]
    #[cfg_attr(has_const_panic, should_panic(expected = "Invalid day of month"))]
    fn test_invalid_date() {
        Date::new(2014, 7, 36);
    }
}
