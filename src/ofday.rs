#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "binio", derive(binprot_derive::BinProtRead, binprot_derive::BinProtWrite))]
pub struct OfDay(i64);

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum OfDayError {
    InvalidHour(u8),
    InvalidMinute(u8),
    InvalidSecond(u8),
    InvalidNanosecond(u32),
}

impl std::fmt::Display for OfDayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for OfDayError {}

impl OfDay {
    // TODO: validate or clamp?
    pub fn of_ns_since_midnight(i: i64) -> Self {
        Self(i)
    }

    pub fn to_ns_since_midnight(self) -> i64 {
        self.0
    }

    pub fn since_midnight(self) -> crate::Span {
        crate::Span::of_int_ns(self.to_ns_since_midnight())
    }

    pub fn hour(self) -> i64 {
        self.0 / 1_000_000_000 / 60 / 60
    }

    pub fn minute(self) -> i64 {
        (self.0 / 1_000_000_000 / 60) % 60
    }

    pub fn second(self) -> i64 {
        (self.0 / 1_000_000_000) % 60
    }

    pub fn nanosecond(self) -> i64 {
        self.0 % 1_000_000_000
    }

    pub fn create(hour: u8, minute: u8, second: u8, nanosecond: u32) -> Result<Self, OfDayError> {
        if hour >= 24 {
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
}

impl std::fmt::Display for OfDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let hr = self.hour();
        let min = self.minute();
        let sec = self.second();
        let ns = self.nanosecond();
        if ns == 0 {
            write!(f, "{:02}:{:02}:{:02}", hr, min, sec)
        } else {
            let mut ns = ns;
            let mut ns_width = 9;
            while ns % 10 == 0 {
                ns /= 10;
                ns_width -= 1;
            }
            write!(f, "{:02}:{:02}:{:02}.{:0ns_width$}", hr, min, sec, ns, ns_width = ns_width)
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
    NoColumn,
    MoreThanTwoColumns,
    MoreThanOneDot,
    InvalidNanosecondString(String),
    ParseIntError(std::num::ParseIntError),
    OfDayError(OfDayError),
}

impl std::fmt::Display for ParseOfDayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
            [] | [_] => Err(ParseOfDayError::NoColumn),
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
mod sexp {
    use std::str::FromStr;
    impl rsexp::SexpOf for crate::OfDay {
        fn sexp_of(&self) -> rsexp::Sexp {
            rsexp::SexpOf::sexp_of(&self.to_string())
        }
    }

    impl rsexp::OfSexp for crate::OfDay {
        fn of_sexp(sexp: &rsexp::Sexp) -> Result<Self, rsexp::IntoSexpError> {
            match sexp {
                rsexp::Sexp::Atom(a) => {
                    crate::OfDay::from_str(std::str::from_utf8(a).map_err(|err| {
                        let err = format!("{}", err);
                        rsexp::IntoSexpError::StringConversionError { err }
                    })?)
                    .map_err(|err| {
                        let err = format!("{}", err);
                        rsexp::IntoSexpError::StringConversionError { err }
                    })
                }
                rsexp::Sexp::List(list) => Err(rsexp::IntoSexpError::ExpectedAtomGotList {
                    type_: "ofday",
                    list_len: list.len(),
                }),
            }
        }
    }
}
