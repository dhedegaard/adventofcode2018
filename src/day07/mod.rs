use std::collections::{HashMap, HashSet, BinaryHeap, BTreeSet};

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

pub fn parse_input_part2(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|l| l.as_bytes())
        .map(|l| (l[5] as char, l[36] as char))
        .collect()
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

pub fn part2(input: &[(char, char)], workercount: usize, time_offset: i32) -> i32 {
    let mut successors = HashMap::new();
    let mut predecessors = HashMap::new();
    for &(a, b) in input {
        successors.entry(a).or_insert_with(Vec::new).push(b);
        predecessors.entry(b).or_insert_with(HashSet::new).insert(a);
    }
    for &s in successors.keys() {
        predecessors.entry(s).or_insert_with(HashSet::new);
    }
    // End, task
    let mut workers = BinaryHeap::new();
    // Tasks ready to execute
    let mut ready = predecessors
        .iter()
        .filter_map(|(&a, b)| if b.is_empty() { Some(a) } else { None })
        .collect::<BTreeSet<_>>();
    for r in ready.iter() {
        predecessors.remove(r);
    }
    let mut time = 0;
    let mut completed = String::new();
    while !(ready.is_empty() && workers.is_empty()) {
        // Assign jobs to free workers
        while workers.len() < workercount && !ready.is_empty() {
            let job = *ready.iter().next().unwrap();
            ready.remove(&job);
            let completion = time - time_offset - (job as i32 - 'A' as i32 + 1);
            workers.push((completion, job));
        }
        // Move to next completion time
        let (t, j) = workers.pop().unwrap();
        time = t;
        completed.push(j);
        // Check if some jobs are now ready to execute
        if let Some(succ) = successors.get(&j) {
            for &job in succ {
                let pred = predecessors.get_mut(&job).unwrap();
                pred.remove(&j);
                if pred.is_empty() {
                    ready.insert(job);
                }
            }
        }
    }
    -time
}


#[cfg(test)]
mod tests {
    use day07::{parse_input, part1, get_input, part2, parse_input_part2};

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

    #[test]
    fn part2_examples() {
        assert_eq!(part2(&mut parse_input_part2(TEST_INPUT),2 ,0), 15);
    }

    #[test]
    fn part2_result() {
        assert_eq!(part2(&mut parse_input_part2(&get_input()),5 ,60), 991);
    }

}