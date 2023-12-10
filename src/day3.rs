use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

// -- Types --------------------------------------------------------------------

type Scheme = Vec<Vec<char>>;

#[derive(Debug)]
struct PartNumber<'a> {
    scheme: &'a Scheme,
    line: usize,
    begin: usize,
    end: usize,
}

#[derive(Debug, PartialEq)]
struct Part {
    symbol: char,
    i: usize,
    j: usize,
}

// -- Implementation -----------------------------------------------------------

impl Part {
    fn new(symbol: char, i: usize, j: usize) -> Self {
        Self { symbol, i, j }
    }
}

impl<'a> PartNumber<'a> {
    fn new(scheme: &'a Scheme, line: usize, begin: usize, end: usize) -> Self {
        Self {
            scheme,
            line,
            begin,
            end,
        }
    }

    fn to_string(&self) -> String {
        self.scheme[self.line][self.begin..self.end]
            .iter()
            .collect()
    }

    fn to_number(&self) -> i32 {
        self.to_string().parse().unwrap()
    }

    fn get_parts(&self) -> Vec<Part> {
        let mut parts = Vec::new();
        let line = self.line;
        let mut begin = self.begin;
        let mut end = self.end;

        if begin > 0 {
            begin -= 1;
        }

        if end < self.scheme[0].len() {
            end += 1;
        }

        if is_part(&self.scheme[line][begin]) {
            parts.push(Part::new(self.scheme[line][begin], line, begin));
        }

        if is_part(&self.scheme[line][end - 1]) {
            parts.push(Part::new(self.scheme[line][end - 1], line, end - 1));
        }

        if line > 0 {
            for j in begin..end {
                if is_part(&self.scheme[line - 1][j]) {
                    parts.push(Part::new(
                        self.scheme[line - 1][j],
                        line - 1,
                        j,
                    ));
                }
            }
        }

        if line < self.scheme.len() - 1 {
            for j in begin..end {
                if is_part(&self.scheme[line + 1][j]) {
                    parts.push(Part::new(
                        self.scheme[line + 1][j],
                        line + 1,
                        j,
                    ));
                }
            }
        }

        parts
    }
}

// -- Helpers ------------------------------------------------------------------

fn read_scheme(reader: impl BufRead) -> Scheme {
    let mut scheme = Scheme::new();
    for line in reader.lines() {
        scheme.push(line.unwrap().chars().collect());
    }
    scheme
}

fn is_part(c: &char) -> bool {
    !c.is_digit(10) && *c != '.'
}

fn is_part_number(part: &PartNumber) -> bool {
    let parts = part.get_parts();
    !parts.is_empty()
}

fn get_part_numbers(scheme: &Scheme) -> Vec<PartNumber> {
    let mut part_numbers = Vec::new();
    for (i, line) in scheme.iter().enumerate() {
        let mut begin = 0;
        let mut end = 0;
        for (j, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                end = j + 1;
            } else {
                if end > begin {
                    let part_numer = PartNumber::new(scheme, i, begin, end);
                    if is_part_number(&part_numer) {
                        part_numbers.push(part_numer);
                    }
                }
                begin = j + 1;
            }
        }
        if end > begin {
            let part_numer = PartNumber::new(scheme, i, begin, end);
            if is_part_number(&part_numer) {
                part_numbers.push(part_numer);
            }
        }
    }
    part_numbers
}

fn find_gears_ratio(scheme: &Scheme) -> i32 {
    let mut gears = HashMap::<(usize, usize), Vec<i32>>::new();
    let part_numbers = get_part_numbers(scheme);

    for part_number in part_numbers {
        for part in part_number.get_parts() {
            if part.symbol == '*' {
                let parts: &mut Vec<i32> =
                    gears.entry((part.i, part.j)).or_default();
                parts.push(part_number.to_number());
            }
        }
    }

    gears
        .iter()
        .filter(|(_, parts)| parts.len() == 2)
        .map(|(_, parts)| parts[0] * parts[1])
        .sum()
}

// -- Tests --------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_part_number() {
        let scheme = vec![
            "...12..*..".chars().collect::<Vec<_>>(),
            "10...+..11".chars().collect::<Vec<_>>(),
            "#......8..".chars().collect::<Vec<_>>(),
        ];

        assert!(is_part_number(&PartNumber::new(&scheme, 0, 3, 5)));
        assert!(is_part_number(&PartNumber::new(&scheme, 1, 0, 2)));
        assert!(is_part_number(&PartNumber::new(&scheme, 1, 8, 10)));
        assert!(!is_part_number(&PartNumber::new(&scheme, 2, 7, 8)));
    }

    #[test]
    fn test_get_part_numbers() {
        let scheme = vec![
            "...12..*..".chars().collect::<Vec<_>>(),
            "10...+..11".chars().collect::<Vec<_>>(),
            "#......8..".chars().collect::<Vec<_>>(),
        ];

        assert_eq!(
            get_part_numbers(&scheme)
                .iter()
                .map(PartNumber::to_number)
                .collect::<Vec<_>>(),
            vec![12, 10, 11]
        );
    }

    #[test]
    fn test_get_parts() {
        let scheme = vec![
            "...12..*..".chars().collect::<Vec<_>>(),
            "10*..+5.11".chars().collect::<Vec<_>>(),
            "#....7*8..".chars().collect::<Vec<_>>(),
        ];

        let seven = PartNumber::new(&scheme, 2, 5, 6);
        assert_eq!(
            seven.get_parts(),
            vec![Part::new('*', 2, 6), Part::new('+', 1, 5)]
        );
    }

    #[test]
    fn test_find_gears_ratio() {
        let scheme = vec![
            "...12..*..".chars().collect::<Vec<_>>(),
            "10*..+5.11".chars().collect::<Vec<_>>(),
            "#....7*8..".chars().collect::<Vec<_>>(),
        ];

        assert_eq!(find_gears_ratio(&scheme), 175);
    }
}

// -- Main ---------------------------------------------------------------------

pub fn main() -> Result<()> {
    let scheme = read_scheme(BufReader::new(File::open("data/input/3.txt")?));
    println!("{}", find_gears_ratio(&scheme));
    Ok(())
}
