extern crate chrono;
extern crate chrono_tz;

#[cfg(feature = "binprot")]
use {
    binprot::BinProtRead,
    binprot_derive::{BinProtRead, BinProtWrite},
};

use chrono::{TimeZone, Timelike};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

// Same representation as OCaml Core.Date.t, i.e.
// 2 bytes year, 1 byte month, 1 byte day
// https://github.com/janestreet/core_kernel/blob/4244b42cac7d1ba834c93bdeda2e29bc7ecfa9aa/core/src/date0.ml
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "binio", derive(BinProtRead, BinProtWrite))]
pub struct Date(u32);

pub const fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl Month {
    pub const fn days_in_month(self, year: u32) -> u8 {
        match self {
            Self::Jan | Self::Mar | Self::May | Self::Jul | Self::Aug | Self::Oct | Self::Dec => 31,
            Self::Apr | Self::Jun | Self::Sep | Self::Nov => 30,
            Self::Feb => {
                if is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            Self::Jan => 1,
            Self::Feb => 2,
            Self::Mar => 3,
            Self::Apr => 4,
            Self::May => 5,
            Self::Jun => 6,
            Self::Jul => 7,
            Self::Aug => 8,
            Self::Sep => 9,
            Self::Oct => 10,
            Self::Nov => 11,
            Self::Dec => 12,
        }
    }

    pub fn of_u8(m: u8) -> Option<Self> {
        match m {
            1 => Some(Self::Jan),
            2 => Some(Self::Feb),
            3 => Some(Self::Mar),
            4 => Some(Self::Apr),
            5 => Some(Self::May),
            6 => Some(Self::Jun),
            7 => Some(Self::Jul),
            8 => Some(Self::Aug),
            9 => Some(Self::Sep),
            10 => Some(Self::Oct),
            11 => Some(Self::Nov),
            12 => Some(Self::Dec),
            _ => None,
        }
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}",
            self.year(),
            self.month_int(),
            self.day()
        )
    }
}

#[derive(Debug)]
pub enum DateError {
    InvalidYear(u32),
    InvalidDayForMonth(u32, Month, u8),
    ParseIntError,
    ParseMonthError,
    ParseError,
}

impl std::fmt::Display for DateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DateError {}

impl Date {
    pub fn year(self) -> u32 {
        self.0 >> 16
    }

    pub fn month_int(self) -> u8 {
        ((self.0 >> 8) & 255) as u8
    }

    pub fn month(self) -> Month {
        Month::of_u8(self.month_int()).expect("not a proper month")
    }

    pub fn day(self) -> u8 {
        (self.0 & 255) as u8
    }

    pub const fn create(year: u32, month: Month, day: u8) -> Result<Self, DateError> {
        if year > 9999 {
            return Err(DateError::InvalidYear(year));
        }
        if day == 0 || day > month.days_in_month(year) {
            return Err(DateError::InvalidDayForMonth(year, month, day));
        }
        let month_as_int = match month {
            Month::Jan => 1,
            Month::Feb => 2,
            Month::Mar => 3,
            Month::Apr => 4,
            Month::May => 5,
            Month::Jun => 6,
            Month::Jul => 7,
            Month::Aug => 8,
            Month::Sep => 9,
            Month::Oct => 10,
            Month::Nov => 11,
            Month::Dec => 12,
        };
        Ok(Date((year << 16) | (month_as_int << 8) | day as u32))
    }

    pub fn to_string_iso8601_extended(self) -> String {
        self.to_string()
    }

    pub fn to_string_iso8601_basic(self) -> String {
        format!("{:04}{:02}{:02}", self.year(), self.month_int(), self.day())
    }

    pub fn to_string_american(self) -> String {
        format!(
            "{:02}/{:02}/{:04}",
            self.month_int(),
            self.day(),
            self.year()
        )
    }

    pub const UNIX_EPOCH: Self = match Self::create(1970, Month::Jan, 1) {
        Ok(date) => date,
        Err(_) => {
            /* Panic here when const panics are stable. */
            Date(0)
        }
    };

    pub fn add_days(self, n: i32) -> Self {
        self + n
    }

