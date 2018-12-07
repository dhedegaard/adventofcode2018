use std::collections::{HashMap, HashSet};

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

pub fn parse_input(input: &str) -> HashMap<char, Vec<char>> {
    let mut result = HashMap::new();
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<_>>();
        result.entry(chars[5]).or_insert(vec![]).push(chars[36]);
    }
    result
}

pub fn part1(input: &mut HashMap<char, Vec<char>>) -> String {
    // Determine all the steps.
    let mut steps = {
        let mut result = HashSet::new();
        for entry in input.iter() {
            result.insert(entry.0.to_owned());
            for val in entry.1 {
                result.insert(val.to_owned());
            }
        }
        result
    };

    let mut result = "".to_owned();

    while !steps.is_empty() {
        let candidate: char = {
            let mut result = vec![];
            for step in &steps {
                if input.values().any(|v| v.contains(step)) {
                    continue;
                }
                result.push(step.to_owned());
            }
            result.sort();
            result[0]
        };
        result.push(candidate);
        steps.remove(&candidate);
        input.remove(&candidate);
    }

    result.to_owned()
}


#[cfg(test)]
mod tests {
    use day07::{parse_input, part1, get_input};

    const TEST_INPUT: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&mut parse_input(TEST_INPUT)), "CABDFE");
    }

    #[test]
    fn part1_result() {
        assert_eq!(part1(&mut parse_input(&get_input())), "OCPUEFIXHRGWDZABTQJYMNKVSL")
    }
}