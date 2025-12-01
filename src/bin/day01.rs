use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::Result;

fn main() -> Result<()> {
    solution()
}

pub fn solution() -> io::Result<()> {
    let mut dial: i32 = 50;
    let max: i32 = 99;
    let mut zeroes: i32 = 0;

    let path = "inputs/day01-puzzle.txt";

    let file = File::open(Path::new(path))?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?.trim().to_string();
        let direction = &line[0..1];
        let number: i32 = line[1..].parse().unwrap();

        if direction == "R" {
            dial = (dial + number) % (max + 1);
        } else {
            dial = (dial - number) % (max + 1);
        }

        if dial == 0 {
            zeroes += 1;
        }

    }

    println!("\n Password: {zeroes}");

    Ok(())
}
