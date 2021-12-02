use crate::{Date, OfDay, Span, Time};

#[derive(Copy, Clone)]
pub struct TzOffset {
    pub utc_offset: i32,
    pub dst_offset: i32,
}

#[derive(Clone)]
pub struct TzInfo {
    pub first: TzOffset,
    pub rest: &'static [(i64, TzOffset)],
}

impl TzOffset {
    pub const ZERO: TzOffset = TzOffset { utc_offset: 0, dst_offset: 0 };

    pub fn total_offset_sec(&self) -> i32 {
        self.utc_offset + self.dst_offset
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum TzError {
    NoTimeInThisTz,
    TwoTimesInThisTz(Time, Time),
}

impl std::fmt::Display for TzError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for TzError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TzParseError {
    UnknownZone(String),
}

impl std::fmt::Display for TzParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for TzParseError {}

impl TzInfo {
    pub fn find(&self, time: Time) -> &TzOffset {
        let sec = time.to_int_ns_since_epoch().div_euclid(Span::SEC.to_int_ns());
        let index = self.rest.partition_point(|&(start_sec, _)| sec >= start_sec);
        if index == 0 {
            &self.first
        } else {
            &self.rest[index - 1].1
        }
    }

    pub fn offset(&self, time: Time) -> Span {
        let fixed_timespan = self.find(time);
        Span::of_int_sec(fixed_timespan.total_offset_sec() as i64)
    }

    pub const GMT: TzInfo = TzInfo { first: TzOffset::ZERO, rest: &[] };

    fn valid_time(&self, gmt_sec: i64, nanosecond: i64, next_i: usize) -> Option<Time> {
        let (min_sec, tz_info) = if next_i == 0 {
            (i64::MIN, self.first)
        } else if next_i > self.rest.len() {
            return None;
        } else {
            self.rest[next_i - 1]
        };
        let sec = gmt_sec - tz_info.total_offset_sec() as i64;
        if sec >= min_sec && (self.rest.len() == next_i || sec < self.rest[next_i].0) {
            Some(crate::Time::of_int_ns_since_epoch(sec * Span::SEC.to_int_ns() + nanosecond))
        } else {
            None
        }
    }

    pub fn date_ofday_to_time(&self, date: Date, ofday: OfDay) -> Result<Time, TzError> {
        let gmt_ns = (date - Date::UNIX_EPOCH) as i64 * Span::DAY.to_int_ns();
        let gmt_ns = gmt_ns + ofday.to_ns_since_midnight();
        let gmt_sec = gmt_ns.div_euclid(Span::SEC.to_int_ns());
        let nanosecond = gmt_ns.rem_euclid(Span::SEC.to_int_ns());
        let next_i = self.rest.partition_point(|&(start_sec, _)| gmt_sec >= start_sec);
        if next_i == 0 {
            let t1 = self.valid_time(gmt_sec, nanosecond, next_i);
            let t2 = self.valid_time(gmt_sec, nanosecond, next_i + 1);
            match (t1, t2) {
                (None, None) => Err(TzError::NoTimeInThisTz),
                (Some(v), None) | (None, Some(v)) => Ok(v),
                (Some(v1), Some(v2)) => Err(TzError::TwoTimesInThisTz(v1, v2)),
            }
        } else {
            let t0 = self.valid_time(gmt_sec, nanosecond, next_i - 1);
            let t1 = self.valid_time(gmt_sec, nanosecond, next_i);
            let t2 = self.valid_time(gmt_sec, nanosecond, next_i + 1);
            match (t0, t1, t2) {
                (None, None, None) => Err(TzError::NoTimeInThisTz),
                (Some(v), None, None) | (None, Some(v), None) | (None, None, Some(v)) => Ok(v),
                (Some(v1), Some(v2), _) | (Some(v1), _, Some(v2)) | (_, Some(v1), Some(v2)) => {
                    Err(TzError::TwoTimesInThisTz(v1, v2))
                }
            }
        }
    }
}
