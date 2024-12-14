use ndarray::Array;
use regex::Regex;
use std::cmp::Ordering::{Greater, Less};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

advent_of_code::solution!(14);

#[derive(Hash, Eq, PartialEq)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Copy, Clone)]
struct Coord(i64, i64);

impl Coord {
    fn get_quadrant(&self) -> Option<Quadrant> {
        match ((self.0).cmp(&50), (self.1).cmp(&51)) {
            (Less, Less) => Some(Quadrant::TopLeft),
            (Greater, Less) => Some(Quadrant::TopRight),
            (Less, Greater) => Some(Quadrant::BottomLeft),
            (Greater, Greater) => Some(Quadrant::BottomRight),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Robot {
    init_pos: Coord,
    velocity: Coord,
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"p=(?P<x_init>-?\d+),(?P<y_init>-?\d+)\s+v=(?P<x_vel>-?\d+),(?P<y_vel>-?\d+)",
        )
        .map_err(|e| e.to_string())?;

        let caps = re
            .captures(s)
            .ok_or_else(|| format!("Input does not match the expected format: {}", s))?;

        let x_init: i64 = caps["x_init"]
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let y_init: i64 = caps["y_init"]
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let x_vel: i64 = caps["x_vel"]
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let y_vel: i64 = caps["y_vel"]
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;

        Ok(Robot {
            init_pos: Coord(x_init, y_init),
            velocity: Coord(x_vel, y_vel),
        })
    }
}

impl Robot {
    fn after_t_seconds(&self, t: usize) -> Coord {
        let init = self.init_pos;
        (1..=t).fold(init, |acc, _| {
            let velocity = &self.velocity;
            Coord(
                (acc.0 + velocity.0).rem_euclid(101),
                (acc.1 + velocity.1).rem_euclid(103),
            )
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let final_positions: Vec<Quadrant> = input
        .lines()
        .filter_map(|line| {
            let robot = Robot::from_str(line).expect("Robot description should be parseable");
            let final_position = robot.after_t_seconds(100);
            final_position.get_quadrant()
        })
        .collect();

    let mut counts: HashMap<Quadrant, usize> = HashMap::new();
    for quadrant in final_positions.into_iter() {
        *counts.entry(quadrant).or_insert(0) += 1;
    }

    let result = counts.values().product();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    /*
    https://www.reddit.com/r/adventofcode/comments/1he0asr/comment/m1zzfsh

    TLDR: You can find the best time by doing 103 iterations, plus everyone's favourite piece of mathematics, the Chinese Remainder Theorem.

    Due to the way the problem today has been set up, the evolution of the x and y coordinates is completely independent of each other, and repeats every 101 time steps for the x coordinate, and every 103 time steps for the y coordinate.
    So, if we can find the 'best' x and y offset, we can combine this information to find the best time, which is the answer to part 2.
    Assuming that the image will be something that causes the points to be clustered together, we can calculate the variances of the x and y coordinates - that's the image above. Looking at these variances, there is very obviously a 'best' x and y offset, bx and by. So, we know that

    t = bx (mod W)
    t = by (mod H)

    As t = bx (mod W), then t = bx + k*W. Substituting this into the second equation we get

    bx + k*W = by (mod H)
    k*W = by - bx (mod H)
    k = inverse(W)*(by - bx) (mod H)

    and so, finally,

    t = bx + inverse(W)*(by-bx)*W

    The difficult part of this is finding the inverse of W (modulo H). Luckily, Python's pow() function has modulo arithetic built into it, so we can just do pow(W, -1, H) to find the inverse of W modulo H.
    -> 51 * 101 = 1 (mod 103)
    */
    let robots: Vec<Robot> = input
        .lines()
        .map(|line| Robot::from_str(line).expect("Robot description should be parseable"))
        .collect();

    let (mut x_vars, y_vars): (Vec<_>, Vec<_>) = (1..=103)
        .map(|t| {
            let positions: Vec<(f64, f64)> = robots
                .iter()
                .map(|robot| {
                    let Coord(x_t, y_t) = robot.after_t_seconds(t);
                    (x_t as f64, y_t as f64)
                })
                .collect();

            let (x_positions, y_positions): (Vec<_>, Vec<_>) = positions.into_iter().unzip();
            (
                Array::from_vec(x_positions).var(1.0),
                Array::from_vec(y_positions).var(1.0),
            )
        })
        .unzip();

    x_vars.truncate(101);

    let x_time = (x_vars
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .expect("Minimum element expected")
        .0
        + 1)
        % 101;
    let y_time = (y_vars
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .expect("Minimum element expected")
        .0
        + 1)
        % 103;

    let best_time = x_time
        + (51 * (y_time as isize - x_time as isize).rem_euclid(103) as usize).rem_euclid(103) * 101;

    Some(best_time)
}
