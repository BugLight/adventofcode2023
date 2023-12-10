use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

// -- Types --------------------------------------------------------------------

type Scheme = Vec<Vec<char>>;

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

fn is_part_number(scheme: &Scheme, line: usize, mut begin: usize, mut end: usize) -> bool {
    if begin > 0 {
        begin -= 1;
    }

    if end < scheme[0].len() {
        end += 1;
    }

    if is_part(&scheme[line][begin]) || is_part(&scheme[line][end - 1]) {
        return true;
    }

    if line > 0 && scheme[line - 1][begin..end].iter().any(is_part) {
        return true;
    }

    if line < scheme.len() - 1 && scheme[line + 1][begin..end].iter().any(is_part) {
        return true;
    }

    false
}

fn get_part_numbers(scheme: &Scheme) -> Vec<u32> {
    let mut part_numbers = Vec::new();
    for (i, line) in scheme.iter().enumerate() {
        let mut begin = 0;
        let mut end = 0;
        for (j, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                end = j + 1;
            } else {
                if end > begin && is_part_number(&scheme, i, begin, end) {
                    let number = line[begin..end].iter().collect::<String>();
                    part_numbers.push(number.parse().unwrap());
                }
                begin = j + 1;
            }
        }
        if end > begin && is_part_number(&scheme, i, begin, end) {
            let number = line[begin..end].iter().collect::<String>();
            part_numbers.push(number.parse().unwrap());
        }
    }
    part_numbers
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

        assert!(is_part_number(&scheme, 0, 3, 5));
        assert!(is_part_number(&scheme, 1, 0, 2));
        assert!(is_part_number(&scheme, 1, 8, 10));
        assert!(!is_part_number(&scheme, 2, 7, 8));
    }

    #[test]
    fn test_get_part_numbers() {
        let scheme = vec![
            "...12..*..".chars().collect::<Vec<_>>(),
            "10...+..11".chars().collect::<Vec<_>>(),
            "#......8..".chars().collect::<Vec<_>>(),
        ];

        assert_eq!(get_part_numbers(&scheme), vec![12, 10, 11]);
    }
}

// -- Main ---------------------------------------------------------------------

pub fn main() -> Result<()> {
    let scheme = read_scheme(BufReader::new(File::open("data/input/3.txt")?));
    let part_numbers = get_part_numbers(&scheme);
    println!("{}", part_numbers.iter().sum::<u32>());
    Ok(())
}
