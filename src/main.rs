use std::error::Error;
use std::io::{self, Read, Write};
use structopt::StructOpt;

mod day1;
mod day2;
#[cfg(test)]
mod testmacros;

struct Solution {
    part1: fn(&str) -> Result<String, Box<dyn Error + '_>>,
    part2: fn(&str) -> Result<String, Box<dyn Error + '_>>,
}

const SOLUTIONS: &[Solution] = &[day1::DAY1, day2::DAY2];

#[derive(StructOpt)]
struct Options {
    /// Day for which a solution should be ran
    day: u8,
    /// Input, if not provided taken from stdin
    input: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Options::from_args();
    let solution = SOLUTIONS
        .get(usize::from(opt.day) - 1)
        .ok_or("Day number out of range")?;
    let input = match opt.input {
        Some(input) => input,
        None => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            input
        }
    };
    writeln!(
        io::stdout(),
        "Part 1: {}",
        (solution.part1)(&input).map_err(|e| e.to_string())?
    )?;
    writeln!(
        io::stdout(),
        "Part 2: {}",
        (solution.part2)(&input).map_err(|e| e.to_string())?
    )?;
    Ok(())
}
