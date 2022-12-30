use std::str::FromStr;
use timens::{Date, DayOfWeek, Month};

#[test]
fn add_sub() {
    let d = Date::create(2000, Month::Jan, 1).unwrap();
    assert_eq!(d.to_string(), "2000-01-01");
    assert_eq!((d + 1).to_string(), "2000-01-02");
    assert_eq!((d - 1).to_string(), "1999-12-31");
    assert_eq!((d + 365).to_string(), "2000-12-31");
    assert_eq!((d + 366).to_string(), "2001-01-01");
    let d = Date::from_str("2021-11-17").unwrap();
    assert_eq!(d.to_string(), "2021-11-17");
    for i in -10_000..10_000 {
        assert_eq!(d - i + i, d);
        assert_eq!(d + i - i, d);
    }
}

#[test]
fn add_months() {
    let d = Date::from_str("2020-01-16").unwrap();
    assert_eq!(d.to_string(), "2020-01-16");
    assert_eq!(d.day_of_week(), DayOfWeek::Thu);
    assert_eq!(d.add_months(0).to_string(), "2020-01-16");
    assert_eq!(d.add_months(1).to_string(), "2020-02-16");
    assert_eq!(d.add_months(2).to_string(), "2020-03-16");
    assert_eq!(d.add_months(20).to_string(), "2021-09-16");
    assert_eq!(d.add_months(-1).to_string(), "2019-12-16");
    assert_eq!(d.add_months(-2).to_string(), "2019-11-16");
    assert_eq!(d.add_months(-20).to_string(), "2018-05-16");
    let d = Date::from_str("2020-01-31").unwrap();
    assert_eq!(d.to_string(), "2020-01-31");
    assert_eq!(d.day_of_week(), DayOfWeek::Fri);
    assert_eq!(timens::Days::of_date(d).day_of_week(), DayOfWeek::Fri);
    assert_eq!(d.add_months(0).to_string(), "2020-01-31");
    assert_eq!(d.add_months(1).to_string(), "2020-02-29");
    assert_eq!(d.add_months(13).to_string(), "2021-02-28");
    assert_eq!(d.add_months(2).to_string(), "2020-03-31");
    assert_eq!(d.add_months(20).to_string(), "2021-09-30");
    assert_eq!(d.add_months(-1).to_string(), "2019-12-31");
    assert_eq!(d.add_months(-2).to_string(), "2019-11-30");
    assert_eq!(d.add_months(-20).to_string(), "2018-05-31");
}

#[test]
fn create() {
    let mut tested_dates = 0;
    for y in [2014, 9999, 2000, 2020] {
        for m in 1..=12 {
            for d in 1..=31 {
                let month = Month::of_u8(m).unwrap();
                assert_eq!(month.to_u8(), m);
                if d > month.days_in_month(y) {
                    continue;
                }
                let date = Date::create(y, month, d).unwrap();
                assert_eq!(date.year(), y);
                assert_eq!(date.month(), month);
                assert_eq!(date.day(), d);
                let date2 = Date::from_str(&date.to_string()).unwrap();
                assert_eq!(date, date2);
                let date2 = Date::from_str(&date.to_string_iso8601_basic()).unwrap();
                assert_eq!(date, date2);
                let date2 = Date::from_str(&date.to_string().replace('-', "/")).unwrap();
                assert_eq!(date, date2);
                tested_dates += 1;
            }
        }
    }
    assert_eq!(tested_dates, 4 * 365 + 2);
}

#[test]
fn iter() {
    let lo = Date::from_str("2021-01-14").unwrap();
    let up = Date::from_str("2021-01-20").unwrap();
    assert_eq!(lo.day_of_week(), DayOfWeek::Thu);
    assert_eq!(timens::Days::of_date(lo).day_of_week(), DayOfWeek::Thu);
    let dates: Vec<_> = lo.dates_until(up).collect();
    assert_eq!(
        format!("{:?}", dates),
        "[2021-01-14, 2021-01-15, 2021-01-16, 2021-01-17, 2021-01-18, 2021-01-19, 2021-01-20]"
    );
    let dates: Vec<_> = lo.weekdays_until(up).collect();
    assert_eq!(
        format!("{:?}", dates),
        "[2021-01-14, 2021-01-15, 2021-01-18, 2021-01-19, 2021-01-20]"
    );
    for diff in [0, 1, 2, 7, 12, 23, 125, 365] {
        let up = lo + diff;
        assert_eq!(lo.dates_until(up).count(), diff as usize + 1)
    }
}

