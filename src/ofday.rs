use crate::Span;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "binio", derive(binprot::macros::BinProtRead, binprot::macros::BinProtWrite))]
pub struct OfDay(i64);

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum OfDayError {
    InvalidHour(u8),
    InvalidMinute(u8),
    InvalidSecond(u8),
    InvalidNanosecond(u32),
}

use std::ops::{Add, AddAssign, Rem, Sub, SubAssign};

impl Add<Span> for OfDay {
    type Output = Self;

    fn add(self, other: Span) -> Self {
        Self(self.0 + other.to_int_ns()).clamp(Self::START_OF_DAY, Self::START_OF_NEXT_DAY)
    }
}

impl AddAssign<Span> for OfDay {
    fn add_assign(&mut self, other: Span) {
        *self = *self + other
    }
}

impl Sub<Span> for OfDay {
    type Output = Self;

    fn sub(self, other: Span) -> Self {
        Self(self.0 - other.to_int_ns()).clamp(Self::START_OF_DAY, Self::START_OF_NEXT_DAY)
    }
}

impl SubAssign<Span> for OfDay {
    fn sub_assign(&mut self, other: Span) {
        *self = *self - other
    }
}

impl Sub for OfDay {
    type Output = Span;

    fn sub(self, other: Self) -> Span {
        Span::of_int_ns(self.0 - other.0)
    }
}

impl Rem<Span> for OfDay {
    type Output = Span;

    fn rem(self, other: Span) -> Span {
        Span::of_int_ns(self.0 % other.to_int_ns())
    }
}

impl std::fmt::Display for OfDayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for OfDayError {}

impl OfDay {
    pub const START_OF_DAY: Self = OfDay(0);
    pub const START_OF_NEXT_DAY: Self = OfDay(24 * 3600 * 1_000_000_000);

    /// Creates a `OfDay` based on a number of nanoseconds since midnight.
    pub fn of_ns_since_midnight(i: i64) -> Self {
        Self(i).clamp(Self::START_OF_DAY, Self::START_OF_NEXT_DAY)
    }

    /// The number of nanoseconds since midnight.
    pub const fn to_ns_since_midnight(self) -> i64 {
        self.0
    }

    pub const fn to_span_since_midnight(self) -> crate::Span {
        crate::Span::of_int_ns(self.to_ns_since_midnight())
    }

    pub fn of_span_since_midnight(span: crate::Span) -> Self {
        Self::of_ns_since_midnight(span.to_int_ns())
    }

    /// The hour in the day, between 0 and 23 (or 24 for the start of next day).
    pub const fn hour(self) -> i64 {
        self.0 / 1_000_000_000 / 60 / 60
    }

    /// The minute, between 0 and 59.
    pub const fn minute(self) -> i64 {
        (self.0 / 1_000_000_000 / 60) % 60
    }

    /// The second, between 0 and 59.
    pub const fn second(self) -> i64 {
        (self.0 / 1_000_000_000) % 60
    }

    /// The nanosecond, between 0 and 999_999_999.
    pub const fn nanosecond(self) -> i64 {
        self.0 % 1_000_000_000
    }

