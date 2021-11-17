use timens::{Date, Month};

#[test]
fn add_sub() {
    let d = Date::create(2000, Month::Jan, 1).unwrap();
    assert_eq!(d.to_string(), "2000-01-01");
    assert_eq!((d + 1).to_string(), "2000-01-02");
    assert_eq!((d - 1).to_string(), "1999-12-31");
    assert_eq!((d + 365).to_string(), "2000-12-31");
    assert_eq!((d + 366).to_string(), "2001-01-01")
}
