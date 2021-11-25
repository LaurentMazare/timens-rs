extern crate chrono;
extern crate chrono_tz;

mod date;
pub use date::{Date, DateError, DayOfWeek, Month};

mod timezone;
pub use timezone::{TzError, TzInfo, TzOffset};

mod span;
pub use span::Span;

mod ofday;
pub use ofday::OfDay;

mod timezone_data;
pub use timezone_data::Tz;

#[cfg(feature = "binio")]
extern crate binprot;
#[cfg(feature = "binio")]
extern crate binprot_derive;
#[cfg(feature = "binio")]
use binprot_derive::{BinProtRead, BinProtWrite};

use chrono::{TimeZone, Timelike};
use std::ops::{Add, Rem, Sub};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "binio", derive(BinProtRead, BinProtWrite))]
pub struct Time(i64);

impl std::fmt::Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let day_ns = Span::DAY.to_int_ns();
        let days = self.0.div_euclid(day_ns);
        let ofday = self.0.rem_euclid(day_ns);
        let date = Date::of_days_since_epoch(days as i32);
        write!(f, "{} {}Z", date, OfDay::of_ns_since_midnight(ofday))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimeParseError {
    NoSpace,
    DateError(date::DateError),
    OfDayError(ofday::ParseOfDayError),
    NoZone,
}

impl std::fmt::Display for TimeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for TimeParseError {}

impl std::convert::From<date::DateError> for TimeParseError {
    fn from(date_error: date::DateError) -> Self {
        TimeParseError::DateError(date_error)
    }
}

impl std::convert::From<ofday::ParseOfDayError> for TimeParseError {
    fn from(ofday_error: ofday::ParseOfDayError) -> Self {
        TimeParseError::OfDayError(ofday_error)
    }
}

impl std::str::FromStr for Time {
    type Err = TimeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            None => Err(TimeParseError::NoSpace),
            Some((date, ofday_with_zone)) => {
                let date = Date::from_str(date)?;
                // TODO: Support other zones.
                match ofday_with_zone.split_once('Z') {
                    Some((ofday, z)) if z.is_empty() => {
                        let ofday = OfDay::from_str(ofday)?;
                        Ok(Self::of_date_ofday_gmt(date, ofday))
                    }
                    Some(_) | None => Err(TimeParseError::NoZone),
                }
            }
        }
    }
}

impl Add<Span> for Time {
    type Output = Self;

    fn add(self, other: Span) -> Self {
        Self(self.0 + other.to_int_ns())
    }
}

impl Sub<Span> for Time {
    type Output = Self;

    fn sub(self, other: Span) -> Self {
        Self(self.0 - other.to_int_ns())
    }
}

impl Sub for Time {
    type Output = Span;

    fn sub(self, other: Self) -> Span {
        Span::of_int_ns(self.0 - other.0)
    }
}

impl Rem<Span> for Time {
    type Output = Self;

    fn rem(self, other: Span) -> Self {
        Self(self.0 % other.to_int_ns())
    }
}

impl Time {
    pub const EPOCH: Self = Self(0);

    pub fn to_span_since_epoch(self) -> Span {
        Span::of_int_ns(self.0)
    }

    pub fn of_span_since_epoch(span: Span) -> Self {
        Self(span.to_int_ns())
    }

    pub fn to_date_ofday(self, tz: Tz) -> (Date, OfDay) {
        let offset = tz.tz_info().offset(self);
        let ns_since_epoch = self.0 + offset.to_int_ns();
        let day_ns = Span::DAY.to_int_ns();
        let days = ns_since_epoch.div_euclid(day_ns);
        let ofday = ns_since_epoch.rem_euclid(day_ns);
        let date = Date::of_days_since_epoch(days as i32);
        (date, OfDay::of_ns_since_midnight(ofday))
    }