    /// Add some number of months to a date.
    /// This returns the date with the last day of the month in the case where the
    /// actual date would be invalid, e.g. adding a month to Jan 31 results in Feb 28
    /// or Feb 29 depending on whether it's a leap year or not.
    pub fn add_months(self, m: i32) -> Self {
        let total_months = self.month_int() as i32 + m;
        let diff_y = if total_months < 0 {
            (total_months + 1) / 12 - 1
        } else {
            total_months / 12
        };
        let y = (self.year() as i32 + diff_y) as u32;
        let m = total_months % 12;
        let (y, m) = match m.cmp(&0) {
            std::cmp::Ordering::Equal => (y - 1, 12),
            std::cmp::Ordering::Less => (y, m + 12),
            std::cmp::Ordering::Greater => (y, m),
        };
        let m = Month::of_u8(m as u8).unwrap();
        let mut d = self.day();
        loop {
            if let Ok(date) = Self::create(y, m, d) {
                return date;
            }
            d -= 1;
        }
    }

    pub fn add_years(self, y: i32) -> Self {
        self.add_months(y * 12)
    }
}

pub struct Days(i32);

impl Days {
    fn of_year(y: i32) -> Self {
        Days(365 * y + y / 4 + y / 400 - y / 100)
    }

    pub fn of_date(date: Date) -> Days {
        let m = (date.month_int() as i32 + 9) % 12;
        let y = date.year() as i32 - m / 10;
        Days(Self::of_year(y).0 + (((m * 306) + 5) / 10) + date.day() as i32 - 1)
    }

    pub fn to_date(self) -> Result<Date, DateError> {
        let y = ((self.0 as i64 * 10_000 + 14_780) / 3_652_425) as i32;
        let ddd = self.0 - Self::of_year(y).0 as i32;
        let (y, ddd) = if ddd < 0 {
            let y = y - 1;
            (y, self.0 - Self::of_year(y).0)
        } else {
            (y, ddd)
        };
        let mi = ((100 * ddd) + 52) / 3_060;
        let y = y + ((mi + 2) / 12);
        let m = ((mi + 2) % 12) + 1;
        let d = ddd - (((mi * 306) + 5) / 10) + 1;
        Date::create(y as u32, Month::of_u8(m as u8).unwrap(), d as u8)
    }
}

impl std::convert::From<std::num::ParseIntError> for DateError {
    fn from(_: std::num::ParseIntError) -> Self {
        DateError::ParseIntError
    }
}

impl std::str::FromStr for Date {
    type Err = DateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        match parts[..] {
            [y, m, d] => {
                let y = u32::from_str(y)?;
                let d = u8::from_str(d)?;
                match Month::of_u8(u8::from_str(m)?) {
                    Some(m) => Date::create(y, m, d),
                    None => Err(DateError::ParseMonthError),
                }
            }
            _ => Err(DateError::ParseError),
        }
    }
}

impl Add<i32> for Days {
    type Output = Days;

    fn add(self, other: i32) -> Self::Output {
        Self(self.0 + other)
    }
}

impl Sub<i32> for Days {
    type Output = Days;

    fn sub(self, other: i32) -> Self::Output {
        Self(self.0 - other)
    }
}

impl Sub for Days {
    type Output = i32;

    fn sub(self, other: Self) -> Self::Output {
        self.0 - other.0
    }
}

impl Add<i32> for Date {
    type Output = Date;

    fn add(self, other: i32) -> Self::Output {
        let days = Days::of_date(self);
        (days + other).to_date().unwrap()
    }
}

impl Sub<i32> for Date {
    type Output = Date;

    fn sub(self, other: i32) -> Self::Output {
        let days = Days::of_date(self);
        (days - other).to_date().unwrap()
    }
}

impl Sub for Date {
    type Output = i32;

    fn sub(self, other: Self) -> Self::Output {
        Days::of_date(self) - Days::of_date(other)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "binio", derive(BinProtRead, BinProtWrite))]
pub struct TimeNs(i64);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "binio", derive(BinProtRead, BinProtWrite))]
pub struct SpanNs(i64);

macro_rules! span_conv {
    ($to_fn:ident, $of_fn_int:ident, $of_fn: ident, $cst: ident, $e: expr) => {
        pub const $cst: Self = Self($e);

        pub fn $to_fn(self) -> f64 {
            self.0 as f64 / $e as f64
        }

        pub fn $of_fn_int(i: i64) -> Self {
            Self((i * $e) as i64)
        }

        pub fn $of_fn(f: f64) -> Self {
            Self((f * $e as f64) as i64)
        }
    };
}

impl SpanNs {
    pub const ZERO: Self = Self(0);

