use std::collections::HashMap;

advent_of_code::solution!(4);

const DIRECTIONS: [[[i32; 2]; 3]; 8] = [
    [[0, 1], [0, 2], [0, 3]],
    [[1, 1], [2, 2], [3, 3]],
    [[1, 0], [2, 0], [3, 0]],
    [[1, -1], [2, -2], [3, -3]],
    [[0, -1], [0, -2], [0, -3]],
    [[-1, -1], [-2, -2], [-3, -3]],
    [[-1, 0], [-2, 0], [-3, 0]],
    [[-1, 1], [-2, 2], [-3, 3]],
];

fn parse_input_into_matrix(input: &str) -> HashMap<(i32, i32), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, val)| ((x as i32, y as i32), val))
        })
        .collect::<HashMap<(i32, i32), char>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = parse_input_into_matrix(input);
    let mas = ['M', 'A', 'S'];

    let xmas_count: usize = matrix
        .iter()
        .filter(|(_, &val)| val == 'X')
        .map(|(coord, _)| {
            let x = coord.0;
            let y = coord.1;
            let count = DIRECTIONS
                .iter()
                .filter(|dir| {
                    dir.iter()
                        .enumerate()
                        .all(|(idx, [dx, dy])| matrix.get(&(x + dx, y + dy)) == Some(&mas[idx]))
                })
                .count();

            count
        })
        .sum();

    Some(xmas_count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = parse_input_into_matrix(input);

    let xmas_count: usize = matrix
        .iter()
        .filter(|(_, &val)| val == 'A')
        .filter(|(coord, _)| {
            let x = coord.0;
            let y = coord.1;

            let upper_left = matrix.get(&(x - 1, y - 1));
            let upper_right = matrix.get(&(x - 1, y + 1));
            let bottom_left = matrix.get(&(x + 1, y - 1));
            let bottom_right = matrix.get(&(x + 1, y + 1));

            matches!(
                (upper_left, upper_right, bottom_left, bottom_right),
                (Some('M'), Some('M'), Some('S'), Some('S'))
                    | (Some('M'), Some('S'), Some('M'), Some('S'))
                    | (Some('S'), Some('S'), Some('M'), Some('M'))
                    | (Some('S'), Some('M'), Some('S'), Some('M'))
            )
        })
        .count();

    Some(xmas_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
