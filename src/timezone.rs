use crate::{SpanNs, TimeNs};

#[derive(Copy, Clone)]
pub struct FixedTimespan {
    pub utc_offset: i32,
    pub dst_offset: i32,
    pub name: &'static str,
}

#[derive(Clone)]
pub struct FixedTimespanSet {
    pub first: FixedTimespan,
    pub rest: &'static [(i64, FixedTimespan)],
}

impl FixedTimespanSet {
    fn find(&self, timens: TimeNs) -> &FixedTimespan {
        let sec = timens.0.div_euclid(SpanNs::SEC.0);
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
}
