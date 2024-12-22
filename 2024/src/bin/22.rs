use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

fn evolve_secret_number(curr_secret: u64) -> u64 {
    let interm1 = ((curr_secret << 6) ^ curr_secret) & 0xffffffu64;
    let interm2 = ((interm1 >> 5) ^ interm1) & 0xffffffu64;

    ((interm2 << 11) ^ interm2) & 0xffffffu64
}

fn get_secret_number_list(initial: u64) -> Vec<u64> {
    let mut secret_numbers: Vec<u64> = vec![initial];

    for i in 0..2000 {
        secret_numbers.push(evolve_secret_number(secret_numbers[i]));
    }

    secret_numbers
}

fn get_sequence_to_price_map(secret_numbers: &[u64]) -> HashMap<(i8, i8, i8, i8), u8> {
    let costs: Vec<u8> = secret_numbers.iter().map(|x| (x % 10) as u8).collect();
    let diffs: Vec<i8> = costs.windows(2).map(|w| w[1] as i8 - w[0] as i8).collect();
    let diffs_and_costs = diffs
        .into_iter()
        .zip(costs.into_iter().skip(1))
        .collect::<Vec<_>>();

    let mut seq_to_price: HashMap<(i8, i8, i8, i8), u8> = HashMap::new();

    diffs_and_costs.windows(4).for_each(|window| {
        if let [(s1, _), (s2, _), (s3, _), (s4, price)] = window {
            seq_to_price.entry((*s1, *s2, *s3, *s4)).or_insert(*price);
        }
    });

    seq_to_price
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|line| {
            let mut secret: u64 = line.parse().expect("Secret number should be parseable");
            for _ in 0..2000 {
                secret = evolve_secret_number(secret);
            }

            secret
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let seq_to_prices: Vec<HashMap<_, _>> = input
        .lines()
        .map(|line| {
            let secret: u64 = line.parse().expect("Secret number should be parseable");
            let secret_numbers = get_secret_number_list(secret);

            get_sequence_to_price_map(&secret_numbers)
        })
        .collect();

    let key_union: HashSet<(i8, i8, i8, i8)> = seq_to_prices
        .iter()
        .flat_map(|map| map.keys().cloned())
        .collect();

    let result = key_union
        .iter()
        .map(|seq| {
            seq_to_prices
                .iter()
                .map(|map| *(map.get(seq).unwrap_or(&0)) as u64)
                .sum()
        })
        .max()
        .unwrap();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
