use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_input_part_one(input: &str) -> (Vec<u32>, Vec<u32>) {
    let pairs = input.lines().map(|line| {
        let mut tokens = line.split_ascii_whitespace();
        let [left, right] = [(); 2].map(|_| tokens.next().unwrap().parse::<u32>().unwrap());
        (left, right)
    });
    let (lefts, rights): (Vec<u32>, Vec<u32>) = pairs.into_iter().unzip();

    (lefts, rights)
}

fn parse_input_part_two(input: &str) -> (Vec<u32>, HashMap<u32, u32>) {
    let mut lefts = Vec::<u32>::new();
    let mut rights = HashMap::<u32, u32>::new();

    input.lines().for_each(|line| {
        let mut tokens = line.split_ascii_whitespace();
        let [left, right] = [(); 2].map(|_| tokens.next().unwrap().parse::<u32>().unwrap());

        lefts.push(left);
        if let Some(count) = rights.get(&right) {
            rights.insert(right, count + 1u32);
        } else {
            rights.insert(right, 1u32);
        }
    });

    (lefts, rights)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut lefts, mut rights) = parse_input_part_one(input);
    lefts.sort_unstable();
    rights.sort_unstable();

    let sorted = std::iter::zip(lefts, rights);
    let total_distance = sorted.fold(0, |acc, val| acc + (val.0.abs_diff(val.1)));

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (lefts, rights) = parse_input_part_two(input);
    let total_sim_scores = lefts
        .iter()
        .fold(0, |acc, val| acc + val * *(rights.get(val).unwrap_or(&0)));

    Some(total_sim_scores)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
