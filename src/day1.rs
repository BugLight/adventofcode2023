use std::{io::BufReader, io::BufRead, fs::File};
use anyhow::Result;
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{iterator, map};

// -- Helpers ------------------------------------------------------------------

fn parse_token(str: &str) -> IResult<&str, char> {
    alt((
        map(tag("one"), |_| '1'),
        map(tag("two"), |_| '2'),
        map(tag("three"), |_| '3'),
        map(tag("four"), |_| '4'),
        map(tag("five"), |_| '5'),
        map(tag("six"), |_| '6'),
        map(tag("seven"), |_| '7'),
        map(tag("eight"), |_| '8'),
        map(tag("nine"), |_| '9'),
        map(tag("zero"), |_| '0'),
        nom::character::complete::anychar
    ))(str)
}

fn parse_callibration_value(str: &str) -> i32 {
    let mut tokens_iterator = iterator(str, parse_token);
    let tokens = tokens_iterator.collect::<Vec<char>>();
    let _ = tokens_iterator.finish();

    let digits = tokens.iter()
        .filter(|t| t.is_ascii_digit())
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

    #[test]
    fn test_parse_callibration_value_with_words() {
        assert_eq!(parse_callibration_value("abcseven1nine5abb"), 75);
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
