use timens::{OfDay, Span};

fn test_str(h: u8, m: u8, s: u8, ns: u32, str: &str) {
    let ofday = OfDay::create(h, m, s, ns).unwrap();
    assert_eq!(ofday.to_string(), str);
    let v: OfDay = str.parse().unwrap();
    assert_eq!(ofday, v);
}

#[test]
fn to_string() {
    test_str(0, 0, 0, 0, "00:00:00");
    test_str(0, 0, 0, 1, "00:00:00.000000001");
    test_str(12, 34, 56, 0, "12:34:56");
    test_str(12, 34, 56, 1, "12:34:56.000000001");
    test_str(12, 34, 56, 10, "12:34:56.00000001");
    test_str(12, 34, 56, 1000, "12:34:56.000001");
    test_str(12, 34, 56, 1000000, "12:34:56.001");
    test_str(12, 34, 56, 789000000, "12:34:56.789");
    test_str(12, 34, 56, 789101100, "12:34:56.7891011");
    test_str(23, 59, 59, 999999999, "23:59:59.999999999");
}

#[test]
fn ops() {
    let ofday = OfDay::create(12, 0, 0, 0).unwrap();
    assert_eq!(ofday.to_string(), "12:00:00");
    assert_eq!((ofday + Span::SEC).to_string(), "12:00:01");
    assert_eq!((ofday - Span::DAY).to_string(), "00:00:00");
    assert_eq!((ofday + Span::DAY).to_string(), "24:00:00");
    assert_eq!((ofday + Span::DAY - Span::NS).to_string(), "23:59:59.999999999");
    assert_eq!(((ofday + Span::MS) % Span::SEC).to_string(), "1ms");
    assert_eq!(((ofday + Span::MS) % Span::HR).to_string(), "1ms");
}

#[cfg(feature = "sexp")]
#[test]
fn sexp_roundtrip() {
    fn test_sexp_rt(input: &str, output: &str) {
        let d: OfDay = input.parse().unwrap();
        let sexp = rsexp::SexpOf::sexp_of(&d);
        assert_eq!(sexp, rsexp::Sexp::Atom(output.as_bytes().to_owned()));
        let rt: OfDay = rsexp::OfSexp::of_sexp(&sexp).unwrap();
        assert_eq!(d, rt)
    }
    test_sexp_rt("13", "13:00:00");
    test_sexp_rt("13:37", "13:37:00");
    test_sexp_rt("13:37:01", "13:37:01");
    test_sexp_rt("13:37:01.0001", "13:37:01.0001");
}
