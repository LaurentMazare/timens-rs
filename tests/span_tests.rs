use timens::SpanNs;

#[test]
fn roundtrip_sexp() {
    assert_eq!(SpanNs::ZERO.to_string(), "0s");
    assert_eq!(SpanNs::SEC.to_string(), "1s");
    assert_eq!((SpanNs::MS * 1.).to_string(), "1ms");
    assert_eq!((SpanNs::MS * 2.).to_string(), "2ms");
    assert_eq!((SpanNs::MS * 222.).to_string(), "222ms");
    assert_eq!((SpanNs::MS * 222.222).to_string(), "222.222ms");
    assert_eq!((SpanNs::MS * 1.5).to_string(), "1.5ms");
    assert_eq!((SpanNs::MS * 1.2).to_string(), "1.2ms");
    assert_eq!((SpanNs::MS * 1.23).to_string(), "1.23ms");
    assert_eq!((SpanNs::MS * 1.234).to_string(), "1.234ms");
    assert_eq!((SpanNs::MS * 1.2345).to_string(), "1.2345ms");
    assert_eq!((SpanNs::MS * 1.23456).to_string(), "1.23456ms");
    assert_eq!((SpanNs::MS * 1.234567).to_string(), "1.234567ms");
    assert_eq!((SpanNs::MS * 0.5).to_string(), "500us");
    assert_eq!(SpanNs::US.to_string(), "1us");
    assert_eq!(SpanNs::NS.to_string(), "1ns");
    assert_eq!(SpanNs::of_day(12.).to_string(), "12d");
    assert_eq!((SpanNs::DAY * 12 + SpanNs::NS).to_string(), "12d1ns");
    assert_eq!((SpanNs::DAY * 12 + SpanNs::NS).neg().to_string(), "-12d1ns");
    assert_eq!(
        (SpanNs::DAY * 12 + SpanNs::MS + SpanNs::NS).to_string(),
        "12d1.000001ms"
    );
    assert_eq!(
        (SpanNs::DAY + SpanNs::MS + SpanNs::US).to_string(),
        "1d1.001ms"
    );
    assert_eq!(
        (SpanNs::DAY + SpanNs::MS * 123456 + SpanNs::US).to_string(),
        "1d2m3.456001s"
    );
    assert_eq!(
        (SpanNs::DAY + SpanNs::MS * 123456).to_string(),
        "1d2m3.456s"
    );
    assert_eq!(
        (SpanNs::HR + SpanNs::US * 123456).to_string(),
        "1h123.456ms"
    );
    assert_eq!(
        (SpanNs::MIN * 12 + SpanNs::US * 123456).to_string(),
        "12m123.456ms"
    );
    assert_eq!(
        (SpanNs::MIN * 12 + SpanNs::NS * 123000).to_string(),
        "12m123us"
    );
    assert_eq!(
        (SpanNs::MIN * 12 + SpanNs::NS * 123001).to_string(),
        "12m123.001us"
    );
}
