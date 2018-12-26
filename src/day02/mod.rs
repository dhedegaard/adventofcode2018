use std::collections::HashMap;

pub fn get_input<'a>() -> Vec<&'a str> {
    include_str!("input.txt").lines().collect::<Vec<_>>()
}

pub fn part1(input: &[&str]) -> u32 {
    let mut twos = 0;
    let mut threes = 0;

    for elem in input {
        let mut chars: HashMap<char, u32> = HashMap::new();
        for c in elem.chars() {
            *chars.entry(c).or_insert(0) += 1;
        }
        if chars.values().any(|&count| count == 2) {
            twos += 1;
        }
        if chars.values().any(|&count| count == 3) {
            threes += 1;
        }
    }
    twos * threes
}

pub fn part2(input: &[&str]) -> String {
    for (idx, id) in input.iter().enumerate() {
        for id2 in input.iter().skip(idx + 1) {
            // If there's only a single digit different between the two ID's.
            if id.chars().zip(id2.chars()).filter(|(a, b)| a != b).count() == 1 {
                // Return the characters that are equal, ignore the characters that differ between the IDs.
                return id
                    .chars()
                    .zip(id2.chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect();
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        let input = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];

        assert_eq!(part1(&input[0..1]), 0); // No match
        assert_eq!(part1(&input[1..2]), 1); // Matches 2 and 3 (1 * 1).

        assert_eq!(part1(&input[..]), 3 * 4)
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(part1(&get_input()[..]), 8296);
    }

    #[test]
    fn test_part2_examples() {
        let input = vec![
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];

        assert_eq!(part2(&input[..]), "fgij");
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(part2(&get_input()[..]), "pazvmqbftrbeosiecxlghkwud");
    }
}
