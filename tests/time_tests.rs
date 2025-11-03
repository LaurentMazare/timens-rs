use std::str::FromStr;
use timens::{Date, OfDay, Span, Time};

#[test]
fn time() {
    let time = Time::from_str("2020-01-16T11:22:33.456Z").unwrap();
    assert_eq!(time.to_string_gmt(), "2020-01-16 11:22:33.456Z");
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

#[test]
fn of_date_ofday_gmt() {
    let time: Time = "2020-01-16 11:22:33.456Z".parse().unwrap();
    let date = Date::create(2020, timens::Month::Jan, 16).unwrap();
    let ofday = OfDay::create(11, 22, 33, 456_000_000).unwrap();
    let (date2, ofday2) = time.to_date_ofday_gmt();
    let time2 = Time::of_date_ofday_gmt(date, ofday);
    assert_eq!(date, date2);
    assert_eq!(ofday, ofday2);
    assert_eq!(time, time2);
    assert_eq!(
        Time::of_date_ofday_gmt(date, OfDay::START_OF_DAY).to_string_gmt(),
        "2020-01-16 00:00:00Z"
    );
    assert_eq!(
        Time::of_date_ofday_gmt(date, OfDay::START_OF_NEXT_DAY).to_string_gmt(),
        "2020-01-17 00:00:00Z"
    );
}

#[cfg(feature = "sexp")]
#[test]
fn sexp_roundtrip() {
    let d: Time = "2020-07-16 15:15:45 Europe/London".parse().unwrap();
    let sexp = rsexp::SexpOf::sexp_of(&d);
    assert_eq!(sexp.to_string(), "(2020-07-16 14:15:45Z)");
    let rt: Time = rsexp::OfSexp::of_sexp(&sexp).unwrap();
    assert_eq!(d, rt)
}
