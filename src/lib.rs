extern crate chrono;
extern crate chrono_tz;

mod date;
pub use date::{Date, DateError, DayOfWeek, Month};

mod timezone;
pub use timezone::{TzInfo, TzOffset};

mod span;
pub use span::SpanNs;

#[cfg(feature = "binio")]
extern crate binprot;
#[cfg(feature = "binio")]
extern crate binprot_derive;
#[cfg(feature = "binio")]
use binprot_derive::{BinProtRead, BinProtWrite};

use chrono::{TimeZone, Timelike};
use std::ops::{Add, Rem, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "binio", derive(BinProtRead, BinProtWrite))]
pub struct TimeNs(i64);

impl Add<SpanNs> for TimeNs {
    type Output = Self;

    fn add(self, other: SpanNs) -> Self {
        Self(self.0 + other.to_int_ns())
    }
}

impl Sub<SpanNs> for TimeNs {
    type Output = Self;

    fn sub(self, other: SpanNs) -> Self {
        Self(self.0 - other.to_int_ns())
    }
}

impl Sub for TimeNs {
    type Output = SpanNs;

    fn sub(self, other: Self) -> SpanNs {
        SpanNs::of_int_ns(self.0 - other.0)
    }
}

impl Rem<SpanNs> for TimeNs {
    type Output = Self;

    fn rem(self, other: SpanNs) -> Self {
        Self(self.0 % other.to_int_ns())
    }
}

impl TimeNs {
    pub const EPOCH: Self = Self(0);

    pub fn to_span_since_epoch(self) -> SpanNs {
        SpanNs::of_int_ns(self.0)
    }

    pub fn of_span_since_epoch(span: SpanNs) -> Self {
        Self(span.to_int_ns())
    }

    // TODO: Add OfDay and use it rather than SpanNs
    pub fn to_date_ofday(self, tz_info: &TzInfo) -> (Date, SpanNs) {
        let offset = tz_info.offset(self);
        let ns_since_epoch = self.0 + offset.to_int_ns();
        let day_ns = SpanNs::DAY.to_int_ns();
        let days = ns_since_epoch.div_euclid(day_ns);
        let ofday = ns_since_epoch.rem_euclid(day_ns);
        let date = Date::of_days_since_epoch(days as i32);
        (date, SpanNs::of_int_ns(ofday))
    }

    pub fn to_date(self, tz_info: &TzInfo) -> Date {
        let offset = tz_info.offset(self);
        let ns_since_epoch = self.0 + offset.to_int_ns();
        let days = ns_since_epoch.div_euclid(SpanNs::DAY.to_int_ns());
        Date::of_days_since_epoch(days as i32)
    }

    pub fn to_naive_datetime(self) -> chrono::NaiveDateTime {
        let day_ns = SpanNs::DAY.to_int_ns();
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
