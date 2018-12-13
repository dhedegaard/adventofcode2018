use std::collections::HashMap;

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

pub fn parse_input(input: &str) -> (String, HashMap<String, char>) {
    let lines = input.lines().collect::<Vec<_>>();
    let state = lines[0].split(": ").nth(1).unwrap().trim().to_owned();
    let map = lines[2..]
        .iter()
        .map(|&line| {
            (
                line[..5].chars().collect::<String>(),
                if line.ends_with('#') { '#' } else { '.' },
            )
        })
        .collect();
    (state, map)
}

pub fn solve(
    initial_state: String,
    instructions: &HashMap<String, char>,
    generations: usize,
) -> isize {
    let mut n = String::from("...");
    n.push_str(&initial_state);
    n.push_str("...");

    let mut last = 0;
    let mut diffs: HashMap<isize, u32> = HashMap::new();

    for gen in 1..=generations {
        // Start with 3 empty pots at the beginning and the end, each time.
        let mut s = String::from("...");
        for i in 2..n.len() - 2 {
            let slice = &n[i - 2..=i + 2];
            match instructions.get(slice) {
                Some('#') => {
                    s.push('#');
                }
                _ => s.push('.'),
            }
        }
        s.push_str("...");
        n = s;

        // Make sure to grow the string at both ends.
        let score = n
            .chars()
            .enumerate()
            .filter(|(_, c)| c == &'#')
            .map(|(i, _)| i as isize - (3 + gen as isize))
            .sum::<isize>();
        let e = diffs.entry(score as isize - last as isize).or_insert(0);
        if *e > 10 {
            return (generations - gen) as isize * (score - last) + score;
        } else {
            *e += 1;
        }
        last = score;
    }
    last
}

pub fn part1(initial_state: String, instructions: &HashMap<String, char>) -> isize {
    solve(initial_state, instructions, 20)
}

pub fn part2(initial_state: String, instructions: &HashMap<String, char>) -> isize {
    solve(initial_state, instructions, 50_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1_examples() {
        let (state, instructions) = parse_input(TEST_INPUT);
        assert_eq!(part1(state, &instructions), 325);
    }

    #[test]
    fn part1_result() {
        let (state, instructions) = parse_input(&get_input());
        assert_eq!(part1(state, &instructions), 2349);
    }

    #[test]
    fn part2_examples() {
        let (state, instructions) = parse_input(TEST_INPUT);
        assert_eq!(part2(state, &instructions), 999999999374);
    }

    #[test]
    fn part2_result() {
        let (state, instructions) = parse_input(&get_input());
        assert_eq!(part2(state, &instructions), 2100000001168);
    }
}
