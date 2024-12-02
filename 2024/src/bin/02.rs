#![feature(iter_map_windows)]
advent_of_code::solution!(2);

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Inc,
    Dec,
}

fn is_safe(nums: &[u32]) -> bool {
    let mut change: Option<Direction> = None;

    nums.iter()
        .map_windows(|[&fst, &snd]| {
            if !(0 < fst.abs_diff(snd) && fst.abs_diff(snd) < 4) {
                false
            } else {
                match change {
                    None => {
                        change = if snd > fst {
                            Some(Direction::Inc)
                        } else {
                            Some(Direction::Dec)
                        };
                        true
                    }
                    Some(Direction::Inc) => snd > fst,
                    Some(Direction::Dec) => snd < fst,
                }
            }
        })
        .all(|x| x)
}

fn exclude(nums: &[u32]) -> Vec<Vec<u32>> {
    nums.iter()
        .enumerate()
        .map(|(i, _)| {
            nums.iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, &val)| val)
                .collect()
        })
        .collect()
}

fn is_safe_with_dampener(nums: &[u32]) -> bool {
    let excluded = exclude(nums);

    excluded.iter().any(|v| is_safe(v))
}

pub fn part_one(input: &str) -> Option<u32> {
    let safe_count = input
        .lines()
        .filter(|line| {
            let nums = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            is_safe(&nums)
        })
        .count();

    Some(safe_count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_count = input
        .lines()
        .filter(|line| {
            let nums = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            is_safe_with_dampener(&nums)
        })
        .count();

    Some(safe_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
