mod day01;
mod day01t2;

use std::collections::HashMap;
use std::io::Result;

fn main() -> Result<()> {
    let day = std::env::args().nth(1).unwrap_or("1".into());

    let mut runners: HashMap<&str, fn() -> Result<()>> = HashMap::new();
    runners.insert("day1-1", day01::solution);
    runners.insert("day1-2", day01t2::solution);

    match runners.get(day.as_str()) {
        Some(run_fn) => run_fn(),
        None => {
            println!("Unknown day {}", day);
            Ok(())
        }
    }
}
