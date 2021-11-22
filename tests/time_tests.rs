use std::str::FromStr;
use timens::{Span, TimeNs};

#[test]
fn time() {
    let time = TimeNs::from_str("2020-01-16 11:22:33.456Z").unwrap();
    assert_eq!(time.to_string_gmt(), "2020-01-16 11:22:33.456Z");
    let time = time + Span::SEC * 5;
    assert_eq!(time.to_string_gmt(), "2020-01-16 11:22:38.456Z");
    let time = time + Span::MS * 544;
    assert_eq!(time.to_string_gmt(), "2020-01-16 11:22:39Z");
    let time = time - Span::NS;
    assert_eq!(time.to_string_gmt(), "2020-01-16 11:22:38.999999999Z");
}
