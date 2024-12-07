use std::collections::VecDeque;

advent_of_code::solution!(7);

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn apply(&self, x: u64, y: u64) -> Option<u64> {
        match self {
            Operator::Add => x.checked_add(y),
            Operator::Mul => x.checked_mul(y),
            Operator::Concat => format!("{}{}", x, y).parse::<u64>().ok(),
        }
    }
}

#[derive(Debug)]
struct Equation {
    goal: u64,
    nums: Vec<u64>,
}

impl Equation {
    fn from_str(line: &str) -> Self {
        let mut nums_iter = line
            .split(|c: char| c.is_ascii_punctuation() || c.is_ascii_whitespace())
            .flat_map(|num| num.parse::<u64>());
        let goal: u64 = nums_iter
            .next()
            .expect("At least two numbers should be present");
        let nums: Vec<u64> = nums_iter.collect();

        Equation { goal, nums }
    }

    fn is_valid(&self) -> bool {
        let num_nums: usize = self.nums.len();
        let operations: Vec<Operator> = vec![Operator::Add, Operator::Mul];

        let mut progress: VecDeque<(usize, u64)> = VecDeque::new();
        progress.push_back((0, self.nums[0]));

        while let Some((curr_step, curr_val)) = progress.pop_front() {
            let next_step = curr_step + 1;
            if next_step == num_nums {
                if curr_val == self.goal {
                    return true;
                } else {
                    continue;
                }
            }

            let next_num = self.nums[next_step];
            for op in &operations {
                let next_val_option = op.apply(curr_val, next_num);
                if let Some(next_val) = next_val_option {
                    if next_val <= self.goal {
                        progress.push_back((next_step, next_val));
                    }
                }
            }
        }

        false
    }

    fn is_valid_with_concat(&self) -> bool {
        let num_nums: usize = self.nums.len();
        let operations: Vec<Operator> = vec![Operator::Add, Operator::Mul, Operator::Concat];

        let mut progress: VecDeque<(usize, u64)> = VecDeque::new();
        progress.push_back((0, self.nums[0]));

        while let Some((curr_step, curr_val)) = progress.pop_front() {
            let next_step = curr_step + 1;
            if next_step == num_nums {
                if curr_val == self.goal {
                    return true;
                } else {
                    continue;
                }
            }

            let next_num = self.nums[next_step];
            for op in &operations {
                let next_val_option = op.apply(curr_val, next_num);
                if let Some(next_val) = next_val_option {
                    if next_val <= self.goal {
                        progress.push_back((next_step, next_val));
                    }
                }
            }
        }

        false
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let eqs: Vec<Equation> = input.lines().map(Equation::from_str).collect();

    let result: u64 = eqs
        .iter()
        .filter_map(|eq| if eq.is_valid() { Some(eq.goal) } else { None })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let eqs: Vec<Equation> = input.lines().map(Equation::from_str).collect();

    let result: u64 = eqs
        .iter()
        .filter_map(|eq| {
            if eq.is_valid_with_concat() {
                Some(eq.goal)
            } else {
                None
            }
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
