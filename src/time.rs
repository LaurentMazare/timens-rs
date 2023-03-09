#[cfg(feature = "binio")]
use binprot::macros::{BinProtRead, BinProtWrite};

#[cfg(feature = "with-chrono")]
use chrono::{TimeZone, Timelike};

use crate::{date, ofday};
use crate::{Date, OfDay, Span, Tz, TzError, TzParseError};
use std::ops::{Add, AddAssign, Rem, Sub, SubAssign};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "binio", derive(BinProtRead, BinProtWrite))]
pub struct Time(i64);

impl std::fmt::Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let (date, ofday) = self.to_date_ofday_gmt();
        write!(f, "{date} {ofday}Z")
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimeParseError {
    NoSpace,
    DateError(date::DateError),
    OfDayError(ofday::ParseOfDayError),
    NoZone,
    ExpectedIntInZone(std::num::ParseIntError),
    TzError(TzError),
    TzParseError(TzParseError),
}

impl std::fmt::Display for TimeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for TimeParseError {}

impl std::convert::From<TzError> for TimeParseError {
    fn from(tz_error: TzError) -> Self {
        TimeParseError::TzError(tz_error)
    }
}

impl std::convert::From<TzParseError> for TimeParseError {
    fn from(tz_error: TzParseError) -> Self {
        TimeParseError::TzParseError(tz_error)
    }
}

impl std::convert::From<std::num::ParseIntError> for TimeParseError {
    fn from(int_error: std::num::ParseIntError) -> Self {
        TimeParseError::ExpectedIntInZone(int_error)
    }
}

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

fn parse_zone_offset(s: &str) -> Result<Span, TimeParseError> {
    match s.split(':').collect::<Vec<_>>()[..] {
        [] => Err(TimeParseError::NoZone),
        [hour] => {
            let hour = u8::from_str(hour)? as i64;
            Ok(Span::HR * hour)
        }
        [hour, minute] => {
            let hour = u8::from_str(hour)? as i64;
            let minute = u8::from_str(minute)? as i64;
            Ok(Span::HR * hour + Span::MIN * minute)
        }
        [hour, minute, second] => {
            let hour = u8::from_str(hour)? as i64;
            let minute = u8::from_str(minute)? as i64;
            let second = u8::from_str(second)? as i64;
            Ok(Span::HR * hour + Span::MIN * minute + Span::SEC * second)
        }
        _ => Err(TimeParseError::NoZone),
    }
}

impl Time {
    fn parse_ofday_with_zone(ofday_with_zone: &str, date: Date) -> Result<Self, TimeParseError> {
        match ofday_with_zone.split_once('Z') {
            Some((ofday, z)) if z.is_empty() => {
                let ofday = OfDay::from_str(ofday)?;
                return Ok(Self::of_date_ofday_gmt(date, ofday));
            }
            Some(_) | None => (),
        };
        if let Some((ofday, zone_offset)) = ofday_with_zone.split_once('+') {
            let ofday = OfDay::from_str(ofday)?;
            let zone_offset = parse_zone_offset(zone_offset)?;
            return Ok(Self::of_date_ofday_gmt(date, ofday) + zone_offset);
        }
        if let Some((ofday, zone_offset)) = ofday_with_zone.split_once('-') {
            let ofday = OfDay::from_str(ofday)?;
            let zone_offset = parse_zone_offset(zone_offset)?;
            return Ok(Self::of_date_ofday_gmt(date, ofday) - zone_offset);
        }
        if let Some((ofday, tz)) = ofday_with_zone.split_once(' ') {
            let ofday = OfDay::from_str(ofday)?;
            let tz = Tz::from_str(tz)?;
            return Ok(Self::of_date_ofday(date, ofday, tz)?);
        }
        Err(TimeParseError::NoZone)
    }
}

