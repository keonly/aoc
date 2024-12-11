use cached::proc_macro::cached;

advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().expect("Should be parseable as usize"))
        .collect::<Vec<usize>>()
}

#[cached]
fn blink(stone: usize, steps_left: usize) -> usize {
    if steps_left == 0 {
        return 1;
    }

    let next_steps = steps_left - 1;
    if stone == 0 {
        return blink(1, next_steps);
    }

    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let (left, right) = stone_str.split_at(stone_str.len() / 2);
        let left_num = left.parse::<usize>().expect("Invalid left number");
        let right_num = right.parse::<usize>().expect("Invalid right number");
        return blink(left_num, next_steps) + blink(right_num, next_steps);
    }

    blink(stone * 2024, next_steps)
}

pub fn part_one(input: &str) -> Option<usize> {
    let stones: Vec<usize> = parse_input(input);
    let result: usize = stones.iter().map(|&stone| blink(stone, 25)).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let stones: Vec<usize> = parse_input(input);
    let result: usize = stones.iter().map(|&stone| blink(stone, 75)).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
