#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    pub fn new(x: i64, y: i64, z: i64) -> Position {
        Position { x, y, z }
    }

    pub fn distance_to(&self, other_pos: &Position) -> i64 {
        (self.x - other_pos.x).abs() + (self.y - other_pos.y).abs() + (self.z - other_pos.z).abs()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Nanobot {
    pos: Position,
    radius: i64,
}

impl Nanobot {
    pub fn new(pos: Position, radius: i64) -> Nanobot {
        Nanobot { pos, radius }
    }

    pub fn in_range(&self, other_bot: &Nanobot) -> bool {
        self.radius >= self.pos.distance_to(&other_bot.pos)
    }
}

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

pub fn parse_input(input: &str) -> Vec<Nanobot> {
    let m_nanobot = regex::Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = m_nanobot.captures(line).unwrap();
            Nanobot::new(
                Position::new(
                    captures[1].parse().unwrap(),
                    captures[2].parse().unwrap(),
                    captures[3].parse().unwrap(),
                ),
                captures[4].parse().unwrap(),
            )
        })
        .collect()
}

pub fn part1(nanobots: &[Nanobot]) -> usize {
    let max_bot = nanobots.iter().max_by_key(|e| e.radius).unwrap();

    let mut total = 0;
    for other in nanobots {
        if max_bot.in_range(other) {
            total += 1;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1_examples() {
        let nanobots = parse_input(TEST_INPUT);
        assert_eq!(part1(&nanobots), 7);
    }

    #[test]
    fn part1_result() {
        let nanobots = parse_input(&get_input());
        assert_eq!(part1(&nanobots), 326);
    }
}
