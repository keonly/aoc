use itertools::iproduct;
use std::str::FromStr;

advent_of_code::solution!(25);

#[derive(Debug)]
enum ParsedType {
    Lock(Lock),
    Key(Key),
}

#[derive(Debug, Clone)]
struct Lock {
    heights: Vec<u8>,
}

impl FromStr for Lock {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().skip(1).collect();
        let num_cols: usize = lines.first().map_or(0, |line| line.len());

        let heights: Vec<u8> = (0..num_cols)
            .map(|col| {
                lines
                    .iter()
                    .filter(|line| line.chars().nth(col) == Some('#'))
                    .count() as u8
            })
            .collect();

        Ok(Self { heights })
    }
}

#[derive(Debug, Clone)]
struct Key {
    heights: Vec<u8>,
}

impl FromStr for Key {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().rev().skip(1).collect();
        let num_cols: usize = lines.first().map_or(0, |line| line.len());

        let heights: Vec<u8> = (0..num_cols)
            .map(|col| {
                lines
                    .iter()
                    .filter(|line| line.chars().nth(col) == Some('#'))
                    .count() as u8
            })
            .collect();

        Ok(Self { heights })
    }
}

fn parse_input(input: &str) -> Result<ParsedType, String> {
    let mut lines = input.lines().peekable();
    if let Some(fst_line) = lines.peek() {
        if fst_line.starts_with('#') {
            Lock::from_str(input).map(ParsedType::Lock)
        } else {
            Key::from_str(input).map(ParsedType::Key)
        }
    } else {
        Err("Input is empty or invalid.".to_string())
    }
}

fn lock_key_match(lock: &Lock, key: &Key) -> bool {
    let lock_height = lock.heights.clone();
    let key_height = key.heights.clone();

    lock_height
        .iter()
        .zip(key_height.iter())
        .all(|(&h1, &h2)| h1 + h2 <= 5)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut locks: Vec<Lock> = vec![];
    let mut keys: Vec<Key> = vec![];

    for s in input.split("\n\n") {
        match parse_input(s) {
            Ok(ParsedType::Lock(lock)) => {
                locks.push(lock);
            }
            Ok(ParsedType::Key(key)) => {
                keys.push(key);
            }
            Err(e) => panic!("Failed to parse input: {}", e),
        }
    }

    let cartesian_prod = iproduct!(locks, keys);
    let matches = cartesian_prod
        .filter(|(lock, key)| lock_key_match(lock, key))
        .count();

    Some(matches)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
