use timens::OfDay;

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
