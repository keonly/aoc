#![feature(iter_array_chunks)]
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

advent_of_code::solution!(15);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coord(usize, usize);

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '^' => Ok(Self::North),
            'v' => Ok(Self::South),
            '<' => Ok(Self::West),
            '>' => Ok(Self::East),
            _ => Err("No matching direction".to_string()),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Entity {
    Wall,
    Box,
    Robot,
    Empty,
}

impl Entity {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '#' => Ok(Self::Wall),
            'O' => Ok(Self::Box),
            '@' => Ok(Self::Robot),
            '.' => Ok(Self::Empty),
            _ => Err(format!("Unknown entry: {}", c)),
        }
    }
}

#[derive(Debug)]
struct GridData {
    num_rows: usize,
    num_cols: usize,
    map: HashMap<Coord, Entity>,
}

impl FromStr for GridData {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<Coord, Entity> = HashMap::new();

        let lines: Vec<&str> = s.lines().collect();

        let num_rows = lines.len();
        let num_cols = lines[0].len();

        for (row_idx, line) in lines.iter().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                let entity = Entity::from_char(c)?;
                map.insert(Coord(row_idx, col_idx), entity);
            }
        }

        Ok(GridData {
            num_rows,
            num_cols,
            map,
        })
    }
}

impl GridData {
    fn get_next_coord(&self, current: Coord, direction: &Direction) -> Option<Coord> {
        match direction {
            Direction::North => {
                if current.0 > 0 {
                    Some(Coord(current.0 - 1, current.1))
                } else {
                    None
                }
            }
            Direction::South => {
                if current.0 + 1 < self.num_rows {
                    Some(Coord(current.0 + 1, current.1))
                } else {
                    None
                }
            }
            Direction::West => {
                if current.1 > 0 {
                    Some(Coord(current.0, current.1 - 1))
                } else {
                    None
                }
            }
            Direction::East => {
                if current.1 + 1 < self.num_cols {
                    Some(Coord(current.0, current.1 + 1))
                } else {
                    None
                }
            }
        }
    }

    fn move_robot(&mut self, direction: Direction) {
        let robot_position = self
            .map
            .iter()
            .find(|(_, entity)| matches!(entity, Entity::Robot))
            .map(|(coord, _)| *coord);

        if let Some(robot_pos) = robot_position {
            let mut next_pos = robot_pos;
            let mut pushable_boxes = vec![];

            while let Some(new_pos) = self.get_next_coord(next_pos, &direction) {
                match self.map.get(&new_pos) {
                    Some(Entity::Empty) => {
                        break;
                    }
                    Some(Entity::Box) => {
                        pushable_boxes.push(new_pos);
                        next_pos = new_pos;
                    }
                    _ => {
                        return;
                    }
                }
            }

            if !pushable_boxes.is_empty() {
                if let Some(new_pos) = self.get_next_coord(next_pos, &direction) {
                    self.map.insert(new_pos, Entity::Box);

                    for &box_pos in pushable_boxes.iter().rev() {
                        let box_next_pos = self.get_next_coord(box_pos, &direction).unwrap();
                        self.map.insert(box_next_pos, Entity::Box);
                    }

                    self.map.insert(pushable_boxes[0], Entity::Robot);
                    self.map.insert(robot_pos, Entity::Empty);
                }
            } else if let Some(new_pos) = self.get_next_coord(robot_pos, &direction) {
                if let Some(Entity::Empty) = self.map.get(&new_pos) {
                    self.map.insert(new_pos, Entity::Robot);
                    self.map.insert(robot_pos, Entity::Empty);
                }
            }
        }
    }

    #[allow(dead_code)]
    fn print_map(&self) {
        for r in 0..self.num_rows {
            let mut line = String::new();
            for c in 0..self.num_cols {
                let coord = Coord(r, c);
                let entity = self.map.get(&coord).unwrap_or(&Entity::Empty);
                let chars = match entity {
                    Entity::Wall => "#",
                    Entity::Box => "O",
                    Entity::Robot => "@",
                    Entity::Empty => ".",
                };
                line.push_str(chars);
            }
            println!("{}", line);
        }
    }
}

#[derive(Debug)]
struct GridData2 {
    num_rows: usize,
    num_cols: usize,
    map: HashMap<Coord, Entity>, // Always store the left column idx
}

impl FromStr for GridData2 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<Coord, Entity> = HashMap::new();

        let lines: Vec<&str> = s.lines().collect();

        let num_rows = lines.len();
        let num_cols = lines[0].len() * 2;

        for (row_idx, line) in lines.iter().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                let coord = Coord(row_idx, col_idx * 2);
                let entity = Entity::from_char(c)?;
                map.insert(coord, entity);
            }
        }

        Ok(GridData2 {
            num_rows,
            num_cols,
            map,
        })
    }
}

impl GridData2 {
    fn get_next_coord(&self, current: Coord, direction: &Direction) -> Option<Coord> {
        match direction {
            Direction::North => {
                if current.0 > 0 {
                    Some(Coord(current.0 - 1, current.1))
                } else {
                    None
                }
            }
            Direction::South => {
                if current.0 + 1 < self.num_rows {
                    Some(Coord(current.0 + 1, current.1))
                } else {
                    None
                }
            }
            Direction::West => {
                if current.1 > 0 {
                    Some(Coord(current.0, current.1 - 1))
                } else {
                    None
                }
            }
            Direction::East => {
                if current.1 + 1 < self.num_cols {
                    Some(Coord(current.0, current.1 + 1))
                } else {
                    None
                }
            }
        }
    }

