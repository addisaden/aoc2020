use std::fs;

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

fn is_valid_2 (lower: usize, upper: usize, single_char: String, validation_string: String) -> bool {
    let mut lower_match : bool = false;
    let mut upper_match : bool = false;

    match validation_string[(lower-1)..].find(single_char.as_str()) {
        None => lower_match = false,
        Some(y) => {
            if y == 0 {
                lower_match = true;
            }
        }
    }
    match validation_string[(upper-1)..].find(single_char.as_str()) {
        None => upper_match = false,
        Some(y) => {
            if y == 0 {
                upper_match = true;
            }
        }
    }

    lower_match != upper_match
}

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

pub fn day02a() -> usize {
    let filename = "inputs/02/input.txt";

    let content = fs::read_to_string(filename).expect("");

    let splittet : Vec<&str> = content.split("\n").collect();

    let mut valid_passwords : usize = 0;

    for a in splittet {
        match parse_line(String::from(a)) {
            None => print!(""),
            Some((lower, upper, chr, stri)) => {
                if is_valid(lower, upper, String::from(chr), String::from(stri)) {
                    valid_passwords += 1;
                }
            }
        }
    }

    valid_passwords
}

pub fn day02b() -> usize {
    let filename = "inputs/02/input.txt";

    let content = fs::read_to_string(filename).expect("");

    let splittet : Vec<&str> = content.split("\n").collect();

    let mut valid_passwords : usize = 0;

    for a in splittet {
        match parse_line(String::from(a)) {
            None => print!(""),
            Some((lower, upper, chr, stri)) => {
                if is_valid_2(lower, upper, String::from(chr), String::from(stri)) {
                    valid_passwords += 1;
                }
            }
        }
    }

    valid_passwords
}


#[test]
fn test_validation() {
    assert_eq!(is_valid(1, 3, String::from("a"), String::from("abcde")), true, "abcde valid");
    assert_eq!(is_valid(1, 3, String::from("b"), String::from("cdefg")), false, "cdefg invalid");
    assert_eq!(is_valid(2, 9, String::from("c"), String::from("ccccccccc")), true, "ccccccccc valid");
    assert_eq!(is_valid(2, 9, String::from("c"), String::from("cccccccccc")), false, "cccccccccc valid");
}

#[test]
fn test_validation_part_2() {
    assert_eq!(is_valid_2(1, 3, String::from("a"), String::from("abcde")), true, "abcde valid");
    assert_eq!(is_valid_2(1, 3, String::from("b"), String::from("cdefg")), false, "cdefg invalid");
    assert_eq!(is_valid_2(2, 9, String::from("c"), String::from("ccccccccc")), false, "ccccccccc invalid");
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

#[test]
fn test_valid_passwords() {
    assert_eq!(445, day02a());
    assert!(250 != day02b());
    assert!(255 != day02b());
    assert_eq!(491, day02b());
}
