use std::collections::HashSet;

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

type Input = Vec<(i64, i64, i64, i64, i64)>;

pub fn parse_input(input: &str) -> Input {
    let mut points = vec![];
    for (i, line) in input.lines().enumerate() {
        let el: Vec<i64> = line.split(',').filter_map(|x| x.parse().ok()).collect();
        // (a, b, c, d, constellation_id)
        points.push((el[0], el[1], el[2], el[3], i as i64));
    }
    points
}

fn dis(a: (i64, i64, i64, i64, i64), b: (i64, i64, i64, i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()
}

pub fn part1(input: &Input) -> usize {
    let mut input = input.to_vec();
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let p0 = input[i];
            let p1 = input[j];

            if dis(p0, p1) <= 3 && p0.4 != p1.4 {
                // merge constellations
                for k in 0..input.len() {
                    if input[k].4 == p1.4 {
                        input[k].4 = p0.4;
                    }
                }
            }
        }
    }

    let part1: HashSet<_> = input.iter().map(|x| x.4).collect();
    part1.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 2);
    }

    #[test]
    fn part1_result() {
        assert_eq!(part1(&parse_input(&get_input())), 331);
    }
}