#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "binio",
    derive(binprot_derive::BinProtRead, binprot_derive::BinProtWrite)
)]
pub struct OfDay(i64);

impl OfDay {
    // TODO: validate or clamp?
    pub fn of_ns_since_midnight(i: i64) -> Self {
        Self(i)
    }

    pub fn to_ns_since_midnight(self) -> i64 {
        self.0
    }

    pub fn since_midnight(self) -> crate::SpanNs {
        crate::SpanNs::of_int_ns(self.to_ns_since_midnight())
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
}

impl std::fmt::Display for OfDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let hr = self.hour();
        let min = self.minute();
        let sec = self.second();
        let ns = self.nanosecond();
        if ns == 0 {
            write!(f, "{:02}:{:02}:{02}", hr, min, sec)
        } else {
            let mut ns = ns;
            let mut ns_width = 9;
            while ns % 10 == 0 {
                ns /= 10;
                ns_width -= 1;
            }
            write!(
                f,
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
