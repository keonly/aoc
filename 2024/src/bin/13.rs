use num_bigint::{BigInt, Sign};
use num_traits::Zero;
use regex::Regex;
use std::num::ParseIntError;

advent_of_code::solution!(13);

struct ClawMachine {
    button_a: [u128; 2],
    button_b: [u128; 2],
    prize: [u128; 2],
}

impl ClawMachine {
    fn from_str(input: &str) -> Result<Self, String> {
        let button_a_re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").map_err(|e| e.to_string())?;
        let button_b_re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").map_err(|e| e.to_string())?;
        let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").map_err(|e| e.to_string())?;

        let button_a_caps = button_a_re
            .captures(input)
            .ok_or("Failed to parse Button A values")?;
        let button_a = [
            button_a_caps[1]
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?,
            button_a_caps[2]
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?,
        ];

        let button_b_caps = button_b_re
            .captures(input)
            .ok_or("Failed to parse Button B values")?;
        let button_b = [
            button_b_caps[1]
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?,
            button_b_caps[2]
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?,
        ];

        let prize_caps = prize_re
            .captures(input)
            .ok_or("Failed to parse Prize values")?;
        let prize = [
            prize_caps[1]
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?,
            prize_caps[2]
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?,
        ];

        Ok(ClawMachine {
            button_a,
            button_b,
            prize,
        })
    }

    fn solve(&self) -> Option<(u64, u64)> {
        let [xa, ya] = self.button_a;
        let [xb, yb] = self.button_b;
        let [px, py] = self.prize;

        let xa = BigInt::from(xa);
        let ya = BigInt::from(ya);
        let xb = BigInt::from(xb);
        let yb = BigInt::from(yb);
        let px = BigInt::from(px);
        let py = BigInt::from(py);

        let zero = BigInt::zero();

        let determinant = &xa * &yb - &ya * &xb;
        if determinant.is_zero() {
            return None;
        }

        let a_numerator = &px * &yb - &py * &xb;
        let b_numerator = &py * &xa - &px * &ya;

        if &a_numerator % &determinant != zero || &b_numerator % &determinant != zero {
            return None;
        }

        let a = a_numerator / &determinant;
        let b = b_numerator / &determinant;

        if a.sign() != Sign::Plus || b.sign() != Sign::Plus {
            return None;
        }

        let (a_sign, a_digits) = a.to_u64_digits();
        let (b_sign, b_digits) = b.to_u64_digits();

        if a_sign == Sign::Plus
            && a_digits.len() == 1
            && b_sign == Sign::Plus
            && b_digits.len() == 1
        {
            Some((a_digits[0], b_digits[0]))
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let claw_machines: Vec<ClawMachine> = input
        .split("\n\n")
        .map(|desc| {
            ClawMachine::from_str(desc).expect("Claw machine description should be parseable")
        })
        .collect();

    let result = claw_machines
        .iter()
        .filter_map(|claw_machine| claw_machine.solve())
        .map(|solution| 3 * solution.0 + solution.1)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let claw_machines: Vec<ClawMachine> = input
        .split("\n\n")
        .map(|desc| {
            let mut claw_machine =
                ClawMachine::from_str(desc).expect("Claw machine description should be parseable");
            let [px, py] = claw_machine.prize;
            claw_machine.prize = [px + 10000000000000, py + 10000000000000];
            claw_machine
        })
        .collect();

    let result = claw_machines
        .iter()
        .filter_map(|claw_machine| claw_machine.solve())
        .map(|solution| 3 * solution.0 + solution.1)
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
