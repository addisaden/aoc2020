#[allow(dead_code)]
fn is_valid (lower: usize, upper: usize, single_char: String, validation_string: String) -> bool {
    let mut found = 0;
    for x in 0..validation_string.len() {
        match validation_string[x..].find(single_char.as_str()) {
            None => break,
            Some(0) => found += 1,
            Some(x) => {
                if x > 0 {
                    continue
                }
            }
        }
    }
    found >= lower && found <= upper
}

#[allow(dead_code)]
fn parse_line (line: String) -> Option<(usize, usize, String, String)> {
    let result: Vec<&str> = line.rsplitn(2, ":").collect();
    if result.len() != 2 {
        return None
    }

    let validation: String = String::from(result[1].trim());
    let result: String = String::from(result[0].trim());

    let validsplit: Vec<&str> = validation.rsplitn(2, " ").collect();
    if validsplit.len() != 2 {
        return None
    }
    let single_char = String::from(validsplit[0].trim());
    let numbers = String::from(validsplit[1].trim());

    let numbersplit: Vec<&str> = numbers.rsplitn(2, "-").collect();
    if numbersplit.len() != 2 {
        return None
    }
    
    let a: usize = numbersplit[1].parse::<usize>().unwrap();
    let b: usize = numbersplit[0].parse::<usize>().unwrap();

    Some((a, b, single_char, result))
}

#[test]
fn test_validation() {
    assert_eq!(is_valid(1, 3, String::from("a"), String::from("abcde")), true, "abcde valid");
    assert_eq!(is_valid(1, 3, String::from("b"), String::from("cdefg")), false, "cdefg invalid");
    assert_eq!(is_valid(2, 9, String::from("c"), String::from("ccccccccc")), true, "ccccccccc valid");
    assert_eq!(is_valid(2, 9, String::from("c"), String::from("cccccccccc")), false, "cccccccccc valid");
}

#[test]
fn test_parser() {
    match parse_line(String::from("1-3 a: abcde")) {
        None => assert_eq!(None, Some((1, 3, "a", "abcde")), "Result should not be None"),
        Some((lower, upper, chr, stri)) => {
            assert_eq!(stri, String::from("abcde"), "test string should be abcde");
            assert_eq!(chr, String::from("a"), "Validate Character should be a");
            assert_eq!(lower, 1, "First parsed Value should be 1");
            assert_eq!(upper, 3, "Second parsed Value should be 3");
        },
    }
}
