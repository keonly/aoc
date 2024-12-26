use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy)]
enum Operator {
    And,
    Or,
    Xor,
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(format!("Unknown operator: {}", s)),
        }
    }
}

impl Operator {
    fn apply(&self, op1: u8, op2: u8) -> u8 {
        match self {
            Operator::And => op1 & op2,
            Operator::Or => op1 | op2,
            Operator::Xor => op1 ^ op2,
        }
    }
}

#[derive(Debug)]
struct Gate {
    op1: String,
    op2: String,
    operator: Operator,
    out: String,
}

impl FromStr for Gate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?P<op1>\w*)\s(?P<operator>\w+)\s(?P<op2>\w*)\s->\s(?P<out>\w*)")
            .map_err(|e| e.to_string())?;

        let caps = re
            .captures(s)
            .ok_or_else(|| format!("Input does not match the expected format: {}", s))?;

        let op1: String = caps["op1"].to_string();
        let operator: Operator = Operator::from_str(&caps["operator"])?;
        let op2: String = caps["op2"].to_string();
        let out: String = caps["out"].to_string();

        Ok(Self {
            op1,
            op2,
            operator,
            out,
        })
    }
}

#[derive(Debug)]
struct Circuit {
    values: HashMap<String, u8>,
    gates: HashMap<String, Gate>,
}

impl Circuit {
    fn new() -> Self {
        let values: HashMap<String, u8> = HashMap::new();
        let gates: HashMap<String, Gate> = HashMap::new();

        Self { values, gates }
    }

    fn set_value(&mut self, s: &str) {
        let re = Regex::new(r"(?P<name>\w*):\s(?P<val>\w*)").expect("Regex error");

        let caps = re
            .captures(s)
            .expect("Input does not match the expected format");

        let name: String = caps["name"].to_string();
        let val: u8 = caps["val"].parse().expect("Value should be parseable");

        self.values.insert(name, val);
    }

    fn add_gate(&mut self, s: &str) {
        let gate = Gate::from_str(s).expect("Gate parse failed");
        let out = gate.out.clone();

        self.gates.insert(out, gate);
    }

    fn dfs(&mut self, name: String) -> u8 {
        if self.values.contains_key(&name) {
            return self.values[&name];
        }

        let gate = self.gates.get(&name).expect("Unknown gate");
        let op1_s = gate.op1.clone();
        let op2_s = gate.op2.clone();
        let operator = gate.operator;

        let op1 = self.dfs(op1_s);
        let op2 = self.dfs(op2_s);
        let result = operator.apply(op1, op2);

        self.values.insert(name, result);
        result
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut description = input.split("\n\n");
    let [vals, gates] = [(); 2].map(|_| description.next().unwrap());

    let mut circuit = Circuit::new();

    for line in vals.lines() {
        circuit.set_value(line);
    }

    for line in gates.lines() {
        circuit.add_gate(line);
    }

    let mut z_bits: Vec<String> = circuit
        .gates
        .keys()
        .filter_map(|x| {
            if x.starts_with('z') {
                Some(x.to_owned())
            } else {
                None
            }
        })
        .collect();
    z_bits.sort();

    let result = z_bits.iter().enumerate().fold(0, |acc, (idx, name)| {
        let bit = circuit.dfs(name.to_string());
        acc | (bit as u64) << idx
    });

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    // Solved manually
    // https://en.wikipedia.org/wiki/Adder_(electronics)#Ripple-carry_adder
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
