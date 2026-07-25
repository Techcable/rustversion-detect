//! Contains a basic [`Date`] type used for differentiating nightly versions of rust.
//!
//! Intentionally ignores timezone information, making it much simpler than the [`time` crate]
//!
//! [`time` crate]: https://github.com/time-rs/time

use core::fmt::{self, Display};
use std::str::FromStr;

/// Indicates the date.
///
/// Used for the nightly versions of rust.
///
/// The timezone is not explicitly specified here,
/// and matches whatever one the rust team uses for nightly releases.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    /// The year (AD/CE)
    year: u32,
    /// The month (1..=12)
    month: u8,
    /// The day of the month.
    day: u8,
}
impl Date {
    /// Create a date, using YYYY-MM-DD format (ISO 8601).
    ///
    /// # Panics
    /// May panic if the date is invalid.
    /// See [`Self::try_new`] for a version that returns an error instead.
    ///
    /// # Examples
    /// ```
    /// # use rustversion_detect::Date;
    /// let x = Date::new(2018, 10, 21);
    /// let y = Date::new(2017, 11, 30);
    /// assert!(x.is_since(y));
    /// assert!(x.is_since(x));
    /// ```
    ///
    #[inline]
    pub fn new(year: u32, month: u32, day: u32) -> Self {
        match Self::try_new(year, month, day) {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        }
    }

    /// Create a date, using YYYY-MM-DD format (ISO 8601).
    ///
    /// # Errors
    /// Returns an error if the date is obviously invalid.
    /// However, this validation may have false negatives.
    /// See [`Self::new`] for a version that panics instead.
    pub fn try_new(year: u32, month: u32, day: u32) -> Result<Self, DateValidationError> {
        if year < 1 {
            return Err(DateValidationError {
                field: InvalidDateField::Year,
                value: year,
            });
        }
        if month < 1 || month > 12 {
            return Err(DateValidationError {
                field: InvalidDateField::Month,
                value: month,
            });
        }
        if day < 1 || day > max_days_of_month(month) {
            return Err(DateValidationError {
                field: InvalidDateField::DayOfMonth { month },
                value: day,
            });
        }
        Ok(Date {
            month: month as u8,
            day: day as u8,
            year,
        })
    }

    /// Check if this date is later than or equal to the specified start.
    ///
    /// Equivalent to `self >= start`, but potentially clearer.
    ///
    /// # Example
    /// ```
    /// # use rustversion_detect::Date;;
    /// assert!(Date::new(2024, 11, 16).is_since(Date::new(2024, 7, 28)));
    /// ```
    #[inline]
    pub fn is_since(&self, start: Date) -> bool {
        *self >= start
    }

    /// Check if this date is before the specified end.
    ///
    /// Equivalent to `self < end`, but potentially clearer.
    ///
    /// # Example
    /// ```
    /// # use rustversion_detect::Date;
    /// assert!(Date::new(2018, 12, 14).is_before(Date::new(2022, 8, 16)));
    /// assert!(Date::new(2024, 11, 14).is_before(Date::new(2024, 12, 7)));
    /// assert!(Date::new(2024, 11, 14).is_before(Date::new(2024, 11, 17)));
    /// ```
    #[inline]
    pub fn is_before(&self, end: Date) -> bool {
        *self < end
    }

    /// The year (AD/CE), in the range `1..`
    #[inline]
    pub fn year(&self) -> u32 {
        self.year
    }

    /// The month of the year, in the range `1..=12`.
    #[inline]
    pub fn month(&self) -> u32 {
        self.month as u32
    }

