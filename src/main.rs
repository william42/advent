use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    println!("{}", day1b());
}

fn adventlines(loc: &str) -> impl Iterator<Item = String> {
    let file = File::open(loc);
    let reader = BufReader::new(file.unwrap());
    reader.lines().map(|r| r.unwrap())
}

fn process_frequency(line: String) -> i32 {
    let num: i32 = line[1..].parse().unwrap();
    if line.starts_with("-") {
        -num
    } else {
        num
    }
}

fn day1a() -> i32 {
    let lines = adventlines("input1a.txt");
    let frequencies = lines.map(process_frequency);
    frequencies.sum()
}

fn day1b() -> i32 {
    let mut total :i32 = 0;
    let mut seen = HashSet::new();
    loop {
        let lines = adventlines("input1a.txt");
        let frequencies = lines.map(process_frequency);
        
        for freq in frequencies {
            if seen.contains(&total) {
                return total;
            }
            seen.insert(total);
            total = total + freq;
        }
    }
    42 //unreachable
}
