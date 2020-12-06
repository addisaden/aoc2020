fn is_valid (lower: i32, upper: i32, single_char: String, validation_string: String) -> bool {
    false
}

fn parse_line (line: String) -> (i32, i32, String, String) {
    (0, 0, String::from(""), String::from(""))
}

#[test]
fn test_validation() {
    assert_eq!(is_valid(1, 3, String::from("a"), String::from("abcde")), true);
    assert_eq!(is_valid(1, 3, String::from("b"), String::from("cdefg")), false);
    assert_eq!(is_valid(2, 9, String::from("c"), String::from("ccccccccc")), true);
}

#[test]
fn test_parser() {
    let (lower, upper, chr, stri) = parse_line(String::from("1-3 a: abcde"));
    assert_eq!(lower, 1);
    assert_eq!(upper, 3);
    assert_eq!(chr, String::from("a"));
    assert_eq!(stri, String::from("abcde"));
}