    /// The day of the year, in the range `1..=31`.
    #[inline]
    pub fn day(&self) -> u32 {
        self.day as u32
    }
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
impl FromStr for Date {
    type Err = DateParseError;
    fn from_str(full_text: &str) -> Result<Self, Self::Err> {
        fn do_parse(full_text: &str) -> Result<Date, ParseErrorReason> {
            let mut raw_parts = full_text.split('-');
            let mut parts: [Option<u32>; 3] = [None; 3];
            for part in &mut parts {
                let raw_part = raw_parts.next().ok_or(ParseErrorReason::MalformedSyntax)?;
                *part = Some(raw_part.parse().map_err(|cause| {
                    ParseErrorReason::NumberParseFailure {
                        text: raw_part.into(),
                        cause,
                    }
                })?);
            }
            if raw_parts.next().is_some() {
                return Err(ParseErrorReason::MalformedSyntax);
            }
            Date::try_new(parts[0].unwrap(), parts[1].unwrap(), parts[2].unwrap())
                .map_err(ParseErrorReason::ValidationFailure)
        }
        match do_parse(full_text) {
            Ok(res) => Ok(res),
            Err(reason) => Err(DateParseError {
                full_text: full_text.into(),
                reason,
            }),
        }
    }
}
/// An error that occurs parsing a [`Date`].
#[derive(Debug)]
pub struct DateParseError {
    full_text: String,
    reason: ParseErrorReason,
}
impl std::error::Error for DateParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.reason {
            ParseErrorReason::MalformedSyntax => None,
            ParseErrorReason::NumberParseFailure { ref cause, .. } => Some(cause),
            ParseErrorReason::ValidationFailure(ref cause) => Some(cause),
        }
    }
}
#[derive(Debug)]
enum ParseErrorReason {
    MalformedSyntax,
    NumberParseFailure {
        text: String,
        cause: std::num::ParseIntError,
    },
    ValidationFailure(DateValidationError),
}
impl Display for DateParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse `{:?}` as a date: ", self.full_text)?;
        match self.reason {
            ParseErrorReason::MalformedSyntax => {
                write!(f, "Not in ISO 8601 format (example: 2025-12-31)")
            }
            ParseErrorReason::NumberParseFailure {
                ref text,
                ref cause,
            } => {
                write!(f, "Failed to parse `{}` as number ({})", text, cause)
            }
            ParseErrorReason::ValidationFailure(ref cause) => Display::fmt(cause, f),
        }
    }
}

/// Return the maximum numbers of days in the specified month,
/// assuming that the Gregorian calendar is being used.
///
/// Does not take into account leap years.
#[inline]
fn max_days_of_month(x: u32) -> u32 {
    match x {
        1 => 31,
        2 => 29,
        _ => 30 + ((x + 1) % 2),
    }
}
/// An error that occurs in [`Date::try_new`] when an invalid date is encountered.
#[derive(Debug)]
pub struct DateValidationError {
    field: InvalidDateField,
    value: u32,
}
impl std::error::Error for DateValidationError {}
#[derive(Debug)]
enum InvalidDateField {
    Year,
    Month,
    DayOfMonth { month: u32 },
}
impl Display for DateValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let field_name = match self.field {
            InvalidDateField::Year => "year",
            InvalidDateField::Month => "month",
            InvalidDateField::DayOfMonth { .. } => "day of month",
        };
        write!(f, "Invalid {} `{}`", field_name, self.value)?;
        match self.field {
            InvalidDateField::Year | InvalidDateField::Month => {}
            InvalidDateField::DayOfMonth { month } => {
                write!(f, " for month {}", month)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // (before, after)
    fn test_dates() -> Vec<(Date, Date)> {
        vec![
            (Date::new(2018, 12, 14), Date::new(2022, 8, 16)),
            (Date::new(2024, 11, 14), Date::new(2024, 12, 7)),
            (Date::new(2024, 11, 14), Date::new(2024, 11, 17)),
        ]
    }

    #[test]
    fn days_of_month() {
        assert_eq!(max_days_of_month(1), 31);
        assert_eq!(max_days_of_month(12), 31);
        assert_eq!(max_days_of_month(2), 29);
        assert_eq!(max_days_of_month(10), 31);
    }

    #[test]
    fn before_after() {
        for (before, after) in test_dates() {
            assert!(before.is_before(after), "{} & {}", before, after);
            assert!(after.is_since(before), "{} & {}", before, after);
            // check equal dates
            for &date in [before, after].iter() {
                assert!(date.is_since(date), "{}", date);
                assert!(!date.is_before(date), "{}", date);
            }
        }
    }

    #[test]
    #[should_panic(expected = "Invalid year")]
    fn invalid_year() {
        Date::new(0, 7, 18);
    }

    #[test]
    #[should_panic(expected = "Invalid month")]
    fn invalid_month() {
        Date::new(2014, 13, 18);
    }

    #[test]
    #[should_panic(expected = "Invalid day of month")]
    fn invalid_date() {
        Date::new(2014, 7, 36);
    }

    #[test]
    #[should_panic(expected = "Invalid day of month")]
    fn contextually_invalid_date() {
        Date::new(2014, 2, 30);
    }
}
