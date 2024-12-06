use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Coord(usize, usize);

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn turn(dir: &Self) -> Self {
        match dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn proceed(dir: &Self) -> [isize; 2] {
        match dir {
            Direction::North => [-1, 0],
            Direction::East => [0, 1],
            Direction::South => [1, 0],
            Direction::West => [0, -1],
        }
    }
}

struct GridData {
    num_rows: usize,
    num_cols: usize,
    guard_position: Coord,
    wall_positions: HashSet<Coord>,
}

impl GridData {
    fn from_str(input: &str) -> Self {
        let mut guard_position_opt = None;
        let mut wall_positions: HashSet<Coord> = HashSet::new();

        let lines: Vec<&str> = input.lines().collect();

        let num_rows = lines.len();
        let num_cols = lines[0].len();

        for (row_idx, line) in lines.iter().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        wall_positions.insert(Coord(row_idx, col_idx));
                    }
                    '^' => {
                        guard_position_opt = Some(Coord(row_idx, col_idx));
                    }
                    _ => {}
                }
            }
        }

        let guard_position = guard_position_opt.expect("Exactly one guard expected");

        GridData {
            num_rows,
            num_cols,
            guard_position,
            wall_positions,
        }
    }

    fn in_board(&self, coord: &Coord) -> bool {
        coord.0 < self.num_rows && coord.1 < self.num_cols
    }

    fn check_loop_exist_with_new_wall(&self, new_wall: Coord) -> bool {
        let mut visited: HashSet<(Coord, Direction)> = HashSet::new();
        let mut curr_pos: Coord = self.guard_position;
        let mut curr_dir = Direction::North;

        while self.in_board(&curr_pos) {
            if visited.contains(&(curr_pos, curr_dir)) {
                return true;
            }
            visited.insert((curr_pos, curr_dir));

            let next_pos = Coord(
                (curr_pos.0 as isize + Direction::proceed(&curr_dir)[0]) as usize,
                (curr_pos.1 as isize + Direction::proceed(&curr_dir)[1]) as usize,
            );

            if self.wall_positions.contains(&next_pos) || next_pos == new_wall {
                curr_dir = Direction::turn(&curr_dir);
            } else {
                curr_pos = next_pos;
            }
        }

        false
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid_data = GridData::from_str(input);

    let mut visited: HashSet<Coord> = HashSet::new();
    let mut curr_pos: Coord = grid_data.guard_position;
    let mut curr_dir = Direction::North;

    while grid_data.in_board(&curr_pos) {
        visited.insert(curr_pos);

        let next_pos = Coord(
            (curr_pos.0 as isize + Direction::proceed(&curr_dir)[0]) as usize,
            (curr_pos.1 as isize + Direction::proceed(&curr_dir)[1]) as usize,
        );

        if grid_data.wall_positions.contains(&next_pos) {
            curr_dir = Direction::turn(&curr_dir);
        } else {
            curr_pos = next_pos;
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid_data = GridData::from_str(input);

    let count = (0..grid_data.num_rows)
        .flat_map(|i| (0..grid_data.num_cols).map(move |j| (i, j)))
        .filter(|&(row_idx, col_idx)| {
            !grid_data.wall_positions.contains(&Coord(row_idx, col_idx)) && {
                grid_data.check_loop_exist_with_new_wall(Coord(row_idx, col_idx))
            }
        })
        .count();

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
