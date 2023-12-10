use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt},
    multi::fold_many1,
    sequence::{delimited, pair, terminated},
    IResult,
};

// -- Types --------------------------------------------------------------------

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u32,
    g: u32,
    b: u32,
}

#[derive(Debug)]
struct Game {
    id: i32,
    reveals: Vec<Rgb>,
}

// -- Implementation -----------------------------------------------------------

impl Default for Rgb {
    fn default() -> Self {
        Rgb { r: 0, g: 0, b: 0 }
    }
}

impl Rgb {
    fn new(r: u32, g: u32, b: u32) -> Self {
        Rgb { r, g, b }
    }

    fn red(i: u32) -> Self {
        Rgb { r: i, g: 0, b: 0 }
    }

    fn green(i: u32) -> Self {
        Rgb { r: 0, g: i, b: 0 }
    }

    fn blue(i: u32) -> Self {
        Rgb { r: 0, g: 0, b: i }
    }

    fn combine(&self, other: &Rgb) -> Self {
        Rgb {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }

    fn power(&self) -> u32 {
        self.r * self.g * self.b
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let red = map(terminated(digit1, tag(" red")), |s: &str| {
            Rgb::red(s.parse::<u32>().unwrap())
        });
        let green = map(terminated(digit1, tag(" green")), |s: &str| {
            Rgb::green(s.parse::<u32>().unwrap())
        });
        let blue = map(terminated(digit1, tag(" blue")), |s: &str| {
            Rgb::blue(s.parse::<u32>().unwrap())
        });

        fold_many1(
            terminated(alt((red, green, blue)), opt(tag(", "))),
            Rgb::default,
            |a, b| a.combine(&b),
        )(input)
    }
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        pair(
            map(delimited(tag("Game "), digit1, tag(": ")), |s: &str| {
                s.parse::<i32>().unwrap()
            }),
            fold_many1(
                terminated(Rgb::parse, opt(tag("; "))),
                Vec::new,
                |mut a, b| {
                    a.push(b);
                    a
                },
            ),
        )(input)
        .map(|(rest, (id, reveals))| (rest, Game { id, reveals }))
    }
}

// -- Tests --------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_parse() -> Result<()> {
        let (rest, rgb) = Rgb::parse("1 red, 2 green, 3 blue")?;

        assert_eq!(rest, "");
        assert_eq!(rgb, Rgb::new(1, 2, 3));

        let (rest, rgb) = Rgb::parse("1 blue, 2 red")?;

        assert_eq!(rest, "");
        assert_eq!(rgb, Rgb::new(2, 0, 1));

        Ok(())
    }

    #[test]
    fn test_game_parse() -> Result<()> {
        let (rest, game) = Game::parse(
            "Game 1: 1 red, 2 green, 3 blue; 4 red, 5 green, 6 blue",
        )?;

        assert_eq!(rest, "");
        assert_eq!(game.id, 1);
        assert_eq!(game.reveals, vec![Rgb::new(1, 2, 3), Rgb::new(4, 5, 6)]);

        Ok(())
    }
}

// -- Main ---------------------------------------------------------------------

fn part1(games: Vec<Game>) -> i32 {
    let mut id_sum = 0;
    for game in &games {
        if game
            .reveals
            .iter()
            .all(|reveal| reveal.r <= 12 && reveal.g <= 13 && reveal.b <= 14)
        {
            id_sum += game.id;
        }
    }
    id_sum
}

fn part2(games: Vec<Game>) -> u32 {
    let mut powers = 0;
    for game in &games {
        powers += game
            .reveals
            .iter()
            .fold(Rgb::default(), |a, b| {
                Rgb::new(max(a.r, b.r), max(a.g, b.g), max(a.b, b.b))
            })
            .power()
    }
    powers
}

pub fn main() -> Result<()> {
    let reader = BufReader::new(File::open("data/input/2.txt")?);
    let games = reader
        .lines()
        .map(|line| Game::parse(&line.unwrap()).unwrap().1)
        .collect::<Vec<_>>();

    println!("{}", part2(games));
    Ok(())
}
