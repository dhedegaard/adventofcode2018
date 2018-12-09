use std::collections::VecDeque;

pub fn get_input() -> (usize, usize) {
    let input = include_str!("input.txt")
        .split_whitespace()
        .map(|e| e.parse())
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();
    (input[0], input[1])
}

pub fn part1(players: usize, marbles: usize) -> usize {
    let mut circle = VecDeque::with_capacity(marbles);
    circle.push_back(0);
    let mut scores = vec![0; players];

    for i in 1..=marbles {
        if i % 23 == 0 {
            scores[i % players] += i;
            for _ in 0..7 {
                let back = circle.pop_back().unwrap();
                circle.push_front(back);
            }
            scores[i % players] += circle.pop_front().unwrap();
        } else {
            for _ in 0..2 {
                let front = circle.pop_front().unwrap();
                circle.push_back(front);
            }
            circle.push_front(i);
        }
    }

    scores.iter().max().unwrap().to_owned()
}

pub fn part2(players: usize, marbles: usize) -> usize {
    part1(players, marbles * 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1(9, 25), 32);
        assert_eq!(part1(10, 1618), 8317);
        assert_eq!(part1(13, 7999), 146373);
        assert_eq!(part1(17, 1104), 2764);
        assert_eq!(part1(21, 6111), 54718);
        assert_eq!(part1(30, 5807), 37305);
    }

    #[test]
    fn part1_result() {
        let (players, marbles) = get_input();
        assert_eq!(part1(players, marbles), 439635);
    }

    #[test]
    fn part2_result() {
        let (players, marbles) = get_input();
        assert_eq!(part2(players, marbles), 3562722971);
    }
}
