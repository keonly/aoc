use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::slice::Iter;
use std::str::FromStr;

advent_of_code::solution!(20);

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

fn manhattan_dist(coord1: Coord, coord2: Coord) -> usize {
    (coord1.0).abs_diff(coord2.0) + (coord1.1).abs_diff(coord2.1)
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
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

    fn to_offset(&self) -> [isize; 2] {
        match self {
            Direction::North => [-1, 0],
            Direction::East => [0, 1],
            Direction::South => [1, 0],
            Direction::West => [0, -1],
        }
    }
}

type Step = usize;

#[derive(Debug)]
struct GridData {
    num_rows: usize,
    num_cols: usize,
    start: Coord,
    end: Coord,
    tracks: HashSet<Coord>,
}

impl FromStr for GridData {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start_opt: Option<Coord> = None;
        let mut end_opt: Option<Coord> = None;
        let mut tracks: HashSet<Coord> = HashSet::new();

        let lines: Vec<&str> = s.lines().collect();
        let num_rows = lines.len();
        let num_cols = lines[0].len();

        for (row_idx, line) in lines.iter().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                let coord = Coord(row_idx, col_idx);
                match c {
                    'S' => {
                        start_opt = Some(coord);
                        tracks.insert(coord);
                    }
                    'E' => {
                        end_opt = Some(coord);
                        tracks.insert(coord);
                    }
                    '.' => {
                        tracks.insert(coord);
                    }
                    _ => {}
                }
            }
        }

        let start = start_opt.ok_or("Starting position must exist")?;
        let end = end_opt.ok_or("Ending position must exist")?;

        Ok(Self {
            num_rows,
            num_cols,
            start,
            end,
            tracks,
        })
    }
}

impl GridData {
    fn in_range(&self, coord: Coord) -> bool {
        coord.0 < self.num_rows && coord.1 < self.num_cols
    }

    fn count_efficient_cheats(&self, allowed_cheat_time: usize) -> usize {
        let mut visiting: VecDeque<(Step, Coord)> = VecDeque::new();
        let mut visited: HashMap<Coord, Step> = HashMap::new();

        visiting.push_back((0, self.end));
        visited.insert(self.end, 0);

        while let Some((curr_step, curr_coord)) = visiting.pop_front() {
            if curr_coord == self.start {
                break;
            }

            for dir in Direction::iter() {
                if let Some(next_coord) = curr_coord.proceed(dir) {
                    if self.in_range(next_coord) && self.tracks.contains(&next_coord) {
                        let next_step = curr_step + 1;
                        visited.entry(next_coord).or_insert_with(|| {
                            visiting.push_back((curr_step + 1, next_coord));
                            next_step
                        });
                    }
                }
            }
        }

        visited
            .iter()
            .tuple_combinations()
            .filter(|((&coord1, &step1), (&coord2, &step2))| {
                let dist = manhattan_dist(coord1, coord2);
                dist <= allowed_cheat_time && step1.abs_diff(step2).abs_diff(dist) >= 100
            })
            .count()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid_data = GridData::from_str(input).expect("Grid data should be parseable");

    Some(grid_data.count_efficient_cheats(2))
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid_data = GridData::from_str(input).expect("Grid data should be parseable");

    Some(grid_data.count_efficient_cheats(20))
}