#[test]
fn business_date() {
    let date = Date::from_str("2021-01-16").unwrap();
    assert_eq!(date.round_forward_to_business_day(|d| d.is_weekday()), date + 2);
    assert_eq!(date.round_backward_to_business_day(|d| d.is_weekday()), date - 1);
    assert_eq!((date + 2).round_forward_to_business_day(|d| d.is_weekday()), date + 2);
    assert_eq!((date + 2).round_backward_to_business_day(|d| d.is_weekday()), date + 2);
    let dates: Vec<_> = date.business_days_until(date + 30, |d| d.is_weekend()).collect();
    assert_eq!(
        format!("{:?}", dates),
        "[2021-01-16, 2021-01-17, 2021-01-23, 2021-01-24, 2021-01-30, 2021-01-31, 2021-02-06, 2021-02-07, 2021-02-13, 2021-02-14]"
    );
    assert_eq!(
        date.add_business_days_rounding_forward(104, |d| d.is_weekend()).to_string(),
        "2022-01-15"
    );
    assert_eq!(
        date.add_business_days_rounding_backward(104, |d| d.is_weekend()).to_string(),
        "2022-01-15"
    );
    assert_eq!(
        (date - 1).add_business_days_rounding_forward(104, |d| d.is_weekend()).to_string(),
        "2022-01-15"
    );
    assert_eq!(
        (date - 1).add_business_days_rounding_backward(104, |d| d.is_weekend()).to_string(),
        "2022-01-09"
    );
    assert_eq!(
        (date - 1).add_business_days_rounding_forward(-104, |d| d.is_weekend()).to_string(),
        "2020-01-18"
    );
    assert_eq!(
        (date - 1).add_business_days_rounding_backward(-104, |d| d.is_weekend()).to_string(),
        "2020-01-12"
    );
    assert_eq!(
        (date - 1).add_business_days_rounding_backward(-1, |d| d.is_weekend()).to_string(),
        "2021-01-09"
    );
    assert_eq!(
        (date - 1).add_business_days_rounding_backward(1, |d| d.is_weekend()).to_string(),
        "2021-01-16"
    );
    assert_eq!(
        (date - 1).add_business_days_rounding_forward(1, |d| d.is_weekend()).to_string(),
        "2021-01-17"
    );
}

#[cfg(feature = "binio")]
#[test]
fn binio_roundtrip() {
    let d = Date::from_str("2021-01-16").unwrap();
    let mut bytes: Vec<u8> = vec![];
    binprot::BinProtWrite::binprot_write(&d, &mut bytes).unwrap();
    let rt: Date = binprot::BinProtRead::binprot_read(&mut bytes.as_slice()).unwrap();
    assert_eq!(d, rt);
    // Serialization test using sample values from:
    // https://github.com/janestreet/core_kernel/blob/b61f11a1c168845ee0b62b57591d6c809c7cdc71/core/test/src/test_date.ml#L28
    let d = Date::create(1066, Month::Oct, 16).unwrap();
    bytes.clear();
    binprot::BinProtWrite::binprot_write(&d, &mut bytes).unwrap();
    assert_eq!(bytes, [254, 42, 4, 9, 16]);
    let rt: Date = binprot::BinProtRead::binprot_read(&mut bytes.as_slice()).unwrap();
    assert_eq!(d, rt);
    let d = Date::create(1955, Month::Nov, 5).unwrap();
    bytes.clear();
    binprot::BinProtWrite::binprot_write(&d, &mut bytes).unwrap();
    assert_eq!(bytes, [254, 163, 7, 10, 5]);
    let rt: Date = binprot::BinProtRead::binprot_read(&mut bytes.as_slice()).unwrap();
    assert_eq!(d, rt);
    let d = Date::create(2012, Month::Apr, 19).unwrap();
    bytes.clear();
    binprot::BinProtWrite::binprot_write(&d, &mut bytes).unwrap();
    assert_eq!(bytes, [254, 220, 7, 3, 19]);
    let rt: Date = binprot::BinProtRead::binprot_read(&mut bytes.as_slice()).unwrap();
    assert_eq!(d, rt);
}

#[cfg(feature = "sexp")]
#[test]
fn sexp_roundtrip() {
    let d = Date::from_str("1991-01-16").unwrap();
    let sexp = rsexp::SexpOf::sexp_of(&d);
    assert_eq!(sexp, rsexp::Sexp::Atom("1991-01-16".as_bytes().to_owned()));
    let rt: Date = rsexp::OfSexp::of_sexp(&sexp).unwrap();
    assert_eq!(d, rt)
}
