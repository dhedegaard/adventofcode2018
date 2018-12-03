extern crate regex;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Claim {
    number: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

pub fn parse_input(input: &str) -> Vec<Claim> {
    let input_regex = regex::Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = input_regex.captures(line).unwrap();
            Claim {
                number: caps[1].parse().unwrap(),
                x: caps[2].parse().unwrap(),
                y: caps[3].parse().unwrap(),
                width: caps[4].parse().unwrap(),
                height: caps[5].parse().unwrap(),
            }
        }).collect()
}

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

pub fn part1(input: &[Claim]) -> u32 {
    let mut map: HashMap<Point, u32> = HashMap::new();

    for claim in input {
        for dy in claim.y..claim.y + claim.height {
            for dx in claim.x..claim.x + claim.width {
                *map.entry(Point { x: dx, y: dy }).or_insert(0) += 1;
            }
        }
    }

    map.values().filter(|&e| *e >= 2).count() as u32
}

pub fn part2(input: &[Claim]) -> u32 {
    let mut map: HashMap<Point, Vec<u32>> = HashMap::new();
    let mut seen_numbers = HashSet::new();

    for claim in input {
        for dy in claim.y..claim.y + claim.height {
            for dx in claim.x..claim.x + claim.width {
                let p = Point { x: dx, y: dy };
                let mut vec = map.entry(p).or_insert(vec![]);
                vec.push(claim.number);
                if vec.len() > 1 {
                    for val in vec {
                        seen_numbers.insert(*val);
                    }
                }
            }
        }
    }

    input
        .iter()
        .map(|c| c.number)
        .find(|number| !seen_numbers.contains(number))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use day03::{get_input, parse_input, part1, part2, Claim};

    const TEST_INPUT: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    #[test]
    fn test_parse_input() {
        let claims = parse_input(TEST_INPUT);

        assert_eq!(
            claims,
            vec![
                Claim {
                    number: 1,
                    x: 1,
                    y: 3,
                    width: 4,
                    height: 4,
                },
                Claim {
                    number: 2,
                    x: 3,
                    y: 1,
                    width: 4,
                    height: 4,
                },
                Claim {
                    number: 3,
                    x: 5,
                    y: 5,
                    width: 2,
                    height: 2,
                }
            ]
        );
    }

    #[test]
    fn test_part1_examples() {
        let claims = parse_input(TEST_INPUT);

        assert_eq!(part1(&claims), 4);
    }

    #[test]
    fn test_part1() {
        let claims = parse_input(&get_input());

        assert_eq!(part1(&claims), 117505);
    }

    #[test]
    fn test_part2_examples() {
        let claims = parse_input(TEST_INPUT);

        assert_eq!(part2(&claims), 3);
    }

    #[test]
    fn test_part2() {
        let claims = parse_input(&get_input());

        assert_eq!(part2(&claims), 1254);
    }
}
