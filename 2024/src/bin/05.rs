use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(5);

fn create_ordering_rule_map(dependencies: &str) -> HashMap<Option<u32>, HashSet<u32>> {
    let mut ordering_rules: HashMap<Option<u32>, HashSet<u32>> = HashMap::new();
    let mut seen_as_postrequesites: HashSet<u32> = HashSet::new();

    ordering_rules.insert(None, HashSet::new());

    for line in dependencies.lines() {
        let [fst, snd]: [u32; 2] = line
            .split('|')
            .map(|s| s.parse::<u32>().expect("String must be parseable as u32"))
            .take(2)
            .collect::<Vec<u32>>()
            .try_into()
            .expect("Dependency description must contain 2 numbers");

        if let Some(postrequisites) = ordering_rules.get_mut(&Some(fst)) {
            postrequisites.insert(snd);
        } else {
            ordering_rules.insert(Some(fst), HashSet::from([snd]));
        }

        if seen_as_postrequesites.contains(&fst) {
            ordering_rules.get_mut(&None).unwrap().remove(&fst);
        } else {
            ordering_rules.get_mut(&None).unwrap().insert(fst);
        }

        seen_as_postrequesites.insert(snd);
    }

    ordering_rules
}

fn parse_pages_to_produce(pages_to_produce_str: &str) -> Vec<Vec<u32>> {
    pages_to_produce_str
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u32>().expect("Pages must be parseable as u32"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn is_in_right_order(pages: &[u32], ordering_rules: &HashMap<Option<u32>, HashSet<u32>>) -> bool {
    let mut seen: HashSet<u32> = HashSet::new();

    pages.iter().fold(true, |acc, &val| {
        if !acc {
            false
        } else {
            let result = if let Some(postrequisites) = ordering_rules.get(&Some(val)) {
                seen.iter().all(|x| !postrequisites.contains(x))
            } else {
                true
            };
            seen.insert(val);
            result
        }
    })
}

fn reorder(pages: &[u32], ordering_rules: &HashMap<Option<u32>, HashSet<u32>>) -> Vec<u32> {
    let mut constrained_ordering_rules: HashMap<Option<u32>, HashSet<u32>> = HashMap::new();
    let mut in_degree_map: HashMap<u32, usize> = pages.iter().map(|&page| (page, 0)).collect();

    for &page in pages.iter() {
        if let Some(postrequisites) = ordering_rules.get(&Some(page)) {
            let filtered_postrequisites = postrequisites.iter().filter(|&&x| pages.contains(&x));
            let entry = constrained_ordering_rules.entry(Some(page)).or_default();

            for &post in filtered_postrequisites {
                entry.insert(post);
                *in_degree_map.get_mut(&post).unwrap() += 1;
            }
        } else {
            constrained_ordering_rules.entry(Some(page)).or_default();
        }
    }

    let mut queue: VecDeque<u32> = pages
        .iter()
        .filter(|&&page| in_degree_map[&page] == 0)
        .copied()
        .collect();
    let mut result: Vec<u32> = vec![];

    while let Some(curr) = queue.pop_front() {
        result.push(curr);

        if let Some(postrequisites) = constrained_ordering_rules.get(&Some(curr)) {
            for &post in postrequisites {
                let in_degree = in_degree_map.get_mut(&post).unwrap();
                *in_degree -= 1;
                if *in_degree == 0 {
                    queue.push_back(post);
                }
            }
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let [ordering_rules_str, pages_to_produce_str]: [&str; 2] = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .try_into()
        .expect("Input must consist of two parts separated by an empty line");

    let ordering_rules = create_ordering_rule_map(ordering_rules_str);
    let pages_to_produce = parse_pages_to_produce(pages_to_produce_str);

    let result: u32 = pages_to_produce
        .iter()
        .filter_map(|v| {
            let mid: usize = (v.len() - 1) / 2;
            if is_in_right_order(v, &ordering_rules) {
                Some(v[mid])
            } else {
                None
            }
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let [ordering_rules_str, pages_to_produce_str]: [&str; 2] = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .try_into()
        .expect("Input must consist of two parts separated by an empty line");

    let ordering_rules = create_ordering_rule_map(ordering_rules_str);
    let pages_to_produce = parse_pages_to_produce(pages_to_produce_str);

    let result: u32 = pages_to_produce
        .iter()
        .filter(|v| !is_in_right_order(v, &ordering_rules))
        .map(|v| {
            let mid: usize = (v.len() - 1) / 2;

            let reordered = reorder(v, &ordering_rules);
            reordered[mid]
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
