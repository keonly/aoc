use std::collections::{HashMap, HashSet, VecDeque};
use std::slice::Iter;

advent_of_code::solution!(12);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coord(usize, usize);

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Coord {
    fn proceed(&self, direction: &Direction) -> Option<Self> {
        let [d_row, d_col] = direction.to_offset();
        let &Coord(curr_coord_row, curr_coord_col) = self;
        let next_row = (curr_coord_row as isize) + d_row;
        let next_col = (curr_coord_col as isize) + d_col;

        if 0 <= next_row && 0 <= next_col {
            Some(Coord(next_row as usize, next_col as usize))
        } else {
            None
        }
    }
}

impl Direction {
    fn iter_orth() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];
        DIRECTIONS.iter()
    }

    pub fn to_offset(self) -> [isize; 2] {
        match self {
            Direction::North => [-1, 0],
            Direction::East => [0, 1],
            Direction::South => [1, 0],
            Direction::West => [0, -1],
            Direction::NorthEast => [-1, 1],
            Direction::NorthWest => [-1, -1],
            Direction::SouthEast => [1, 1],
            Direction::SouthWest => [1, -1],
        }
    }
}

#[derive(Debug)]
struct ConnectedComponent(HashSet<Coord>);

impl ConnectedComponent {
    fn new() -> Self {
        Self(HashSet::<Coord>::new())
    }

    fn area(&self) -> usize {
        self.0.len()
    }

    fn perimeter(&self) -> usize {
        self.0.iter().fold(0, |acc, curr_coord| {
            acc + Direction::iter_orth()
                .filter(|dir| {
                    if let Some(next_coord) = curr_coord.proceed(dir) {
                        !self.0.contains(&next_coord)
                    } else {
                        true
                    }
                })
                .count()
        })
    }

    fn side(&self) -> usize {
        self.0.iter().fold(0, |acc, curr_coord| {
            let adjacents: Vec<bool> = Direction::iter_orth()
                .map(|dir| {
                    curr_coord
                        .proceed(dir)
                        .and_then(|next_coord| self.0.get(&next_coord))
                        .is_some()
                })
                .collect();

            match adjacents.as_slice() {
                [false, false, false, false] => acc + 4,
                [true, true, false, false] | [false, false, true, true] => acc,
                _ => {
                    let corners = [
                        ((0, 2), Direction::NorthEast),
                        ((0, 3), Direction::NorthWest),
                        ((1, 2), Direction::SouthEast),
                        ((1, 3), Direction::SouthWest),
                    ];

                    let convex_sides = corners
                        .iter()
                        .filter(|((i, j), _)| !adjacents[*i] && !adjacents[*j])
                        .count();

                    let concave_sides = corners
                        .iter()
                        .filter(|((i, j), diag)| {
                            adjacents[*i]
                                && adjacents[*j]
                                && curr_coord
                                    .proceed(diag)
                                    .and_then(|next_coord| self.0.get(&next_coord))
                                    .is_none()
                        })
                        .count();

                    acc + convex_sides + concave_sides
                }
            }
        })
    }
}

#[derive(Debug)]
struct GridData(HashMap<Coord, char>);

impl GridData {
    fn from_str(input: &str) -> Self {
        let map: HashMap<Coord, char> = input
            .lines()
            .enumerate()
            .flat_map(|(row_idx, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col_idx, c)| (Coord(row_idx, col_idx), c))
            })
            .collect();

        GridData(map)
    }

    fn bfs(&self, start: Coord, visited: &mut ConnectedComponent) -> ConnectedComponent {
        let target_plant: char = *self.0.get(&start).expect("Plant type expected");
        let mut component: ConnectedComponent = ConnectedComponent::new();
        let mut visiting: VecDeque<Coord> = VecDeque::new();
        visiting.push_back(start);

        while let Some(curr_coord) = visiting.pop_front() {
            if !visited.0.insert(curr_coord) {
                continue;
            }

            component.0.insert(curr_coord);

            if let Some(&_) = self.0.get(&curr_coord) {
                for dir in Direction::iter_orth() {
                    if let Some(next_coord) = curr_coord.proceed(dir) {
                        match self.0.get(&next_coord) {
                            Some(plant) if plant == &target_plant => {
                                visiting.push_back(next_coord);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        component
    }

    fn get_connected_components(&self) -> Vec<ConnectedComponent> {
        let mut visited: ConnectedComponent = ConnectedComponent::new();
        let mut components: Vec<ConnectedComponent> = Vec::new();

        for &coord in self.0.keys() {
            if !visited.0.contains(&coord) {
                let component = self.bfs(coord, &mut visited);
                components.push(component);
            }
        }

        components
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid_data = GridData::from_str(input);
    let connected_components = grid_data.get_connected_components();

    let result: usize = connected_components
        .iter()
        .map(|component| component.area() * component.perimeter())
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid_data = GridData::from_str(input);
    let connected_components = grid_data.get_connected_components();

    let result: usize = connected_components
        .iter()
        .map(|component| component.area() * component.side())
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
