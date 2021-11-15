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

pub fn is_leap_year(year: u32) -> bool {
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
    pub fn days_in_month(self, year: u32) -> u8 {
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

    pub fn create(year: u32, month: Month, day: u8) -> Result<Self, DateError> {
        if year > 9999 {
            return Err(DateError::InvalidYear(year));
        }
        if day == 0 || day > month.days_in_month(year) {
            return Err(DateError::InvalidDayForMonth(year, month, day));
        }
        let month_as_int = match month {
            Month::Jan => 0,
            Month::Feb => 1,
            Month::Mar => 2,
            Month::Apr => 3,
            Month::May => 4,
            Month::Jun => 5,
            Month::Jul => 6,
            Month::Aug => 7,
            Month::Sep => 8,
            Month::Oct => 9,
            Month::Nov => 10,
            Month::Dec => 11,
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
