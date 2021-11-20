use timens::{SpanNs, TimeNs, TzInfo, TzOffset};

const GMT: TzOffset = TzOffset {
    utc_offset: 0,
    dst_offset: 0,
};

const BST: TzOffset = TzOffset {
    utc_offset: 0,
    dst_offset: 3600,
};

const REST: [(i64, TzOffset); 16] = [
    (1509238800, GMT),
    (1521939600, BST),
    (1540688400, GMT),
    (1553994000, BST),
    (1572138000, GMT),
    (1585443600, BST),
    (1603587600, GMT),
    (1616893200, BST),
    (1635642000, GMT),
    (1648342800, BST),
    (1667091600, GMT),
    (1679792400, BST),
    (1698541200, GMT),
    (1711846800, BST),
    (1729990800, GMT),
    (1743296400, BST),
];

const EUROPE_LONDON: TzInfo = TzInfo {
    first: TzOffset {
        utc_offset: 0,
        dst_offset: 3600,
    },
    rest: &REST,
};

#[test]
fn tz() {
    let timens = TimeNs::of_span_since_epoch(SpanNs::of_int_sec(1637437386));
    assert_eq!(EUROPE_LONDON.offset(timens), SpanNs::ZERO);
    let (date, ofday) = timens.to_date_ofday(&EUROPE_LONDON);
    assert_eq!(date.to_string(), "2021-11-20");
    assert_eq!(ofday.to_string(), "19h43m6s");
    let (date, ofday) = timens.to_date_ofday(&TzInfo::GMT);
    assert_eq!(date.to_string(), "2021-11-20");
    assert_eq!(ofday.to_string(), "19h43m6s");
    let timens = TimeNs::of_span_since_epoch(SpanNs::of_int_sec(1630037386));
    assert_eq!(EUROPE_LONDON.offset(timens), SpanNs::of_int_hr(1));
    let (date, ofday) = timens.to_date_ofday(&EUROPE_LONDON);
    assert_eq!(date.to_string(), "2021-08-27");
    assert_eq!(ofday.to_string(), "5h9m46s");
    let (date, ofday) = timens.to_date_ofday(&TzInfo::GMT);
    assert_eq!(date.to_string(), "2021-08-27");
    assert_eq!(ofday.to_string(), "4h9m46s");
    let timens = TimeNs::of_span_since_epoch(SpanNs::of_int_sec(1630037386 - 5 * 3600));
    assert_eq!(EUROPE_LONDON.offset(timens), SpanNs::of_int_hr(1));
    let (date, ofday) = timens.to_date_ofday(&EUROPE_LONDON);
    assert_eq!(date.to_string(), "2021-08-27");
    assert_eq!(ofday.to_string(), "9m46s");
    let (date, ofday) = timens.to_date_ofday(&TzInfo::GMT);
    assert_eq!(date.to_string(), "2021-08-26");
    assert_eq!(ofday.to_string(), "23h9m46s");
}
