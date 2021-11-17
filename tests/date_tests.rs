use std::str::FromStr;
use timens::{Date, Month};

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
