use std::ops::{Add, Sub};

// Same representation as OCaml Core.Date.t, i.e.
// 2 bytes year, 1 byte month, 1 byte day
// https://github.com/janestreet/core_kernel/blob/4244b42cac7d1ba834c93bdeda2e29bc7ecfa9aa/core/src/date0.ml
/// Represents a date.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(u32);

#[cfg(feature = "binio")]
mod binio {
    use std::convert::TryFrom;
    impl binprot::BinProtRead for crate::Date {
        fn binprot_read<R>(r: &mut R) -> Result<Self, binprot::Error>
        where
            R: std::io::Read + ?Sized,
        {
            let year: i64 = binprot::BinProtRead::binprot_read(r)?;
            let year = u32::try_from(year)?;
            let month: crate::Month = binprot::BinProtRead::binprot_read(r)?;
            let day: i64 = binprot::BinProtRead::binprot_read(r)?;
            let day = u8::try_from(day)?;
            match Self::create(year, month, day) {
                Ok(date) => Ok(date),
                Err(err) => Err(binprot::Error::CustomError(Box::new(err))),
            }
        }
    }

    impl binprot::BinProtWrite for crate::Date {
        fn binprot_write<W>(&self, w: &mut W) -> Result<(), std::io::Error>
        where
            W: std::io::Write,
        {
            binprot::BinProtWrite::binprot_write(&(self.year() as i64), w)?;
            binprot::BinProtWrite::binprot_write(&self.month(), w)?;
            binprot::BinProtWrite::binprot_write(&(self.day() as i64), w)?;
            Ok(())
        }
    }
}

#[cfg(feature = "sexp")]
mod sexp {
    use std::str::FromStr;
    impl rsexp::SexpOf for crate::Date {
        fn sexp_of(&self) -> rsexp::Sexp {
            rsexp::SexpOf::sexp_of(&self.to_string())
        }
    }

    impl rsexp::OfSexp for crate::Date {
        fn of_sexp(sexp: &rsexp::Sexp) -> Result<Self, rsexp::IntoSexpError> {
            match sexp {
                rsexp::Sexp::Atom(a) => {
                    crate::Date::from_str(std::str::from_utf8(a).map_err(|err| {
                        let err = format!("{}", err);
                        rsexp::IntoSexpError::StringConversionError { err }
                    })?)
                    .map_err(|err| {
                        let err = format!("{}", err);
                        rsexp::IntoSexpError::StringConversionError { err }
                    })
                }
                rsexp::Sexp::List(list) => Err(rsexp::IntoSexpError::ExpectedAtomGotList {
                    type_: "date",
                    list_len: list.len(),
                }),
            }
        }
    }
}

