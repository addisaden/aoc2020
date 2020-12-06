use std::fs;

pub fn aoc01() {
    let filename = "inputs/01/input.txt";
    let content = fs::read_to_string(filename).expect("");
    println!("{}", content);

    let splittet : Vec<&str> = content.split("\n").collect();
    println!("{}", splittet.len());

    let splittet2 : Vec<&str> = content.split("\n").collect();
    println!("{}", splittet2.len());

    let splitter3 : Vec<&str> = content.split("\n").collect();
    println!("{}", splitter3.len());

    for a in splittet {
        if a.len() == 0 {
            continue;
        }
        let inta = a.parse::<i32>().unwrap();

        for b in &splittet2 {
            if b.len() == 0 {
                continue;
            }
            let intb = b.parse::<i32>().unwrap();
            if inta + intb == 2020 {
                println!("{} * {} = {}", inta, intb, inta * intb);
            }

            for c in &splitter3 {
                if c.len() == 0 {
                    continue;
                }
                let intc = c.parse::<i32>().unwrap();
                if inta + intb + intc == 2020 {
                    println!("{} * {} * {} = {}", inta, intb, intc, inta * intb * intc);
                }
            }
        }
    }
}

#[test]
fn test_hello() {
    // aoc01();
    assert_eq!(1, 1);
}
