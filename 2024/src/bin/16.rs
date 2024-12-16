use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::str::FromStr;

advent_of_code::solution!(16);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coord(usize, usize);

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).then_with(|| self.1.cmp(&other.1))
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn to_offset(self) -> [isize; 2] {
        match self {
            Direction::North => [-1, 0],
            Direction::East => [0, 1],
            Direction::South => [1, 0],
            Direction::West => [0, -1],
        }
    }
    fn turn_cw(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn turn_ccw(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Coord,
    direction: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct GridData {
    num_rows: usize,
    num_cols: usize,
    start: Coord,
    end: Coord,
    tiles: HashSet<Coord>,
}

impl FromStr for GridData {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start_opt: Option<Coord> = None;
        let mut end_opt: Option<Coord> = None;
        let mut tiles: HashSet<Coord> = HashSet::new();

        let lines: Vec<&str> = s.lines().collect();
        let num_rows = lines.len();
        let num_cols = lines[0].len();

        for (row_idx, line) in lines.iter().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                let coord = Coord(row_idx, col_idx);
                match c {
                    'S' => {
                        start_opt = Some(coord);
                        tiles.insert(coord);
                    }
                    'E' => {
                        end_opt = Some(coord);
                        tiles.insert(coord);
                    }
                    '.' => {
                        tiles.insert(coord);
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
            tiles,
        })
    }
}

impl GridData {
    fn proceed(&self, coord: Coord, direction: &Direction) -> Option<Coord> {
        let [d_row, d_col] = direction.to_offset();
        let Coord(curr_row, curr_col) = coord;
        let next_row = (curr_row as isize) + d_row;
        let next_col = (curr_col as isize) + d_col;

        if 0 <= next_row
            && (next_row as usize) < self.num_rows
            && 0 <= next_col
            && (next_col as usize) < self.num_cols
        {
            let next_coord = Coord(next_row as usize, next_col as usize);
            if self.tiles.contains(&next_coord) {
                Some(next_coord)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_adjacent_nodes(&self, state: State) -> Vec<State> {
        let mut adjacents = vec![];

        let State {
            cost: curr_cost,
            position: curr_pos,
            direction: curr_dir,
        } = state;

        let next_cw_dir = curr_dir.turn_cw();
        let next_ccw_dir = curr_dir.turn_ccw();

        if let Some(next_pos) = self.proceed(curr_pos, &curr_dir) {
            adjacents.push(State {
                cost: curr_cost + 1,
                position: next_pos,
                direction: curr_dir,
            });
        }

        if let Some(next_cw_pos) = self.proceed(curr_pos, &next_cw_dir) {
            adjacents.push(State {
                cost: curr_cost + 1001,
                position: next_cw_pos,
                direction: next_cw_dir,
            });
        }

        if let Some(next_ccw_pos) = self.proceed(curr_pos, &next_ccw_dir) {
            adjacents.push(State {
                cost: curr_cost + 1001,
                position: next_ccw_pos,
                direction: next_ccw_dir,
            });
        }

        adjacents
    }

    fn get_least_cost(&self) -> Option<usize> {
        let mut dists: HashMap<(Coord, Direction), usize> = HashMap::new();
        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        dists.insert((self.start, Direction::East), 0);
        heap.push(State {
            cost: 0,
            position: self.start,
            direction: Direction::East,
        });

        while let Some(curr_state) = heap.pop() {
            if curr_state.position == self.end {
                return Some(curr_state.cost);
            }

            if curr_state.cost
                > *dists
                    .get(&(curr_state.position, curr_state.direction))
                    .unwrap_or(&usize::MAX)
            {
                continue;
            }

            for next_state in self.get_adjacent_nodes(curr_state) {
                if next_state.cost
                    < *dists
                        .get(&(next_state.position, next_state.direction))
                        .unwrap_or(&usize::MAX)
                {
                    heap.push(next_state);
                    dists.insert((next_state.position, next_state.direction), next_state.cost);
                }
            }
        }

        None
    }

    fn backtrack(
        &self,
        prevs: &HashMap<(Coord, Direction), HashSet<(Coord, Direction)>>,
        end_state: (Coord, Direction),
    ) -> usize {
        let mut visiting: VecDeque<(Coord, Direction)> = VecDeque::from([end_state]);
        let mut visited_states: HashSet<(Coord, Direction)> = HashSet::new();
        let mut visited_coords: HashSet<Coord> = HashSet::new();

        while let Some(state) = visiting.pop_front() {
            if !visited_states.insert(state) {
                continue;
            }
            visited_coords.insert(state.0);

            if let Some(predecessors) = prevs.get(&state) {
                for &prev_state in predecessors {
                    if !visited_states.contains(&prev_state) {
                        visiting.push_back(prev_state);
                    }
                }
            }
        }

        visited_coords.len()
    }

    fn get_best_path_passthrough_count(&self) -> Option<usize> {
        let mut dists: HashMap<(Coord, Direction), usize> = HashMap::new();
        let mut prevs: HashMap<(Coord, Direction), HashSet<(Coord, Direction)>> = HashMap::new();
        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        dists.insert((self.start, Direction::East), 0);
        heap.push(State {
            cost: 0,
            position: self.start,
            direction: Direction::East,
        });

        while let Some(curr_state) = heap.pop() {
            if curr_state.position == self.end {
                return Some(self.backtrack(&prevs, (curr_state.position, curr_state.direction)));
            }

            if curr_state.cost
                > *dists
                    .get(&(curr_state.position, curr_state.direction))
                    .unwrap_or(&usize::MAX)
            {
                continue;
            }

            for next_state in self.get_adjacent_nodes(curr_state) {
                match next_state.cost.cmp(
                    dists
                        .get(&(next_state.position, next_state.direction))
                        .unwrap_or(&usize::MAX),
                ) {
                    Ordering::Less => {
                        prevs.insert(
                            (next_state.position, next_state.direction),
                            HashSet::from([(curr_state.position, curr_state.direction)]),
                        );
                        heap.push(next_state);
                        dists.insert((next_state.position, next_state.direction), next_state.cost);
                    }
                    Ordering::Equal => {
                        prevs
                            .entry((next_state.position, next_state.direction))
                            .and_modify(|prev| {
                                prev.insert((curr_state.position, curr_state.direction));
                            });
                    }
                    _ => {}
                }
            }
        }
        None
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid_data = GridData::from_str(input).expect("Input should be parseable");

    let result = grid_data
        .get_least_cost()
        .expect("At least one path should exist");
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid_data = GridData::from_str(input).expect("Input should be parseable");

    let result = grid_data
        .get_best_path_passthrough_count()
        .expect("At least one path should exist");
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
