use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    println!("{}", day2b());
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
    42 //shouldn't happen
}


fn day2a() -> i32 {
    let lines= adventlines("input2.txt");
    let mut twos = 0;
    let mut threes = 0;
    for line in lines {
        let mut counts : HashMap<char, u32> = HashMap::new();
        for l in line.chars() {
            let counter = counts.entry(l).or_insert(0);
            *counter += 1;
        }
        if counts.values().any(|&x| x==2) {
            twos += 1;
        }
        if counts.values().any(|&x| x==3) {
            threes += 1;
        }
    }
    twos * threes
}

fn day2b() -> String {
    let lines : Vec<String> = adventlines("input2.txt").collect();
    let len = lines[0].len();
    for i in 0..len {
        let mut seen = HashSet::new();
        for line in &lines {
            let mut common = String::with_capacity(len);
            common.push_str(&line[..i]);
            common.push_str(&line[i+1..]);
            if seen.contains(&common) {
                return common;
            }
            seen.insert(common);
        }
    }
    String::new() //shouldn't happen
}
