use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = "inputs/day02-puzzle.txt";
    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok) {
            solution(line);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_repeating_sequence(num: u64) -> bool {
    let s = num.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    let (left, right) = s.split_at(half);

    return left == right
}

fn solution(line: String) {
    let ranges = line.split(",");
    let mut total: u64 = 0;
    
    for range in ranges {
        let vec_ranges: Vec<&str> = range.split('-').collect();
        let start: u64 = vec_ranges[0].parse().unwrap();
        let end: u64 = vec_ranges[1].parse().unwrap();

        for i in start..end+1{            
            if is_repeating_sequence(i){
                total += i;
            }
        }
    }
    println!("Total: {total}");
}