    fn get_affected_entity_coords(
        &self,
        current: Coord,
        entity: &Entity,
        direction: &Direction,
    ) -> VecDeque<Coord> {
        let mut next_coords: VecDeque<Coord> = VecDeque::new();
        let Coord(curr_row, curr_col) = current;

        match direction {
            Direction::North => {
                if curr_row > 0 {
                    let next_row = curr_row - 1;
                    let direct_next_col = curr_col;
                    next_coords.push_back(Coord(next_row, direct_next_col));
                    if direct_next_col > 0 {
                        next_coords.push_back(Coord(next_row, direct_next_col - 1));
                    }
                    if *entity == Entity::Box && direct_next_col + 1 < self.num_cols {
                        next_coords.push_back(Coord(next_row, direct_next_col + 1));
                    }
                }
            }
            Direction::South => {
                if curr_row > 0 {
                    let next_row = curr_row + 1;
                    let direct_next_col = curr_col;
                    next_coords.push_back(Coord(next_row, direct_next_col));
                    if direct_next_col > 0 {
                        next_coords.push_back(Coord(next_row, direct_next_col - 1));
                    }
                    if *entity == Entity::Box && direct_next_col + 1 < self.num_cols {
                        next_coords.push_back(Coord(next_row, direct_next_col + 1));
                    }
                }
            }
            Direction::West => {
                if curr_col > 1 {
                    let next_col = curr_col - 2;
                    next_coords.push_back(Coord(curr_row, next_col));
                }
            }
            Direction::East => match entity {
                &Entity::Box => {
                    if curr_col + 2 < self.num_cols {
                        let next_col = curr_col + 2;
                        next_coords.push_back(Coord(curr_row, next_col));
                    }
                }
                _ => {
                    if curr_col + 1 < self.num_cols {
                        let next_col = curr_col + 1;
                        next_coords.push_back(Coord(curr_row, next_col));
                    }
                }
            },
        }

        next_coords
    }

    fn move_robot(&mut self, direction: Direction) {
        let robot_pos = self
            .map
            .iter()
            .find(|(_, entity)| matches!(entity, Entity::Robot))
            .map(|(coord, _)| *coord)
            .expect("Robot position should exist");

        let mut pushable_boxes = vec![];
        let mut affected_positions =
            self.get_affected_entity_coords(robot_pos, &Entity::Robot, &direction);
        let mut can_move = true;

        while let Some(new_pos) = affected_positions.pop_front() {
            match self.map.get(&new_pos) {
                Some(Entity::Empty) | None => {
                    continue;
                }
                Some(Entity::Box) => {
                    pushable_boxes.push(new_pos);
                    affected_positions.extend(
                        self.get_affected_entity_coords(new_pos, &Entity::Box, &direction)
                            .iter(),
                    );
                }
                _ => {
                    can_move = false;
                    break;
                }
            }
        }

        if can_move {
            pushable_boxes.iter().for_each(|&pos| {
                self.map.insert(pos, Entity::Empty);
            });
            let next_bos_pos = pushable_boxes
                .iter()
                .map(|&pos| self.get_next_coord(pos, &direction).unwrap())
                .collect::<Vec<_>>();

            next_bos_pos.into_iter().for_each(|pos| {
                self.map.insert(pos, Entity::Box);
            });

            let robot_next_pos = self.get_next_coord(robot_pos, &direction).unwrap();
            self.map.insert(robot_pos, Entity::Empty);
            self.map.insert(robot_next_pos, Entity::Robot);
        }
    }

    #[allow(dead_code)]
    fn print_map(&self) {
        for r in 0..self.num_rows {
            let mut line = String::new();
            let mut c = 0;
            while c < self.num_cols {
                let coord = Coord(r, c);
                let entity = self.map.get(&coord).unwrap_or(&Entity::Empty);
                let chars = match entity {
                    Entity::Wall => "##",
                    Entity::Box => "[]",
                    Entity::Robot => "@",
                    Entity::Empty => ".",
                };
                line.push_str(chars);
                c += chars.len();
            }
            println!("{}", line);
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut descriptions = input.split("\n\n");

    let map_str = descriptions.next().expect("Map info expected");
    let move_str = descriptions.next().expect("Movement info expected");

    let mut grid_data = GridData::from_str(map_str).expect("Grid data should be parseable");
    let moves: Vec<Direction> = move_str
        .chars()
        .filter_map(|c| Direction::from_char(c).ok())
        .collect();

    for dir in moves {
        grid_data.move_robot(dir);
    }

    let result = grid_data
        .map
        .iter()
        .fold(0, |acc, (curr_coord, curr_entity)| match curr_entity {
            Entity::Box => {
                let Coord(row_idx, col_idx) = curr_coord;
                acc + 100 * row_idx + col_idx
            }
            _ => acc,
        });

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut descriptions = input.split("\n\n");

    let map_str = descriptions.next().expect("Map info expected");
    let move_str = descriptions.next().expect("Movement info expected");

    let mut grid_data = GridData2::from_str(map_str).expect("Grid data should be parseable");
    let moves: Vec<Direction> = move_str
        .chars()
        .filter_map(|c| Direction::from_char(c).ok())
        .collect();

    for dir in moves {
        grid_data.move_robot(dir);
    }

    let result = grid_data
        .map
        .iter()
        .fold(0, |acc, (curr_coord, curr_entity)| match curr_entity {
            Entity::Box => {
                let Coord(row_idx, col_idx) = curr_coord;
                acc + 100 * row_idx + col_idx
            }
            _ => acc,
        });

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
