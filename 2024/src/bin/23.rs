use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

#[derive(Debug)]
struct Graph {
    adjacency: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Self {
        let adjacency: HashMap<String, HashSet<String>> = HashMap::new();

        Self { adjacency }
    }

    fn insert_edge(&mut self, v1: String, v2: String) {
        self.adjacency
            .entry(v1.clone())
            .and_modify(|set| {
                set.insert(v2.clone());
            })
            .or_insert(HashSet::from([v2.clone()]));

        self.adjacency
            .entry(v2.clone())
            .and_modify(|set| {
                set.insert(v1.clone());
            })
            .or_insert(HashSet::from([v1.clone()]));
    }

    fn is_connected(&self, v1: &String, v2: &String) -> bool {
        self.adjacency
            .get(v1)
            .is_some_and(|neighbours| neighbours.contains(v2))
    }

    fn count_chief_lan_parties(&self, target_vertices: &HashSet<String>) -> usize {
        let mut lan_parties: HashSet<(String, String, String)> = HashSet::new();

        for target in target_vertices {
            if let Some(neighbors) = self.adjacency.get(target) {
                for (v1, v2) in neighbors.iter().tuple_combinations() {
                    if self.is_connected(v1, v2) {
                        let mut triplet = [target.clone(), v1.clone(), v2.clone()];
                        triplet.sort();

                        lan_parties.insert((
                            triplet[0].clone(),
                            triplet[1].clone(),
                            triplet[2].clone(),
                        ));
                    }
                }
            }
        }

        lan_parties.len()
    }

    fn maximal_clique(&self) -> HashSet<String> {
        let mut cliques = Vec::new();
        let mut r = HashSet::new();
        let mut p: HashSet<String> = self.adjacency.keys().cloned().collect();
        let mut x = HashSet::new();

        self.bron_kerbosch(&mut r, &mut p, &mut x, &mut cliques);
        cliques
            .iter()
            .max_by_key(|clique| clique.len())
            .expect("At least one clique should exist")
            .clone()
    }

    fn bron_kerbosch(
        &self,
        r: &mut HashSet<String>,
        p: &mut HashSet<String>,
        x: &mut HashSet<String>,
        cliques: &mut Vec<HashSet<String>>,
    ) {
        if p.is_empty() && x.is_empty() {
            cliques.push(r.clone());
            return;
        }

        let p_clone = p.clone();
        for v in p_clone.iter() {
            r.insert((*v).to_string());

            let neighbours: HashSet<String> =
                self.adjacency.get(v).unwrap_or(&HashSet::new()).clone();

            let new_p: HashSet<String> = p.intersection(&neighbours).cloned().collect();
            let new_x: HashSet<String> = x.intersection(&neighbours).cloned().collect();

            self.bron_kerbosch(r, &mut new_p.clone(), &mut new_x.clone(), cliques);

            r.remove(v);
            p.remove(v);
            x.insert(v.clone());
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut starts_with_t: HashSet<String> = HashSet::new();
    let mut graph = Graph::new();

    for line in input.lines() {
        let mut splits = line.split('-');
        let [v1, v2] = [(); 2].map(|_| splits.next().unwrap().to_string());

        for v in [&v1, &v2] {
            if v.starts_with('t') {
                starts_with_t.insert(v.to_string());
            }
        }

        graph.insert_edge(v1, v2);
    }

    let lan_parties = graph.count_chief_lan_parties(&starts_with_t);
    Some(lan_parties)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut starts_with_t: HashSet<String> = HashSet::new();
    let mut graph = Graph::new();

    for line in input.lines() {
        let mut splits = line.split('-');
        let [v1, v2] = [(); 2].map(|_| splits.next().unwrap().to_string());

        for v in [&v1, &v2] {
            if v.starts_with('t') {
                starts_with_t.insert(v.to_string());
            }
        }

        graph.insert_edge(v1, v2);
    }

    let mut maximal_clique: Vec<_> = graph.maximal_clique().into_iter().collect();
    maximal_clique.sort();
    Some(maximal_clique.iter().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
