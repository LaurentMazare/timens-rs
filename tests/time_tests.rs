use std::str::FromStr;
use timens::{Span, Time};

#[test]
fn time() {
    let time = Time::from_str("2020-01-16 11:22:33.456Z").unwrap();
    assert_eq!(time.to_string_gmt(), "2020-01-16 11:22:33.456Z");
    let time = time + Span::SEC * 5;
    assert_eq!(time.to_string_gmt(), "2020-01-16 11:22:38.456Z");
    let time = time + Span::MS * 544;
    assert_eq!(time.to_string_gmt(), "2020-01-16 11:22:39Z");
    let time = time - Span::NS;
    assert_eq!(time.to_string_gmt(), "2020-01-16 11:22:38.999999999Z");
}

#[test]
fn from_str() {
    let time = Time::from_str("2020-01-16 11:22:33.456Z").unwrap();
    assert_eq!(time.to_string_gmt(), "2020-01-16 11:22:33.456Z");
    let time2 = Time::from_str("2020-01-16 11:22:33.456+01").unwrap();
    assert_eq!(time2.to_string_gmt(), "2020-01-16 12:22:33.456Z");
    assert_eq!(time2, time + Span::HR);
    let time2 = Time::from_str("2020-01-16 11:22:33.456+01:00").unwrap();
    assert_eq!(time2, time + Span::HR);
    let time2 = Time::from_str("2020-01-16 11:22:33.456+00:00").unwrap();
    assert_eq!(time2, time);
    let time2 = Time::from_str("2020-01-16 11:22:33.456-00:00").unwrap();
    assert_eq!(time2, time);
    let time2 = Time::from_str("2020-01-16 11:22:33.456-07:00").unwrap();
    assert_eq!(time2, time - Span::HR * 7);
    let time2 = Time::from_str("2020-01-16 11:22:33.456-07:30").unwrap();
    assert_eq!(time2, time - Span::HR * 7 - Span::MIN * 30);
    let time2 = Time::from_str("2020-01-16 11:22:33.456-07:30:00").unwrap();
    assert_eq!(time2, time - Span::HR * 7 - Span::MIN * 30);
    let time2 = Time::from_str("2020-01-16 11:22:33.456-07:30:59").unwrap();
    assert_eq!(time2, time - Span::HR * 7 - Span::MIN * 30 - Span::SEC * 59);
    let time2 = Time::from_str("2020-01-16 11:22:33.456 America/New_York").unwrap();
    assert_eq!(time2, time + Span::HR * 5);
    assert_eq!(
        Time::from_str("2020-01-16 23:22:33 America/New_York").unwrap(),
        Time::from_str("2020-01-17 04:22:33 GMT").unwrap(),
    );
}
