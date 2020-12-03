use std::env;
use std::fs;

fn aoc01(filename : &str) {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let filename = &args[1];
        println!("Filename: {}", filename);
        aoc01(filename);
    }
}
