use std::str::FromStr;
use timens::{Date, OfDay, Span, Time, Tz, TzError};

fn to_date_ofday_rt(time: Time, tz: Tz) -> (Date, OfDay) {
    let (date, ofday) = time.to_date_ofday(tz);
    let rt_time = Time::of_date_ofday(date, ofday, tz).unwrap();
    assert_eq!(time, rt_time);
    (date, ofday)
}

#[test]
fn tz() {
    let europe_london = Tz::Europe__London.tz_info();
    let time = Time::of_span_since_epoch(Span::of_int_sec(1637437386));
    assert_eq!(europe_london.offset(time), Span::ZERO);
    let (date, ofday) = to_date_ofday_rt(time, Tz::Europe__London);
    assert_eq!(date.to_string(), "2021-11-20");
    assert_eq!(ofday.to_string(), "19:43:06");
    let (date, ofday) = to_date_ofday_rt(time, Tz::GMT);
    assert_eq!(date.to_string(), "2021-11-20");
    assert_eq!(ofday.to_string(), "19:43:06");
    let time = Time::of_span_since_epoch(Span::of_int_sec(1630037386));
    assert_eq!(europe_london.offset(time), Span::of_int_hr(1));
    let (date, ofday) = to_date_ofday_rt(time, Tz::Europe__London);
    assert_eq!(date.to_string(), "2021-08-27");
    assert_eq!(ofday.to_string(), "05:09:46");
    let (date, ofday) = to_date_ofday_rt(time, Tz::GMT);
    assert_eq!(date.to_string(), "2021-08-27");
    assert_eq!(ofday.to_string(), "04:09:46");
    let time = Time::of_span_since_epoch(Span::of_int_sec(1630037386 - 5 * 3600));
    assert_eq!(europe_london.offset(time), Span::of_int_hr(1));
    let (date, ofday) = to_date_ofday_rt(time, Tz::Europe__London);
    assert_eq!(date.to_string(), "2021-08-27");
    assert_eq!(ofday.to_string(), "00:09:46");
    let (date, ofday) = to_date_ofday_rt(time, Tz::GMT);
    assert_eq!(date.to_string(), "2021-08-26");
    assert_eq!(ofday.to_string(), "23:09:46");
    assert_eq!(time.to_string_tz(Tz::GMT), "2021-08-26 23:09:46Z");
    assert_eq!(time.to_string_tz(Tz::Europe__London), "2021-08-27 00:09:46+01:00");
}

fn of_date_ofday_ldn(date: &str, ofday: &str) -> Result<Time, TzError> {
    let date = Date::from_str(date).unwrap();
    let ofday = OfDay::from_str(ofday).unwrap();
    Time::of_date_ofday(date, ofday, Tz::Europe__London)
}

#[test]
fn tz_daylightsaving() {
    let time = of_date_ofday_ldn("2021-11-21", "20:51:45").unwrap();
    assert_eq!(time.to_string_gmt(), "2021-11-21 20:51:45Z");
    // Day-light saving boundary, the same hour happened twice on 2021-10-31.
    let time = of_date_ofday_ldn("2021-10-31", "00:59:59").unwrap();
    assert_eq!(time.to_string_gmt(), "2021-10-30 23:59:59Z");
    match of_date_ofday_ldn("2021-10-31", "01:00:00") {
        Err(TzError::TwoTimesInThisTz(t1, t2)) => {
            assert_eq!(t1.to_string_gmt(), "2021-10-31 00:00:00Z");
            assert_eq!(t2.to_string_gmt(), "2021-10-31 01:00:00Z");
        }
        otherwise => panic!("unexpected {:?}", otherwise),
    }
    match of_date_ofday_ldn("2021-10-31", "01:30:00") {
        Err(TzError::TwoTimesInThisTz(t1, t2)) => {
            assert_eq!(t1.to_string_gmt(), "2021-10-31 00:30:00Z");
            assert_eq!(t2.to_string_gmt(), "2021-10-31 01:30:00Z");
        }
        otherwise => panic!("unexpected {:?}", otherwise),
    }
    match of_date_ofday_ldn("2021-10-31", "01:59:59.999999999") {
        Err(TzError::TwoTimesInThisTz(t1, t2)) => {
            assert_eq!(t1.to_string_gmt(), "2021-10-31 00:59:59.999999999Z");
            assert_eq!(t2.to_string_gmt(), "2021-10-31 01:59:59.999999999Z");
        }
        otherwise => panic!("unexpected {:?}", otherwise),
    }
    let time = of_date_ofday_ldn("2021-10-31", "02:00:00").unwrap();
    assert_eq!(time.to_string_gmt(), "2021-10-31 02:00:00Z");
    // Day-light saving boundary, an hour disappeared on 2021-03-28.
    let time = of_date_ofday_ldn("2021-03-28", "00:59:59.9").unwrap();
    assert_eq!(time.to_string_gmt(), "2021-03-28 00:59:59.9Z");
    let time = of_date_ofday_ldn("2021-03-28", "02:00:00").unwrap();
    assert_eq!(time.to_string_gmt(), "2021-03-28 01:00:00Z");
    match of_date_ofday_ldn("2021-03-28", "01:59:59.999999999") {
        Err(TzError::NoTimeInThisTz) => (),
        otherwise => panic!("unexpected {:?}", otherwise),
    }
    match of_date_ofday_ldn("2021-03-28", "01:00:00") {
        Err(TzError::NoTimeInThisTz) => (),
        otherwise => panic!("unexpected {:?}", otherwise),
    }
    match of_date_ofday_ldn("2021-03-28", "01:27:00.123456789") {
        Err(TzError::NoTimeInThisTz) => (),
        otherwise => panic!("unexpected {:?}", otherwise),
    }
}

#[test]
fn tz_daylightsaving_nyc() {
    let tz = Tz::America__New_York;
    let time = Time::from_str(&"2021-03-14 06:00Z").unwrap();
    let (date, ofday) = time.to_date_ofday(tz);
    let time2 = Time::of_date_ofday(date, ofday, tz).unwrap();
    assert_eq!(time, time2);
    assert_eq!(time.to_string_tz(tz), "2021-03-14 01:00:00-05:00");
    let time = Time::from_str(&"2021-03-14 06:59:59.9Z").unwrap();
    let (date, ofday) = time.to_date_ofday(tz);
    let time2 = Time::of_date_ofday(date, ofday, tz).unwrap();
    assert_eq!(time, time2);
    assert_eq!(time.to_string_tz(tz), "2021-03-14 01:59:59.9-05:00");
    let time = Time::from_str(&"2021-03-14 07:00Z").unwrap();
    let (date, ofday) = time.to_date_ofday(tz);
    let time2 = Time::of_date_ofday(date, ofday, tz).unwrap();
    assert_eq!(time, time2);
    assert_eq!(time.to_string_tz(tz), "2021-03-14 03:00:00-04:00");
}
