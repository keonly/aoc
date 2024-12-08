use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coord(usize, usize);

#[derive(Debug)]
struct GridData {
    num_rows: usize,
    num_cols: usize,
    antennae: HashMap<char, Vec<Coord>>,
}

impl GridData {
    fn from_str(input: &str) -> Self {
        let mut antennae: HashMap<char, Vec<Coord>> = HashMap::new();

        let lines: Vec<&str> = input.lines().collect();

        let num_rows = lines.len();
        let num_cols = lines[0].len();

        for (row_idx, line) in lines.iter().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                match c {
                    '.' => {}
                    _ => {
                        antennae.entry(c).or_default().push(Coord(row_idx, col_idx));
                    }
                }
            }
        }

        GridData {
            num_rows,
            num_cols,
            antennae,
        }
    }

    fn in_board(&self, coord: &Coord) -> bool {
        coord.0 < self.num_rows && coord.1 < self.num_cols
    }

    fn get_k_antinodes(&self, k: usize, coords: &[&Coord]) -> Option<Vec<Coord>> {
        assert!(
            coords.len() == 2,
            "You must pass a Vec with 2 Coords! Got {}",
            coords.len()
        );
        assert!(k > 0, "k should be positive! Got {}", k);

        let mut result: Vec<Coord> = vec![];

        let mut check_and_push = |coord0, coord1| {
            let Coord(x0, y0) = coord0;
            let Coord(x1, y1) = coord1;

            if let (Some(new_x), Some(new_y)) = (
                x0.checked_mul(k)?
                    .checked_sub(x1.checked_mul(k - 1).expect("Multiplication out of range")),
                y0.checked_mul(k)?
                    .checked_sub(y1.checked_mul(k - 1).expect("Multiplication out of range")),
            ) {
                let antinode = Coord(new_x, new_y);
                if self.in_board(&antinode) {
                    result.push(Coord(new_x, new_y))
                }
            }

            None::<bool>
        };

        check_and_push(*coords[0], *coords[1]);
        check_and_push(*coords[1], *coords[0]);

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid_data = GridData::from_str(input);
    let mut antinodes: HashSet<Coord> = HashSet::new();

    for coords in grid_data.antennae.values() {
        let combinations: Vec<Vec<&Coord>> = coords.iter().combinations(2).collect();
        for coords in combinations.iter() {
            if let Some(antinode_coords) = grid_data.get_k_antinodes(2, coords) {
                antinode_coords.iter().for_each(|&antinode| {
                    antinodes.insert(antinode);
                });
            }
        }
    }

    let result = antinodes.len();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid_data = GridData::from_str(input);
    let mut antinodes: HashSet<Coord> = HashSet::new();

    for coords in grid_data.antennae.values() {
        let combinations: Vec<Vec<&Coord>> = coords.iter().combinations(2).collect();
        for coords in combinations.iter() {
            let mut k: usize = 1;
            while let Some(antinode_coords) = grid_data.get_k_antinodes(k, coords) {
                antinode_coords.iter().for_each(|&antinode| {
                    antinodes.insert(antinode);
                });
                k += 1;
            }
        }
    }

    let result = antinodes.len();

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
