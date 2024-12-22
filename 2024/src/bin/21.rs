#![feature(iter_chain)]

use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::once;

advent_of_code::solution!(21);

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum NumKey {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    A,
}

impl NumKey {
    fn from_char(c: &char) -> Self {
        match c {
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            '0' => Self::Zero,
            'A' => Self::A,
            _ => panic!("Unknown key: {}", c),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum DirKey {
    Left,
    Right,
    Up,
    Down,
    A,
}

impl DirKey {
    fn from_direction(dir: &Direction) -> Self {
        match dir {
            Direction::Up => DirKey::Up,
            Direction::Right => DirKey::Right,
            Direction::Down => DirKey::Down,
            Direction::Left => DirKey::Left,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

trait Adjacency {
    fn adjacencies() -> HashMap<Self, Vec<(Self, Direction)>>
    where
        Self: Sized;
}

impl Adjacency for NumKey {
    // Numpad
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+

    fn adjacencies() -> HashMap<Self, Vec<(Self, Direction)>>
    where
        Self: Sized,
    {
        HashMap::from([
            (
                NumKey::Zero,
                vec![(NumKey::Two, Direction::Up), (NumKey::A, Direction::Right)],
            ),
            (
                NumKey::One,
                vec![
                    (NumKey::Four, Direction::Up),
                    (NumKey::Two, Direction::Right),
                ],
            ),
            (
                NumKey::Two,
                vec![
                    (NumKey::Five, Direction::Up),
                    (NumKey::Three, Direction::Right),
                    (NumKey::Zero, Direction::Down),
                    (NumKey::One, Direction::Left),
                ],
            ),
            (
                NumKey::Three,
                vec![
                    (NumKey::Six, Direction::Up),
                    (NumKey::A, Direction::Down),
                    (NumKey::Two, Direction::Left),
                ],
            ),
            (
                NumKey::Four,
                vec![
                    (NumKey::Seven, Direction::Up),
                    (NumKey::Five, Direction::Right),
                    (NumKey::One, Direction::Down),
                ],
            ),
            (
                NumKey::Five,
                vec![
                    (NumKey::Eight, Direction::Up),
                    (NumKey::Six, Direction::Right),
                    (NumKey::Two, Direction::Down),
                    (NumKey::Four, Direction::Left),
                ],
            ),
            (
                NumKey::Six,
                vec![
                    (NumKey::Nine, Direction::Up),
                    (NumKey::Three, Direction::Down),
                    (NumKey::Five, Direction::Left),
                ],
            ),
            (
                NumKey::Seven,
                vec![
                    (NumKey::Eight, Direction::Right),
                    (NumKey::Four, Direction::Down),
                ],
            ),
            (
                NumKey::Eight,
                vec![
                    (NumKey::Nine, Direction::Right),
                    (NumKey::Five, Direction::Down),
                    (NumKey::Seven, Direction::Left),
                ],
            ),
            (
                NumKey::Nine,
                vec![
                    (NumKey::Six, Direction::Down),
                    (NumKey::Eight, Direction::Left),
                ],
            ),
            (
                NumKey::A,
                vec![
                    (NumKey::Three, Direction::Up),
                    (NumKey::Zero, Direction::Left),
                ],
            ),
        ])
    }
}

impl Adjacency for DirKey {
    //  Dirpad
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+

    fn adjacencies() -> HashMap<Self, Vec<(Self, Direction)>>
    where
        Self: Sized,
    {
        HashMap::from([
            (DirKey::Left, vec![(DirKey::Down, Direction::Right)]),
            (
                DirKey::Down,
                vec![
                    (DirKey::Up, Direction::Up),
                    (DirKey::Right, Direction::Right),
                    (DirKey::Left, Direction::Left),
                ],
            ),
            (
                DirKey::Right,
                vec![(DirKey::A, Direction::Up), (DirKey::Down, Direction::Left)],
            ),
            (
                DirKey::Up,
                vec![
                    (DirKey::A, Direction::Right),
                    (DirKey::Down, Direction::Down),
                ],
            ),
            (
                DirKey::A,
                vec![
                    (DirKey::Right, Direction::Down),
                    (DirKey::Up, Direction::Left),
                ],
            ),
        ])
    }
}

type Step = usize;

// Unfortunaately #[cached] does not support neither methods nor generic functions,
// so I had to come up with this super redundant functions.

#[cached]
fn num_shortest_path(from: NumKey, to: NumKey) -> Vec<Vec<Direction>> {
    let adjacencies = NumKey::adjacencies();
    let mut visiting: VecDeque<(NumKey, Vec<Direction>)> = VecDeque::new();
    let mut dists: HashMap<NumKey, Step> = HashMap::new();
    let mut paths: Vec<Vec<Direction>> = vec![];
    let mut min_steps: Option<Step> = None;

    visiting.push_back((from, vec![]));
    dists.insert(from, 0);

    while let Some((curr_key, prevs)) = visiting.pop_front() {
        if let Some(step) = min_steps {
            if prevs.len() > step {
                continue;
            }
        }

        if curr_key == to {
            if let Some(step) = min_steps {
                if prevs.len() == step {
                    paths.push(prevs.clone());
                }
            } else {
                min_steps = Some(prevs.len());
                paths.push(prevs.clone());
            }
        } else {
            for (next_key, direction) in adjacencies[&curr_key].iter() {
                let next_steps = prevs.len() + 1;

                if dists
                    .get(next_key)
                    .is_none_or(|&known_steps| next_steps <= known_steps)
                {
                    dists.insert(*next_key, next_steps);

                    let mut next_prevs = prevs.clone();
                    next_prevs.push(*direction);
                    visiting.push_back((*next_key, next_prevs));
                }
            }
        }
    }

    paths
}

#[cached]
fn dir_shortest_path(from: DirKey, to: DirKey) -> Vec<Vec<Direction>> {
    let adjacencies = DirKey::adjacencies();
    let mut visiting: VecDeque<(DirKey, Vec<Direction>)> = VecDeque::new();
    let mut dists: HashMap<DirKey, Step> = HashMap::new();
    let mut paths: Vec<Vec<Direction>> = vec![];
    let mut min_steps: Option<Step> = None;

    visiting.push_back((from, vec![]));
    dists.insert(from, 0);

    while let Some((curr_key, prevs)) = visiting.pop_front() {
        if let Some(step) = min_steps {
            if prevs.len() > step {
                continue;
            }
        }

        if curr_key == to {
            if let Some(step) = min_steps {
                if prevs.len() == step {
                    paths.push(prevs.clone());
                }
            } else {
                min_steps = Some(prevs.len());
                paths.push(prevs.clone());
            }
        } else {
            for (next_key, direction) in adjacencies[&curr_key].iter() {
                let next_steps = prevs.len() + 1;

                if dists
                    .get(next_key)
                    .is_none_or(|&known_steps| next_steps <= known_steps)
                {
                    dists.insert(*next_key, next_steps);

                    let mut next_prevs = prevs.clone();
                    next_prevs.push(*direction);
                    visiting.push_back((*next_key, next_prevs));
                }
            }
        }
    }

    paths
}

#[cached]
fn dir_shortest_len(from: DirKey, to: DirKey) -> usize {
    let adjacencies = DirKey::adjacencies();
    let mut visiting: VecDeque<(DirKey, Step)> = VecDeque::new();
    let mut visited: HashSet<DirKey> = HashSet::new();

    visiting.push_back((from, 0));

    while let Some((curr_key, curr_step)) = visiting.pop_front() {
        if curr_key == to {
            return curr_step;
        } else {
            for (next_key, _) in adjacencies[&curr_key].iter() {
                if visited.insert(*next_key) {
                    visiting.push_back((*next_key, curr_step + 1));
                }
            }
        }
    }

    unreachable!()
}

#[cached]
fn num_dfs(sequence: Vec<NumKey>, depth: usize) -> usize {
    let mut proper_sequence = vec![NumKey::A];
    proper_sequence.extend(sequence);

    proper_sequence
        .iter()
        .tuple_windows()
        .map(|(from, to)| {
            num_shortest_path(*from, *to)
                .iter()
                .map(|path| {
                    let keys = path
                        .iter()
                        .map(DirKey::from_direction)
                        .chain(once(DirKey::A))
                        .collect::<Vec<_>>();
                    dir_dfs(keys, depth)
                })
                .min()
                .unwrap()
        })
        .sum()
}

#[cached]
fn dir_dfs(sequence: Vec<DirKey>, depth: usize) -> usize {
    let mut proper_sequence = vec![DirKey::A];
    proper_sequence.extend(sequence);

    proper_sequence
        .iter()
        .tuple_windows()
        .map(|(from, to)| {
            if depth == 1 {
                dir_shortest_len(*from, *to) + 1
            } else {
                dir_shortest_path(*from, *to)
                    .iter()
                    .map(|path| {
                        let keys = path
                            .iter()
                            .map(DirKey::from_direction)
                            .chain(once(DirKey::A))
                            .collect::<Vec<_>>();
                        dir_dfs(keys, depth - 1)
                    })
                    .min()
                    .unwrap()
            }
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let result = input
        .lines()
        .map(|line| {
            let actual_val = line.trim_end_matches('A').parse::<usize>().unwrap();
            let sequence = line
                .chars()
                .map(|c| NumKey::from_char(&c))
                .collect::<Vec<_>>();
            let min_keys = num_dfs(sequence, 2);

            actual_val * min_keys
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let result = input
        .lines()
        .map(|line| {
            let actual_val = line.trim_end_matches('A').parse::<usize>().unwrap();
            let sequence = line
                .chars()
                .map(|c| NumKey::from_char(&c))
                .collect::<Vec<_>>();
            let min_keys = num_dfs(sequence, 25);

            actual_val * min_keys
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