    /// Creates a new `OfDay` based on hour/minute/second/nanosecond.
    pub const fn create(
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> Result<Self, OfDayError> {
        if hour >= 24 && !(hour == 24 && minute == 0 && second == 0 && nanosecond == 0) {
            Err(OfDayError::InvalidHour(hour))
        } else if minute >= 60 {
            Err(OfDayError::InvalidMinute(minute))
        } else if second >= 60 {
            Err(OfDayError::InvalidSecond(second))
        } else if nanosecond >= 1_000_000_000 {
            Err(OfDayError::InvalidNanosecond(nanosecond))
        } else {
            let second = hour as i64 * 3600 + minute as i64 * 60 + second as i64;
            Ok(Self(second * 1_000_000_000 + nanosecond as i64))
        }
    }

    /// Rounds the time to the previous multiple of `rhs`.
    pub fn prev_multiple(self, rhs: Span) -> Self {
        Self::of_span_since_midnight(self.to_span_since_midnight().prev_multiple(rhs))
    }

    /// Rounds the time to the next multiple of `rhs`.
    pub fn next_multiple(self, rhs: Span) -> Self {
        Self::of_span_since_midnight(self.to_span_since_midnight().next_multiple(rhs))
    }
}

impl std::fmt::Display for OfDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let hr = self.hour();
        let min = self.minute();
        let sec = self.second();
        let ns = self.nanosecond();
        if ns == 0 {
            write!(f, "{hr:02}:{min:02}:{sec:02}")
        } else {
            let mut ns = ns;
            let mut ns_width = 9;
            while ns % 10 == 0 {
                ns /= 10;
                ns_width -= 1;
            }
            write!(f, "{hr:02}:{min:02}:{sec:02}.{ns:0ns_width$}")
        }
    }
}

impl std::fmt::Debug for OfDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::Display::fmt(self, f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseOfDayError {
    EmptyString,
    MoreThanTwoColumns,
    MoreThanOneDot,
    InvalidNanosecondString(String),
    ParseIntError(std::num::ParseIntError),
    OfDayError(OfDayError),
}

impl std::fmt::Display for ParseOfDayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ParseOfDayError {}

impl From<std::num::ParseIntError> for ParseOfDayError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl From<OfDayError> for ParseOfDayError {
    fn from(e: OfDayError) -> Self {
        Self::OfDayError(e)
    }
}

impl std::str::FromStr for OfDay {
    type Err = ParseOfDayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(':').collect::<Vec<_>>()[..] {
            [] => Err(ParseOfDayError::EmptyString),
            [hour] => {
                let hour = u8::from_str(hour)?;
                Ok(Self::create(hour, 0, 0, 0)?)
            }
            [hour, minute] => {
                let hour = u8::from_str(hour)?;
                let minute = u8::from_str(minute)?;
                Ok(Self::create(hour, minute, 0, 0)?)
            }
            [hour, minute, second] => {
                let hour = u8::from_str(hour)?;
                let minute = u8::from_str(minute)?;
                match second.split('.').collect::<Vec<_>>()[..] {
                    [] => Ok(Self::create(hour, minute, 0, 0)?),
                    [second] => {
                        let second = u8::from_str(second)?;
                        Ok(Self::create(hour, minute, second, 0)?)
                    }
                    [second, nanosecond] => {
                        let second = u8::from_str(second)?;
                        if !nanosecond.chars().all(char::is_numeric) {
                            return Err(ParseOfDayError::InvalidNanosecondString(
                                nanosecond.to_string(),
                            ));
                        }
                        let ns_width = nanosecond.len();
                        let mut nanosecond = u32::from_str(nanosecond)?;
                        for _i in ns_width..9 {
                            nanosecond *= 10
                        }
                        Ok(Self::create(hour, minute, second, nanosecond)?)
                    }
                    [_, _, ..] => Err(ParseOfDayError::MoreThanOneDot),
                }
            }
            [_, _, _, _, ..] => Err(ParseOfDayError::MoreThanTwoColumns),
        }
    }
}

#[cfg(feature = "sexp")]
impl rsexp::UseToString for OfDay {}

#[cfg(feature = "with_serde")]
mod with_serde {
    use super::OfDay;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::str::FromStr;

    impl Serialize for OfDay {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            if serializer.is_human_readable() {
                self.to_string().serialize(serializer)
            } else {
                serializer.serialize_i64(self.0)
            }
        }
    }

    impl<'de> Deserialize<'de> for OfDay {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            if deserializer.is_human_readable() {
                let s = String::deserialize(deserializer)?;
                OfDay::from_str(&s).map_err(serde::de::Error::custom)
            } else {
                let v = i64::deserialize(deserializer)?;
                Ok(OfDay(v))
            }
        }
    }
}
