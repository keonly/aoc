use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;
use std::slice::Iter;
use std::str::FromStr;

advent_of_code::solution!(18);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coord(usize, usize);

impl FromStr for Coord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [col, row] = s
            .split(',')
            .map(|c| c.parse::<usize>().expect("Coordinate should be parseable"))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Only a single pair should be given at a time");

        Ok(Coord(row, col))
    }
}

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
    corrupted: HashSet<Coord>,
}

impl<'a> FromIterator<&'a str> for GridData {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut corrupted: HashSet<Coord> = HashSet::new();

        for line in iter {
            corrupted
                .insert(Coord::from_str(line).expect("Coordinate information should be parseable"));
        }

        Self {
            num_rows: 71,
            num_cols: 71,
            corrupted,
        }
    }
}

impl GridData {
    fn in_range(&self, coord: Coord) -> bool {
        coord.0 < self.num_rows && coord.1 < self.num_cols
    }

    fn bfs(&self) -> Result<usize, ()> {
        let mut visiting: VecDeque<(Step, Coord)> = VecDeque::new();
        let mut visited: HashSet<Coord> = HashSet::new();

        visiting.push_back((0, Coord(0, 0)));
        visited.insert(Coord(0, 0));

        while let Some((curr_step, curr_coord)) = visiting.pop_front() {
            if curr_coord == Coord(self.num_rows - 1, self.num_cols - 1) {
                return Ok(curr_step);
            }

            for dir in Direction::iter() {
                if let Some(next_coord) = curr_coord.proceed(dir) {
                    if self.in_range(next_coord)
                        && !self.corrupted.contains(&next_coord)
                        && visited.insert(next_coord)
                    {
                        visiting.push_back((curr_step + 1, next_coord));
                    }
                }
            }
        }

        Err(())
    }
}

fn binary_search(grid_data_vec: &mut [GridData]) -> usize {
    let mut lo = 0;
    let mut hi = grid_data_vec.len() - 1;

    while lo < hi {
        let mid = lo + (hi - lo) / 2;

        match grid_data_vec[mid].bfs() {
            Ok(_) => {
                lo = mid + 1;
            }
            Err(_) => {
                hi = mid;
            }
        }
    }

    lo
}

pub fn part_one(input: &str) -> Option<usize> {
    let fallen_bytes = input.lines().take(1024);
    let grid_data = GridData::from_iter(fallen_bytes);
    let result = grid_data.bfs().expect("A path must exist at time 1024");

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let falling_bytes: Vec<_> = input.lines().collect();
    let (initial, rest) = falling_bytes.split_at(1024);
    let grid_data = GridData::from_iter(initial.iter().cloned());

    let mut grid_data_vec: Vec<_> = rest
        .iter()
        .scan(grid_data.corrupted, |state, item| {
            let coord = Coord::from_str(item).expect("Coordinate information should be parseable");
            state.insert(coord);
            Some(GridData {
                num_rows: 71,
                num_cols: 71,
                corrupted: state.clone(),
            })
        })
        .collect();

    let index = binary_search(&mut grid_data_vec);
    Some(rest[index].to_string())
}
