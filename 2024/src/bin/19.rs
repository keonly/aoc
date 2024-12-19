use std::collections::{HashMap, HashSet};
use std::str::FromStr;

advent_of_code::solution!(19);

#[derive(Debug)]
struct Towels {
    set: HashSet<String>,
}

impl FromStr for Towels {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let set: HashSet<String> = s.split(", ").map(String::from).collect();

        Ok(Self { set })
    }
}

impl Towels {
    fn count_combinations(&self, design: &str) -> usize {
        let mut cache = HashMap::new();
        self.count_combinations_with_cache(design, &mut cache)
    }

    fn count_combinations_with_cache<'a>(
        &self,
        design: &'a str,
        cache: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if let Some(&result) = cache.get(design) {
            return result;
        }

        let result = if design.is_empty() {
            1
        } else {
            self.set
                .iter()
                .filter_map(|towel| design.strip_prefix(towel))
                .map(|rest| self.count_combinations_with_cache(rest, cache))
                .sum()
        };

        cache.insert(design, result);
        result
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    if let Some((towels_str, designs_str)) = input.split_once("\n\n") {
        let towels = Towels::from_str(towels_str).expect("Towel info should be parseable");
        let result = designs_str
            .lines()
            .map(|design| towels.count_combinations(design))
            .filter(|x| *x > 0)
            .count();

        Some(result)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    if let Some((towels_str, designs_str)) = input.split_once("\n\n") {
        let towels = Towels::from_str(towels_str).expect("Towel info should be parseable");
        let result = designs_str
            .lines()
            .map(|design| towels.count_combinations(design))
            .sum();

        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
