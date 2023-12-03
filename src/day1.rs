use std::{io::BufReader, io::BufRead, fs::File};
use anyhow::Result;

// -- Helpers ------------------------------------------------------------------

fn parse_callibration_value(str: &str) -> i32 {
    let digits = str.chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();
    10 * digits.first().unwrap() + digits.last().unwrap()
}

// -- Tests --------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_callibration_value() {
        assert_eq!(parse_callibration_value("aa1a2b"), 12);
        assert_eq!(parse_callibration_value("1a2b1"), 11);
        assert_eq!(parse_callibration_value("aaaa1aaaa"), 11);
    }
}

// -- Main ---------------------------------------------------------------------

pub fn main() -> Result<()> {
    let reader = BufReader::new(File::open("data/input/1.txt")?);
    let answer: i32 = reader.lines()
        .map(|line| parse_callibration_value(&line.unwrap()))
        .sum();
    println!("{}", answer);
    Ok(())
}
