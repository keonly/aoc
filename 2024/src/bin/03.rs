#![feature(str_as_str)]
use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(pattern).expect("Invalid regex");

    Some(
        re.captures_iter(input)
            .map(|cap| cap.extract())
            .map(|(_, [d1, d2])| {
                let num1 = d1.parse::<u32>().unwrap();
                let num2 = d2.parse::<u32>().unwrap();
                num1 * num2
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let pattern = r"(?P<operator>mul\((?P<d1>\d{1,3}),(?P<d2>\d{1,3})\)|do\(\)|don't\(\))";
    let re = Regex::new(pattern).expect("Invalid regex");

    Some(
        re.captures_iter(input)
            .fold((true, 0), |acc, cap| {
                let (do_flag, sum) = acc;
                match cap["operator"].as_str() {
                    "do()" => (true, sum),
                    "don't()" => (false, sum),
                    _ => match do_flag {
                        true => {
                            let d1 = cap["d1"].as_str().parse::<u32>().unwrap();
                            let d2 = cap["d2"].as_str().parse::<u32>().unwrap();
                            (do_flag, sum + d1 * d2)
                        }
                        false => (do_flag, sum),
                    },
                }
            })
            .1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
