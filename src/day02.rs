#[test]
fn test_validation() {
    assert_eq!(is_valid(1, 3, "a", "abcde"), true);
    assert_eq!(is_valid(1, 3, "b", "cdefg"), false);
    assert_eq!(is_valid(2, 9, "c", "ccccccccc"), true);
}

#[test]
fn test_parser() {
    let (lower, upper, chr, stri) = parse_line("1-3 a: abcde");
    assert_eq!(lower, 1);
    assert_eq!(upper, 3);
    assert_eq!(chr, "a");
    assert_eq!(stri, "abcde");
}