    pub fn to_date(self, tz: Tz) -> Date {
        let offset = tz.tz_info().offset(self);
        let ns_since_epoch = self.0 + offset.to_int_ns();
        let days = ns_since_epoch.div_euclid(Span::DAY.to_int_ns());
        Date::of_days_since_epoch(days as i32)
    }

    pub fn of_date_ofday(date: Date, ofday: OfDay, tz: Tz) -> Result<Self, TzError> {
        tz.tz_info().date_ofday_to_time(date, ofday)
    }

    pub fn of_date_ofday_gmt(date: Date, ofday: OfDay) -> Self {
        let gmt_ns = (date - Date::UNIX_EPOCH) as i64 * Span::DAY.to_int_ns();
        Time(gmt_ns + ofday.to_ns_since_midnight())
    }

    pub fn to_string_gmt(self) -> String {
        format!("{:?}", self)
    }

    pub fn write_tz<W: std::fmt::Write>(self, w: &mut W, tz: Tz) -> Result<(), std::fmt::Error> {
        let offset_sec = tz.tz_info().find(self).total_offset_sec();
        let ns_since_epoch = self.0 + offset_sec as i64 * Span::SEC.to_int_ns();
        let day_ns = Span::DAY.to_int_ns();
        let days = ns_since_epoch.div_euclid(day_ns);
        let ofday = OfDay::of_ns_since_midnight(ns_since_epoch.rem_euclid(day_ns));
        let date = Date::of_days_since_epoch(days as i32);
        if offset_sec == 0 {
            write!(w, "{} {}Z", date, ofday)
        } else {
            let (abs_offset, sign) =
                if offset_sec < 0 { (-offset_sec, '-') } else { (offset_sec, '+') };
            let offset_sec = abs_offset % 60;
            let abs_offset = abs_offset / 60;
            let offset_min = abs_offset % 60;
            let offset_hr = abs_offset / 60;
            write!(w, "{} {}{}{:02}:{:02}", date, ofday, sign, offset_hr, offset_min)?;
            if offset_sec != 0 {
                write!(w, ":{:02}", offset_sec)?;
            }
            Ok(())
        }
    }

    pub fn to_string_tz(self, tz: Tz) -> String {
        let mut s = String::new();
        self.write_tz(&mut s, tz).unwrap();
        s
    }

    pub fn to_naive_datetime(self) -> chrono::NaiveDateTime {
        let day_ns = Span::DAY.to_int_ns();
        let sec = self.0.div_euclid(day_ns);
        let ns = self.0.rem_euclid(day_ns);
        chrono::NaiveDateTime::from_timestamp(sec, ns as u32)
    }

    pub fn to_datetime(self, tz: &chrono_tz::Tz) -> Option<chrono::DateTime<chrono_tz::Tz>> {
        match chrono_tz::UTC.from_local_datetime(&self.to_naive_datetime()) {
            chrono::LocalResult::None | chrono::LocalResult::Ambiguous(_, _) => None,
            chrono::LocalResult::Single(t) => Some(t.with_timezone(tz)),
        }
    }

    pub fn to_ofday_string_no_trailing_zeros(self, tz: &chrono_tz::Tz) -> String {
        match self.to_datetime(tz) {
            None => format!("unable to format for timezone {} {:?}", self.to_naive_datetime(), tz),
            Some(t) => {
                let t = t.time();
                let hr = t.hour();
                let min = t.minute();
                let sec = t.second();
                let ns = t.nanosecond();
                if ns == 0 {
                    format!("{:02}:{:02}:{:02}", hr, min, sec)
                } else {
                    let mut ns = ns;
                    let mut ns_width = 9;
                    while ns % 10 == 0 {
                        ns /= 10;
                        ns_width -= 1;
                    }
                    format!(
                        "{:02}:{:02}:{:02}.{:0ns_width$}",
                        hr,
                        min,
                        sec,
                        ns,
                        ns_width = ns_width
                    )
                }
            }
        }
    }
}
