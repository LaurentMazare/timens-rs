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
    assert_eq!(d.add_months(0).to_string(), "2020-01-31");
    assert_eq!(d.add_months(1).to_string(), "2020-02-29");
    assert_eq!(d.add_months(13).to_string(), "2021-02-28");
    assert_eq!(d.add_months(2).to_string(), "2020-03-31");
    assert_eq!(d.add_months(20).to_string(), "2021-09-30");
    assert_eq!(d.add_months(-1).to_string(), "2019-12-31");
    assert_eq!(d.add_months(-2).to_string(), "2019-11-30");
    assert_eq!(d.add_months(-20).to_string(), "2018-05-31");
}

#[cfg(feature = "binio")]
#[test]
fn binio_roundtrip() {
    let d = Date::from_str("2021-01-16").unwrap();
    let mut bytes: Vec<u8> = vec![];
    binprot::BinProtWrite::binprot_write(&d, &mut bytes).unwrap();
    let rt: Date = binprot::BinProtRead::binprot_read(&mut bytes.as_slice()).unwrap();
    assert_eq!(d, rt)
}

#[cfg(feature = "sexp")]
#[test]
fn sexp_roundtrip() {
    let d = Date::from_str("1991-01-16").unwrap();
    let sexp = rsexp::SexpOf::sexp_of(&d);
    let rt: Date = rsexp::OfSexp::of_sexp(&sexp).unwrap();
    assert_eq!(d, rt)
}
