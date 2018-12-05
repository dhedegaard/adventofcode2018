use std::collections::HashSet;

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

pub fn part1(input: &str) -> String {
    let mut result = input.chars().collect::<Vec<_>>();
    let mut i = 0;
    while !result.is_empty() && i < result.len() - 1 {
        let c = result[i];
        let c2 = result[i + 1];
        if c != c2 && c.to_ascii_lowercase() == c2.to_ascii_lowercase() {
            result.remove(i);
            result.remove(i);
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }
    result.iter().collect()
}

pub fn part2(input: &str) -> usize {
    let mut min_length = input.len();
    for unit in input.to_ascii_lowercase().chars().collect::<HashSet<_>>() {
        // Remove the selected unit.
        let polymer = input.to_owned().replace(unit, "").replace(unit.to_ascii_uppercase(), "");
        let length = part1(&polymer).len();
        if length < min_length {
            min_length = length;
        }
    }
    min_length
}

#[cfg(test)]
mod tests {
    use day05::{part1, get_input, part2};

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1("aA"), "");
        assert_eq!(part1("abBA"), "");
        assert_eq!(part1("abAB"), "abAB");
        assert_eq!(part1("aabAAB"), "aabAAB");
        assert_eq!(part1("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    }

    #[test]
    fn test_part1_result() {
        assert_eq!(part1(&get_input()).len(), 9562);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(part2("dabAcCaCBAcCcaDA"), 4);
    }

    #[test]
    fn test_part2_result() {
        assert_eq!(part2(&get_input()), 4934);
    }
}