impl std::str::FromStr for Time {
    type Err = TimeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            None => Err(TimeParseError::NoSpace),
            Some((date, ofday_with_zone)) => {
                let date = Date::from_str(date)?;
                Self::parse_ofday_with_zone(ofday_with_zone, date)
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

impl AddAssign<Span> for Time {
    fn add_assign(&mut self, other: Span) {
        self.0 += other.to_int_ns()
    }
}

impl Sub<Span> for Time {
    type Output = Self;

    fn sub(self, other: Span) -> Self {
        Self(self.0 - other.to_int_ns())
    }
}

impl SubAssign<Span> for Time {
    fn sub_assign(&mut self, other: Span) {
        self.0 -= other.to_int_ns()
    }
}

impl Sub for Time {
    type Output = Span;

    fn sub(self, other: Self) -> Span {
        Span::of_int_ns(self.0 - other.0)
    }
}

impl Rem<Span> for Time {
    type Output = Span;

    fn rem(self, other: Span) -> Span {
        Span::of_int_ns(self.0 % other.to_int_ns())
    }
}

impl Time {
    pub const EPOCH: Self = Self(0);

    pub fn now() -> Self {
        let now = std::time::SystemTime::now();
        let dt = now.duration_since(std::time::UNIX_EPOCH).expect("system time before Unix epoch");
        let dt = Span::of_int_sec(dt.as_secs() as i64) + Span::of_int_ns(dt.subsec_nanos() as i64);
        Self::of_span_since_epoch(dt)
    }

    pub const fn to_span_since_epoch(self) -> Span {
        Span::of_int_ns(self.0)
    }

    pub const fn of_span_since_epoch(span: Span) -> Self {
        Self(span.to_int_ns())
    }

    pub const fn to_int_ns_since_epoch(self) -> i64 {
        self.0
    }

    pub const fn of_int_ns_since_epoch(ns: i64) -> Self {
        Self(ns)
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

    pub fn to_date_ofday_gmt(self) -> (Date, OfDay) {
        let day_ns = Span::DAY.to_int_ns();
        let days = self.0.div_euclid(day_ns);
        let ofday = self.0.rem_euclid(day_ns);
        let date = Date::of_days_since_epoch(days as i32);
        (date, OfDay::of_ns_since_midnight(ofday))
    }

    pub fn to_date(self, tz: Tz) -> Date {
        let offset = tz.tz_info().offset(self);
        let ns_since_epoch = self.0 + offset.to_int_ns();
        let days = ns_since_epoch.div_euclid(Span::DAY.to_int_ns());
        Date::of_days_since_epoch(days as i32)
    }

    pub fn to_ofday(self, tz: Tz) -> OfDay {
        let offset = tz.tz_info().offset(self);
        let ns_since_epoch = self.0 + offset.to_int_ns();
        let ofday = ns_since_epoch.rem_euclid(Span::DAY.to_int_ns());
        OfDay::of_ns_since_midnight(ofday)
    }

    pub fn of_date_ofday(date: Date, ofday: OfDay, tz: Tz) -> Result<Self, TzError> {
        tz.tz_info().date_ofday_to_time(date, ofday)
    }

    pub fn of_date_ofday_gmt(date: Date, ofday: OfDay) -> Self {
        let gmt_ns = (date - Date::UNIX_EPOCH) as i64 * Span::DAY.to_int_ns();
        Time(gmt_ns + ofday.to_ns_since_midnight())
    }

    pub fn to_string_gmt(self) -> String {
        format!("{self:?}")
    }

    pub fn write_tz<W: std::fmt::Write>(self, w: &mut W, tz: Tz) -> Result<(), std::fmt::Error> {
        let offset_sec = tz.tz_info().find(self).total_offset_sec();
        let ns_since_epoch = self.0 + offset_sec as i64 * Span::SEC.to_int_ns();
        let day_ns = Span::DAY.to_int_ns();
        let days = ns_since_epoch.div_euclid(day_ns);
        let ofday = OfDay::of_ns_since_midnight(ns_since_epoch.rem_euclid(day_ns));
        let date = Date::of_days_since_epoch(days as i32);
        if offset_sec == 0 {
            write!(w, "{date} {ofday}Z")
        } else {
            let (abs_offset, sign) =
                if offset_sec < 0 { (-offset_sec, '-') } else { (offset_sec, '+') };
            let offset_sec = abs_offset % 60;
            let abs_offset = abs_offset / 60;
            let offset_min = abs_offset % 60;
            let offset_hr = abs_offset / 60;
            write!(w, "{date} {ofday}{sign}{offset_hr:02}:{offset_min:02}")?;
            if offset_sec != 0 {
                write!(w, ":{offset_sec:02}")?;
            }
            Ok(())
        }
    }

    pub fn to_string_tz(self, tz: Tz) -> String {
        let mut s = String::new();
        self.write_tz(&mut s, tz).unwrap();
        s
    }

    pub const fn prev_multiple(self, rhs: Span) -> Self {
        Self::of_span_since_epoch(self.to_span_since_epoch().prev_multiple(rhs))
    }

    pub const fn next_multiple(self, rhs: Span) -> Self {
        Self::of_span_since_epoch(self.to_span_since_epoch().next_multiple(rhs))
    }
}

#[cfg(feature = "with-chrono")]
impl Time {
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

#[cfg(feature = "sexp")]
mod sexp {
    use rsexp::Sexp;
    impl rsexp::SexpOf for crate::Time {
        fn sexp_of(&self) -> Sexp {
            let (date, ofday) = self.to_date_ofday_gmt();
            let ofday = format!("{ofday}Z");
            rsexp::SexpOf::sexp_of(&(date.to_string(), ofday))
        }
    }

    impl rsexp::OfSexp for crate::Time {
        fn of_sexp(sexp: &Sexp) -> Result<Self, rsexp::IntoSexpError> {
            match sexp {
                Sexp::List(ref list) => match list[..] {
                    [Sexp::Atom(ref date), Sexp::Atom(ref ofday_with_zone)] => {
                        let date: crate::Date = String::from_utf8_lossy(date).parse().map_err(
                            |err: crate::DateError| rsexp::IntoSexpError::StringConversionError {
                                err: err.to_string(),
                            },
                        )?;
                        let time = Self::parse_ofday_with_zone(
                            &String::from_utf8_lossy(ofday_with_zone),
                            date,
                        )
                        .map_err(|err| {
                            rsexp::IntoSexpError::StringConversionError { err: err.to_string() }
                        })?;
                        Ok(time)
                    }
                    _ => Err(rsexp::IntoSexpError::ListLengthMismatch {
                        type_: "time",
                        list_len: list.len(),
                        expected_len: 2,
                    }),
                },
                Sexp::Atom(_) => Err(rsexp::IntoSexpError::ExpectedListGotAtom { type_: "time" }),
            }
        }
    }
}

#[cfg(feature = "with_serde")]
mod with_serde {
    use super::Time;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::str::FromStr;

    impl Serialize for Time {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            if serializer.is_human_readable() {
                self.to_string().serialize(serializer)
            } else {
                serializer.serialize_i64(self.0)
            }
        }
    }

    impl<'de> Deserialize<'de> for Time {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            if deserializer.is_human_readable() {
                let s = String::deserialize(deserializer)?;
                Time::from_str(&s).map_err(serde::de::Error::custom)
            } else {
                let v = i64::deserialize(deserializer)?;
                Ok(Time(v))
            }
        }
    }
}
