use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map,
    multi::many1,
    sequence::{delimited, preceded, terminated},
    IResult,
};

// -- Types -------------------------------------------------------------------

#[derive(Debug, PartialEq)]
struct Card {
    id: i32,
    winning: HashSet<i32>,
    numbers: Vec<i32>,
}

// -- Implementation ----------------------------------------------------------

impl Card {
    fn parse(input: &str) -> IResult<&str, Card> {
        let (rest, id) = delimited(
            terminated(tag("Card"), many1(tag(" "))),
            map(digit1, |s: &str| s.parse().unwrap()),
            terminated(tag(":"), many1(tag(" "))),
        )(input)?;

        let (rest, winning) = terminated(
            many1(terminated(
                map(digit1, |s: &str| s.parse::<i32>().unwrap()),
                many1(tag(" ")),
            )),
            tag("|"),
        )(rest)?;
        let winning = HashSet::from_iter(winning.iter().cloned());

        let (rest, numbers) = many1(preceded(
            many1(tag(" ")),
            map(digit1, |s: &str| s.parse::<i32>().unwrap()),
        ))(rest)?;

        Ok((
            rest,
            Card {
                id,
                winning,
                numbers,
            },
        ))
    }

    fn winning_numbers_count(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }
}

// -- Tests -------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_parse() -> Result<()> {
        let (rest, card) =
            Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")?;

        assert_eq!(rest, "");
        assert_eq!(
            card,
            Card {
                id: 1,
                winning: HashSet::from([41, 48, 83, 86, 17]),
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
            }
        );

        let (rest, card) = Card::parse("Card   2:   11 | 33  22")?;

        assert_eq!(rest, "");
        assert_eq!(
            card,
            Card {
                id: 2,
                winning: HashSet::from([11]),
                numbers: vec![33, 22]
            }
        );
        Ok(())
    }
}

// -- Main --------------------------------------------------------------------

fn part1(cards: &Vec<Card>) {
    println!(
        "Part 1: {}",
        cards
            .iter()
            .map(|c| c.winning_numbers_count())
            .filter(|&x| x > 0)
            .map(|x| (2u32).pow((x - 1).try_into().unwrap()))
            .sum::<u32>()
    );
}

pub fn main() -> Result<()> {
    let reader = BufReader::new(File::open("data/input/4.txt")?);
    let cards = reader
        .lines()
        .map(|line| Card::parse(&line.unwrap()).unwrap().1)
        .collect::<Vec<_>>();

    part1(&cards);
    Ok(())
}