pub const fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DayOfWeek {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

impl DayOfWeek {
    pub fn of_u8(i: u8) -> Option<DayOfWeek> {
        match i {
            0 => Some(Self::Sun),
            1 => Some(Self::Mon),
            2 => Some(Self::Tue),
            3 => Some(Self::Wed),
            4 => Some(Self::Thu),
            5 => Some(Self::Fri),
            6 => Some(Self::Sat),
            _ => None,
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            Self::Sun => 0,
            Self::Mon => 1,
            Self::Tue => 2,
            Self::Wed => 3,
            Self::Thu => 4,
            Self::Fri => 5,
            Self::Sat => 6,
        }
    }

    pub fn is_weekday(self) -> bool {
        match self {
            Self::Mon | Self::Tue | Self::Wed | Self::Thu | Self::Fri => true,
            Self::Sat | Self::Sun => false,
        }
    }

    pub fn is_weekend(self) -> bool {
        !self.is_weekday()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "binio", derive(binprot_derive::BinProtRead, binprot_derive::BinProtWrite))]
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

impl std::fmt::Debug for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year(), self.month_int(), self.day())
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    /// The year for this date.
    pub fn year(self) -> u32 {
        self.0 >> 16
    }

    /// The month for this date as an int, between 1 and 12.
    pub fn month_int(self) -> u8 {
        ((self.0 >> 8) & 255) as u8
    }

    /// The month for this date.
    pub fn month(self) -> Month {
        Month::of_u8(self.month_int()).expect("not a proper month")
    }

    /// The day of the month for this date, as an int between 1 and 31.
    pub fn day(self) -> u8 {
        (self.0 & 255) as u8
    }

    /// Create a date based on a year, a month, and a day.
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

    /// String representation for the current date, e.g. "2021-01-16".
    pub fn to_string_iso8601_extended(self) -> String {
        self.to_string()
    }

    /// String representation for the current date, e.g. "20210116".
    pub fn to_string_iso8601_basic(self) -> String {
        format!("{:04}{:02}{:02}", self.year(), self.month_int(), self.day())
    }

    /// String representation for the current date, e.g. "01/16/2021".
    pub fn to_string_american(self) -> String {
        format!("{:02}/{:02}/{:04}", self.month_int(), self.day(), self.year())
    }

    pub const UNIX_EPOCH: Self = match Self::create(1970, Month::Jan, 1) {
        Ok(date) => date,
        Err(_) => {
            /* Panic here when const panics are stable. */
            Date(0)
        }
    };

    /// Add a number of days to this date.
    pub fn add_days(self, n: i32) -> Self {
        self + n
    }

    /// Add some number of months to a date.
    /// This returns the date with the last day of the month in the case where the
    /// actual date would be invalid, e.g. adding a month to Jan 31 results in Feb 28
    /// or Feb 29 depending on whether it's a leap year or not.
    pub fn add_months(self, m: i32) -> Self {
        let total_months = self.month_int() as i32 + m;
        let diff_y = if total_months < 0 { (total_months + 1) / 12 - 1 } else { total_months / 12 };
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

    /// Add some number of years to a date.
    /// If the returned day would not exist (e.g. Feb 28), the last day of the
    /// target month is returned.
    pub fn add_years(self, y: i32) -> Self {
        self.add_months(y * 12)
    }

    const DAYOFWEEK_TABLE: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

    /// The day of the week for this date.
    pub fn day_of_week(self) -> DayOfWeek {
        let m = self.month_int();
        let y = if m < 3 { self.year() - 1 } else { self.year() };
        let y = y as i32;
        let d = (y + y / 4 - y / 100
            + y / 400
            + Self::DAYOFWEEK_TABLE[(m - 1) as usize]
            + self.day() as i32)
            % 7;
        DayOfWeek::of_u8(d as u8).unwrap()
    }

    /// The number of days since epoch (1970-01-01).
    pub fn of_days_since_epoch(d: i32) -> Self {
        Self::UNIX_EPOCH + d
    }

    /// The current date in the given timezone, this calls Time::now.
    pub fn today(tz: crate::Tz) -> Self {
        crate::Time::now().to_date(tz)
    }

    pub fn is_weekday(self) -> bool {
        self.day_of_week().is_weekday()
    }

    pub fn is_weekend(self) -> bool {
        self.day_of_week().is_weekend()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn day_of_week(self) -> DayOfWeek {
        DayOfWeek::of_u8(((self.0 + 3) % 7) as u8).unwrap()
    }

    pub fn is_weekday(self) -> bool {
        self.day_of_week().is_weekday()
    }

    pub fn is_weekend(self) -> bool {
        self.day_of_week().is_weekend()
    }
}

impl std::fmt::Debug for Days {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_date() {
            Ok(date) => std::fmt::Debug::fmt(&date, f),
            Err(err) => std::fmt::Debug::fmt(&err, f),
        }
    }
}

impl std::fmt::Display for Days {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
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

pub struct DatesBetween {
    current_day: Days,
    last_day: Days,
}

impl Iterator for DatesBetween {
    type Item = Date;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_day <= self.last_day {
            let res = self.current_day.to_date().unwrap();
            self.current_day = self.current_day + 1;
            Some(res)
        } else {
            None
        }
    }
}

impl Date {
    /// List all the dates between two dates (inclusive).
    pub fn dates_between(lo: Self, up: Self) -> DatesBetween {
        DatesBetween { current_day: Days::of_date(lo), last_day: Days::of_date(up) }
    }

    pub fn dates_until(self, up: Self) -> DatesBetween {
        Self::dates_between(self, up)
    }
}

pub struct WeekdaysBetween {
    current_day: Days,
    last_day: Days,
}

impl Iterator for WeekdaysBetween {
    type Item = Date;
    fn next(&mut self) -> Option<Self::Item> {
        while self.current_day <= self.last_day {
            if self.current_day.is_weekday() {
                let res = self.current_day.to_date().unwrap();
                self.current_day = self.current_day + 1;
                return Some(res);
            }
            self.current_day = self.current_day + 1;
        }
        None
    }
}

impl Date {
    /// List all the weekdays between two dates (inclusive).
    pub fn weekdays_between(lo: Self, up: Self) -> WeekdaysBetween {
        WeekdaysBetween { current_day: Days::of_date(lo), last_day: Days::of_date(up) }
    }

    pub fn weekdays_until(self, up: Self) -> WeekdaysBetween {
        Self::weekdays_between(self, up)
    }
}
