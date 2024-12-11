#![feature(let_chains)]
use std::collections::{HashMap, HashSet, VecDeque};
use std::slice::Iter;

advent_of_code::solution!(10);

const RADIX: u32 = 10;

#[derive(Hash, Eq, PartialEq, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
        DIRECTIONS.iter()
    }

    pub fn to_offset(&self) -> [isize; 2] {
        match self {
            Direction::North => [-1, 0],
            Direction::East => [0, 1],
            Direction::South => [1, 0],
            Direction::West => [0, -1],
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coord(usize, usize);

impl Coord {
    fn proceed(&self, direction: &Direction) -> Option<Self> {
        let [d_row, d_col] = direction.to_offset();
        let &Coord(curr_row, curr_col) = self;
        let next_row = (curr_row as isize) + d_row;
        let next_col = (curr_col as isize) + d_col;

        if 0 <= next_row && 0 <= next_col {
            Some(Coord(next_row as usize, next_col as usize))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct GridData {
    num_rows: usize,
    num_cols: usize,
    map: HashMap<u32, HashSet<Coord>>,
}

impl GridData {
    fn from_str(input: &str) -> Self {
        let mut map: HashMap<u32, HashSet<Coord>> = HashMap::new();

        let lines: Vec<&str> = input.lines().collect();

        let num_rows = lines.len();
        let num_cols = lines[0].len();

        for (row_idx, line) in lines.iter().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                let height: u32 = c.to_digit(RADIX).expect("Digit should be parseable");
                map.entry(height)
                    .or_default()
                    .insert(Coord(row_idx, col_idx));
            }
        }

        GridData {
            num_rows,
            num_cols,
            map,
        }
    }

    fn in_board(&self, coord: &Coord) -> bool {
        coord.0 < self.num_rows && coord.1 < self.num_cols
    }

    fn traverse_grid(&self, start: &Coord, count_unique: bool) -> u32 {
        let mut visiting = VecDeque::new();
        let mut visited = HashSet::new();
        let mut count = 0;

        visiting.push_back((0, *start));

        while let Some((curr_height, curr_coord)) = visiting.pop_back() {
            if curr_height == 9 {
                if !count_unique || visited.insert(curr_coord) {
                    count += 1;
                }
            } else {
                let next_height = curr_height + 1;
                for dir in Direction::iter() {
                    if let Some(next_coord) = curr_coord.proceed(dir) {
                        if self.in_board(&next_coord)
                            && self
                                .map
                                .get(&next_height)
                                .is_some_and(|set| set.contains(&next_coord))
                        {
                            visiting.push_back((next_height, next_coord));
                        }
                    }
                }
            }
        }

        count
    }

    fn calculate_trailhead(&self, start: &Coord) -> u32 {
        self.traverse_grid(start, true)
    }

    fn calculate_rating(&self, start: &Coord) -> u32 {
        self.traverse_grid(start, false)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid_data = GridData::from_str(input);

    let result: u32 = grid_data
        .map
        .get(&0)
        .expect("At least one starting point expected")
        .iter()
        .map(|start| grid_data.calculate_trailhead(start))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid_data = GridData::from_str(input);

    let result: u32 = grid_data
        .map
        .get(&0)
        .expect("At least one starting point expected")
        .iter()
        .map(|start| grid_data.calculate_rating(start))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
