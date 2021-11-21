use crate::{SpanNs, TimeNs};

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
    pub const ZERO: TzOffset = TzOffset {
        utc_offset: 0,
        dst_offset: 0,
    };
}

impl TzInfo {
    fn find(&self, timens: TimeNs) -> &TzOffset {
        let sec = timens.0.div_euclid(SpanNs::SEC.to_int_ns());
        let index = self
            .rest
            .partition_point(|&(start_sec, _)| sec >= start_sec);
        if index == 0 {
            &self.first
        } else {
            &self.rest[index - 1].1
        }
    }

    pub fn offset(&self, timens: TimeNs) -> SpanNs {
        let fixed_timespan = self.find(timens);
        SpanNs::of_int_sec((fixed_timespan.utc_offset + fixed_timespan.dst_offset) as i64)
    }

    pub const GMT: TzInfo = TzInfo {
        first: TzOffset::ZERO,
        rest: &[],
    };
}
