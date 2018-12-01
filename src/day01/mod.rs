use std::collections::HashSet;

pub fn raw_input() -> String {
    include_str!("input.txt").to_string()
}

pub fn parse_input(input: &str) -> Vec<i32> {
    input.split_whitespace().map(|e|
        e.replace(",", "").replace("+", "").parse::<i32>().unwrap()
    ).collect()
}

pub fn part1(input: Vec<i32>) -> i32 {
    let mut result = 0;
    for elem in input {
        result += elem;
    }
    result
}

pub fn part2(input: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut seen_numbers = HashSet::new();
    seen_numbers.insert(0);
    loop {
        for elem in &input {
            result += elem;
            if seen_numbers.contains(&result) {
                return result.to_owned();
            }
            seen_numbers.insert(result.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use day01::{raw_input, parse_input, part1, part2};

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input("+1, +1, +1"), vec![1, 1, 1]);
        assert_eq!(parse_input("+1, +1, -2"), vec![1, 1, -2]);
        assert_eq!(parse_input("-1, -2, -3"), vec![-1, -2, -3]);
    }

    #[test]
    fn examples_part1() {
        assert_eq!(part1(parse_input("+1, -2, +3, +1")), 3);
        assert_eq!(part1(parse_input("+1, +1, +1")), 3);
        assert_eq!(part1(parse_input("+1, +1, -2")), 0);
        assert_eq!(part1(parse_input("-1, -2, -3")), -6);
    }

    #[test]
    fn result_part1() {
        assert_eq!(part1(parse_input(&raw_input())), 516);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(part2(parse_input("+1, -2, +3, +1")), 2);
        assert_eq!(part2(parse_input("+1, -1")), 0);
        assert_eq!(part2(parse_input("+3, +3, +4, -2, -4")), 10);
        assert_eq!(part2(parse_input("-6, +3, +8, +5, -6")), 5);
        assert_eq!(part2(parse_input("+7, +7, -2, -7, -4")), 14);
    }

    #[test]
    fn result_part2() {
        assert_eq!(part2(parse_input(&raw_input())), 71892);
    }
}
