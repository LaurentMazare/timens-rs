extern crate chrono;
extern crate chrono_tz;

use chrono::{TimeZone, Timelike};
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeNs(i64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpanNs(i64);

macro_rules! span_conv {
    ($to_fn:ident, $of_fn: ident, $cst: ident, $e: expr) => {
        pub const $cst: Self = Self($e);

        pub fn $to_fn(self) -> f64 {
            self.0 as f64 / $e as f64
        }

        pub fn $of_fn(f: f64) -> Self {
            Self((f * $e as f64) as i64)
        }
    };
}

impl SpanNs {
    pub const ZERO: Self = Self(0);

    span_conv!(to_ns, of_ns, NS, 1);
    span_conv!(to_us, of_us, US, 1000);
    span_conv!(to_ms, of_ms, MS, 1_000_000);
    span_conv!(to_sec, of_sec, SEC, 1_000_000_000);
    span_conv!(to_min, of_min, MIN, 60_000_000_000i64);
    span_conv!(to_hr, of_hr, HR, 3_600_000_000_000i64);
    span_conv!(to_day, of_day, DAY, 24 * 3_600_000_000_000i64);

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
