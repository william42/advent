use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    println!("{}", day3b());
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
    panic!("unreachable code")
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
    panic!("should be unreachable")
}

struct CRect {
    num: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

fn day3line(line: String) -> CRect {
    let line = &line[1..]; //omit initial #
    let mut it = line.splitn(2, " @ ");
    let raw_num = it.next().unwrap();
    let rest = it.next().unwrap();
    let mut it = rest.splitn(2, ",");
    let raw_left = it.next().unwrap();
    let rest = it.next().unwrap();
    let mut it = rest.splitn(2, ": ");
    let raw_top = it.next().unwrap();
    let rest = it.next().unwrap();
    let mut it = rest.splitn(2, "x");
    let raw_width = it.next().unwrap();
    let raw_height = it.next().unwrap();
    CRect {
        num: raw_num.parse().unwrap(),
        left: raw_left.parse().unwrap(),
        top: raw_top.parse().unwrap(),
        width: raw_width.parse().unwrap(),
        height: raw_height.parse().unwrap(),
    }
}

fn day3a() -> i32 {
    let lines = adventlines("input3.txt");
    let lines = lines.map(day3line);
    // let right = lines.iter().map(|x| x.left + x.width).max().unwrap();
    // let bottom = lines.iter().map(|x| x.top + x.height).max().unwrap();
    // println!("{} wide and {} tall", right, bottom);
    let mut fabric: [[i32; 1000]; 1000] = [[0; 1000]; 1000];
    for x in lines {
        for i in x.left..x.left+x.width {
            for j in x.top..x.top+x.height {
                fabric[i as usize][j as usize] += 1;
            }
        }
    }
    // wanted to do this functionally but hit weird type barriers
    let mut res = 0;
    for line in fabric.iter() {
        for x in line.iter() {
            if x >= &2 {
                res += 1;
            }
        }
    }
    res
}

fn day3b() -> u32 {
    let lines = adventlines("input3.txt").map(day3line);
    let mut fabric: [[i32; 1000]; 1000] = [[0; 1000]; 1000];
    for x in lines {
        for i in x.left..x.left+x.width {
            for j in x.top..x.top+x.height {
                fabric[i as usize][j as usize] += 1;
            }
        }
    }

    let lines = adventlines("input3.txt").map(day3line); // i could fix the borrow issue, or i could just read the file twice
    for x in lines {
        let mut good = true;
        for i in x.left..x.left+x.width {
            for j in x.top..x.top+x.height {
                if fabric[i as usize][j as usize] > 1 {
                    good = false;
                }
            }
        }
        if good {
            return x.num;
        }
    }
    panic!("this should never happen")
}