    span_conv!(to_ns, of_int_ns, of_ns, NS, 1);
    span_conv!(to_us, of_int_us, of_us, US, 1_000);
    span_conv!(to_ms, of_int_ms, of_ms, MS, 1_000_000);
    span_conv!(to_sec, of_int_sec, of_sec, SEC, 1_000_000_000);
    span_conv!(to_min, of_int_min, of_min, MIN, 60_000_000_000i64);
    span_conv!(to_hr, of_int_hr, of_hr, HR, 3_600_000_000_000i64);
    span_conv!(to_day, of_int_day, of_day, DAY, 24 * 3_600_000_000_000i64);

    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    pub fn is_positive(self) -> bool {
        self.0 > 0
    }

    pub fn is_non_negative(self) -> bool {
        self.0 >= 0
    }

    pub fn is_negative(self) -> bool {
        self.0 < 0
    }

    pub fn is_non_positive(self) -> bool {
        self.0 <= 0
    }
}

fn remove_trailing_zeros(value: i64, max_digits: usize) -> (i64, usize) {
    let mut max_digits = max_digits;
    let mut value = value;
    while value % 10 == 0 {
        value /= 10;
        max_digits -= 1;
    }
    (value, max_digits)
}

impl std::fmt::Display for SpanNs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let ns = self.0;
        if ns == 0 {
            write!(f, "0s")?
        } else {
            if ns < 0 {
                write!(f, "-")?
            }
            let mut ns = ns.abs();
            if ns >= Self::DAY.0 {
                let days = ns / Self::DAY.0;
                write!(f, "{}d", days)?;
                ns -= days * Self::DAY.0;
            }
            if ns >= Self::HR.0 {
                let hr = ns / Self::HR.0;
                write!(f, "{}h", hr)?;
                ns -= hr * Self::HR.0;
            }
            if ns >= Self::MIN.0 {
                let min = ns / Self::MIN.0;
                write!(f, "{}m", min)?;
                ns -= min * Self::MIN.0;
            }
            if ns == 0 {
            } else if ns < 1_000 {
                write!(f, "{}ns", ns)?
            } else if ns < 1_000_000 {
                write!(f, "{}", ns / 1000)?;
                let rem_ns = ns % 1000;
                if rem_ns != 0 {
                    let (rem_ns, width) = remove_trailing_zeros(rem_ns, 3);
                    write!(f, ".{:0width$}", rem_ns, width = width)?;
                }
                write!(f, "us")?;
            } else if ns < 1_000_000_000 {
                write!(f, "{}", ns / 1_000_000)?;
                let rem_ns = ns % 1_000_000;
                if rem_ns != 0 {
                    let (rem_ns, width) = remove_trailing_zeros(rem_ns, 6);
                    write!(f, ".{:0width$}", rem_ns, width = width)?;
                }
                write!(f, "ms")?;
            } else {
                write!(f, "{}", ns / 1_000_000_000)?;
                let rem_ns = ns % 1_000_000_000;
                if rem_ns != 0 {
                    let (rem_ns, width) = remove_trailing_zeros(rem_ns, 9);
                    write!(f, ".{:0width$}", rem_ns, width = width)?;
                }
                write!(f, "s")?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for SpanNs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::Display::fmt(self, f)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseSpanError {
    EmptyString,
    UnexpectedChar(String, char),
    UnexpectedCharAfterU(String),
    UnexpectedCharAfterN(String),
}

impl std::str::FromStr for SpanNs {
    type Err = ParseSpanError;

    // TODO: Maybe add a bit of validation to avoid 1d1d for example.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().peekable();
        let negative = match chars.peek() {
            None => return Err(ParseSpanError::EmptyString),
            Some('+') => {
                chars.next();
                false
            }
            Some('-') => {
                chars.next();
                true
            }
            Some(_) => false,
        };
        let mut res = SpanNs::ZERO;
        let mut value = 0;
        let mut frac_value = 0;
        let mut frac_digits = 1;
        let mut after_point = false;
        while let Some(c) = chars.next() {
            // d, h, m, s, ms, us, ns
            let should_init = match c {
                '.' => {
                    after_point = true;
                    false
                }
                'd' => {
                    res = res + SpanNs::DAY * value;
                    if frac_value != 0 {
                        res = res + SpanNs((SpanNs::DAY.0 * frac_value) / frac_digits)
                    }
                    true
                }
                'h' => {
                    res = res + SpanNs::HR * value;
                    if frac_value != 0 {
                        res = res + SpanNs((SpanNs::HR.0 * frac_value) / frac_digits)
                    }
                    true
                }
                'm' => {
                    match chars.next_if_eq(&'s') {
                        Some(_) => {
                            res = res + SpanNs::MS * value;
                            if frac_value != 0 {
                                res = res + SpanNs((SpanNs::MS.0 * frac_value) / frac_digits)
                            }
                        }
                        None => {
                            res = res + SpanNs::MIN * value;
                            if frac_value != 0 {
                                res = res + SpanNs((SpanNs::MIN.0 * frac_value) / frac_digits)
                            }
                        }
                    }
                    true
                }
                's' => {
                    res = res + SpanNs::SEC * value;
                    if frac_value != 0 {
                        res = res + SpanNs((SpanNs::SEC.0 * frac_value) / frac_digits)
                    }
                    true
                }
                'u' => match chars.next_if_eq(&'s') {
                    Some(_) => {
                        res = res + SpanNs::US * value;
                        if frac_value != 0 {
                            res = res + SpanNs((SpanNs::US.0 * frac_value) / frac_digits)
                        }
                        true
                    }
                    None => return Err(ParseSpanError::UnexpectedCharAfterU(s.to_string())),
                },
                'n' => match chars.next_if_eq(&'s') {
                    Some(_) => {
                        res = res + SpanNs::NS * value;
                        if frac_value * 2 > frac_digits {
                            res = res + SpanNs::NS;
                        }
                        true
                    }
                    None => return Err(ParseSpanError::UnexpectedCharAfterN(s.to_string())),
                },
                other => match other.to_digit(10) {
                    Some(digit) => {
                        if after_point {
                            frac_value = 10 * frac_value + digit as i64;
                            frac_digits *= 10;
                        } else {
                            value = 10 * value + digit as i64;
                        }
                        false
                    }
                    None => return Err(ParseSpanError::UnexpectedChar(s.to_string(), other)),
                },
            };
            if should_init {
                value = 0;
                frac_value = 0;
                frac_digits = 1;
                after_point = false
            }
        }
        if negative {
            res = res.neg();
        }
        Ok(res)
    }
}

impl Add for SpanNs {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Add<SpanNs> for TimeNs {
    type Output = Self;

    fn add(self, other: SpanNs) -> Self {
        Self(self.0 + other.0)
    }
}

impl Neg for SpanNs {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Sub for SpanNs {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Sub<SpanNs> for TimeNs {
    type Output = Self;

    fn sub(self, other: SpanNs) -> Self {
        Self(self.0 - other.0)
    }
}

impl Sub for TimeNs {
    type Output = SpanNs;

    fn sub(self, other: Self) -> SpanNs {
        SpanNs(self.0 - other.0)
    }
}

impl Mul<i64> for SpanNs {
    type Output = Self;

    fn mul(self, other: i64) -> Self {
        Self(self.0 * other)
    }
}

impl Mul<f64> for SpanNs {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self((self.0 as f64 * other) as i64)
    }
}

impl Rem for SpanNs {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self(self.0 % other.0)
    }
}

impl Rem<SpanNs> for TimeNs {
    type Output = Self;

    fn rem(self, other: SpanNs) -> Self {
        Self(self.0 % other.0)
    }
}

impl Div for SpanNs {
    type Output = f64;

    fn div(self, other: Self) -> f64 {
        self.0 as f64 / other.0 as f64
    }
}

impl Div<i64> for SpanNs {
    type Output = Self;

    fn div(self, other: i64) -> Self {
        Self(self.0 / other)
    }
}

impl Div<f64> for SpanNs {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self((self.0 as f64 / other) as i64)
    }
}

impl TimeNs {
    pub const EPOCH: Self = Self(0);

    pub fn to_span_since_epoch(self) -> SpanNs {
        SpanNs(self.0)
    }

    pub fn of_span_since_epoch(span: SpanNs) -> Self {
        Self(span.0)
    }

    pub fn to_naive_datetime(self) -> chrono::NaiveDateTime {
        let sec = self.0 / SpanNs::SEC.0;
        let ns = self.0 % SpanNs::SEC.0;
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
            None => format!(
                "unable to format for timezone {} {:?}",
                self.to_naive_datetime(),
                tz
            ),
            Some(t) => {
                let t = t.time();
                let hr = t.hour();
                let min = t.minute();
                let sec = t.second();
                let ns = t.nanosecond();
                if ns == 0 {
                    format!("{:02}:{:02}:{02}", hr, min, sec)
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
