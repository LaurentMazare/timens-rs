use timens::SpanNs;

fn test_str(s: SpanNs, str: &str) {
    assert_eq!(s.to_string(), str);
    let v: SpanNs = str.parse().unwrap();
    assert_eq!(s, v);
}

#[test]
fn to_string() {
    test_str(SpanNs::ZERO, "0s");
    test_str(SpanNs::SEC, "1s");
    test_str(SpanNs::MS * 1., "1ms");
    test_str(SpanNs::MS * 2., "2ms");
    test_str(SpanNs::MS * 222., "222ms");
    test_str(SpanNs::MS * 222.002, "222.002ms");
    test_str(SpanNs::MS * 222.022, "222.022ms");
    test_str(SpanNs::MS * 222.222, "222.222ms");
    test_str(SpanNs::MS * 1.5, "1.5ms");
    test_str(SpanNs::MS * 1.2, "1.2ms");
    test_str(SpanNs::MS * 1.23, "1.23ms");
    test_str(SpanNs::MS * 1.234, "1.234ms");
    test_str(SpanNs::MS * 1.2345, "1.2345ms");
    test_str(SpanNs::MS * 1.23456, "1.23456ms");
    test_str(SpanNs::MS * 1.234567, "1.234567ms");
    test_str(SpanNs::MS * 1.034567, "1.034567ms");
    test_str(SpanNs::MS * 0.5, "500us");
    test_str(SpanNs::US, "1us");
    test_str(SpanNs::NS, "1ns");
    test_str(SpanNs::of_day(12.), "12d");
    test_str(SpanNs::DAY * 12 + SpanNs::NS, "12d1ns");
    test_str((SpanNs::DAY * 12 + SpanNs::NS).neg(), "-12d1ns");
    test_str(SpanNs::DAY * 12 + SpanNs::MS + SpanNs::NS, "12d1.000001ms");
    test_str(SpanNs::DAY + SpanNs::MS + SpanNs::US, "1d1.001ms");
    test_str(
        SpanNs::DAY + SpanNs::MS * 123456 + SpanNs::US,
        "1d2m3.456001s",
    );
    test_str(SpanNs::DAY + SpanNs::MS * 123456, "1d2m3.456s");
    test_str(SpanNs::HR + SpanNs::US * 123456, "1h123.456ms");
    test_str(SpanNs::MIN * 12 + SpanNs::US * 123456, "12m123.456ms");
    test_str(SpanNs::MIN * 12 + SpanNs::NS * 123000, "12m123us");
    test_str(SpanNs::MIN * 12 + SpanNs::NS * 123001, "12m123.001us");
}
