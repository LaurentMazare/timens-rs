use timens::Span;

fn test_str(s: Span, str: &str) {
    assert_eq!(s.to_string(), str);
    let v: Span = str.parse().unwrap();
    assert_eq!(s, v);
}

#[test]
fn to_string() {
    test_str(Span::ZERO, "0s");
    test_str(Span::SEC, "1s");
    test_str(Span::MS * 1., "1ms");
    test_str(Span::MS * 2., "2ms");
    test_str(Span::MS * 222., "222ms");
    test_str(Span::MS * 222.002, "222.002ms");
    test_str(Span::MS * 222.022, "222.022ms");
    test_str(Span::MS * 222.222, "222.222ms");
    test_str(Span::MS * 1.5, "1.5ms");
    test_str(Span::MS * 1.2, "1.2ms");
    test_str(Span::MS * 1.23, "1.23ms");
    test_str(Span::MS * 1.234, "1.234ms");
    test_str(Span::MS * 1.2345, "1.2345ms");
    test_str(Span::MS * 1.23456, "1.23456ms");
    test_str(Span::MS * 1.234567, "1.234567ms");
    test_str(Span::MS * 1.034567, "1.034567ms");
    test_str(Span::MS * 0.5, "500us");
    test_str(Span::US, "1us");
    test_str(Span::NS, "1ns");
    test_str(Span::of_day(12.), "12d");
    test_str(Span::DAY * 12 + Span::NS, "12d1ns");
    test_str(-Span::DAY * 12 - Span::NS, "-12d1ns");
    test_str(-Span::DAY * 12 + Span::NS, "-11d23h59m59.999999999s");
    test_str(Span::DAY * 12 + Span::MS + Span::NS, "12d1.000001ms");
    test_str(Span::DAY + Span::MS + Span::US, "1d1.001ms");
    test_str(Span::DAY + Span::MS * 123456 + Span::US, "1d2m3.456001s");
    test_str(Span::DAY + Span::MS * 123456, "1d2m3.456s");
    test_str(Span::HR + Span::US * 123456, "1h123.456ms");
    test_str(Span::MIN * 12 + Span::US * 123456, "12m123.456ms");
    test_str(Span::MIN * 12 + Span::NS * 123000, "12m123us");
    test_str(Span::MIN * 12 + Span::NS * 123001, "12m123.001us");

    assert_eq!("1.1d".parse::<Span>().unwrap(), Span::DAY + Span::MIN * 144);

    // Some weird formatting
    assert_eq!("1.d0s.0s.1d".parse::<Span>().unwrap(), Span::DAY + Span::MIN * 144);
}

#[cfg(feature = "sexp")]
#[test]
fn sexp_roundtrip() {
    for span in ["0s", "1us", "10m", "12m123.001us", "-12d1ns"] {
        let d: Span = span.parse().unwrap();
        let sexp = rsexp::SexpOf::sexp_of(&d);
        assert_eq!(sexp, rsexp::Sexp::Atom(span.as_bytes().to_owned()));
        let rt: Span = rsexp::OfSexp::of_sexp(&sexp).unwrap();
        assert_eq!(d, rt)
    }
}
