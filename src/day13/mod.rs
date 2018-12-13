use std::collections::HashSet;

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
/// A two-dimensional direction.
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Inter {
    Left = 0,
    Straight = 1,
    Right = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
/// A cart with a position and a direction.
pub struct Cart {
    position: Position,
    direction: Dir,
    intersection: Inter,
}

pub fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Cart>) {
    let board = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let mut carts = vec![];
    for (y, row) in board.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let direction = match c {
                '<' => Some(Dir::Left),
                '>' => Some(Dir::Right),
                '^' => Some(Dir::Up),
                'v' => Some(Dir::Down),
                _ => None,
            };
            if direction.is_none() {
                continue;
            }
            carts.push(Cart {
                direction: direction.unwrap(),
                position: Position { x, y },
                intersection: Inter::Left,
            })
        }
    }
    (board, carts)
}

impl Cart {
    fn tick(&mut self, c: char) {
        self.direction = match c {
            // Change direction when we hit a corner.
            '/' => match self.direction {
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Right => Dir::Up,
                Dir::Up => Dir::Right,
            },
            '\\' => match self.direction {
                Dir::Left => Dir::Up,
                Dir::Up => Dir::Left,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Right,
            },
            // Handle intersections.
            '+' => match self.intersection {
                Inter::Left => {
                    self.intersection = Inter::Straight;
                    match self.direction {
                        Dir::Left => Dir::Down,
                        Dir::Down => Dir::Right,
                        Dir::Right => Dir::Up,
                        Dir::Up => Dir::Left,
                    }
                }
                Inter::Straight => {
                    self.intersection = Inter::Right;
                    self.direction
                }
                Inter::Right => {
                    self.intersection = Inter::Left;
                    match self.direction {
                        Dir::Left => Dir::Up,
                        Dir::Up => Dir::Right,
                        Dir::Right => Dir::Down,
                        Dir::Down => Dir::Left,
                    }
                }
            },
            // Otherwise, proceed.
            _ => self.direction,
        };
        // Move in that direction.
        match self.direction {
            Dir::Left => self.position.x -= 1,
            Dir::Right => self.position.x += 1,
            Dir::Up => self.position.y -= 1,
            Dir::Down => self.position.y += 1,
        };
    }
}

pub fn part1(board: &[Vec<char>], initial_carts: &[Cart]) -> Position {
    let mut carts = initial_carts.to_vec();
    loop {
        // All the positions, for all the carts, before ticking anything.
        let mut positions = carts.iter().map(|c| c.position).collect::<HashSet<_>>();

        // Sort the carts to make sure we tick them in the right order.
        carts.sort_by(|a, b| {
            let p1 = a.position;
            let p2 = b.position;
            (p1.y, p1.x).cmp(&(p2.y, p2.x))
        });

        // Tick all the carts, while checking for collisions.
        for cart in &mut carts {
            let c = board[cart.position.y][cart.position.x];
            positions.remove(&cart.position);
            cart.tick(c);
            if !positions.insert(cart.position) {
                return cart.position;
            }
        }
    }
}

pub fn part2(board: &[Vec<char>], initial_carts: &[Cart]) -> Position {
    let mut carts = initial_carts.to_vec();
    loop {
        // If we're done, return the last carts position.
        if carts.len() == 1 {
            return carts[0].position;
        }

        let mut positions = carts.iter().map(|c| c.position).collect::<HashSet<_>>();
        let mut crashed_locations = HashSet::new();

        // Sort the carts to make sure we tick them in the right order.
        carts.sort_by(|a, b| {
            let p1 = a.position;
            let p2 = b.position;
            (p1.y, p1.x).cmp(&(p2.y, p2.x))
        });

        // Tick all the carts, while checking for collisions.
        for cart in &mut carts {
            // if we're at a crash position, don't move, you're dead.
            if crashed_locations.contains(&cart.position) {
                continue;
            }
            // Move the cart.
            let c = board[cart.position.y][cart.position.x];
            positions.remove(&cart.position);
            cart.tick(c);

            // If we're in a collision, mark the position as dead, keeping all future carts at the same place.
            if !positions.insert(cart.position) {
                crashed_locations.insert(cart.position);
            }
        }

        // Remove all colliding carts.
        carts = carts
            .iter()
            .filter(|&c| !crashed_locations.contains(&c.position))
            .cloned()
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");
    const TEST_INPUT2: &str = include_str!("test2.txt");

    #[test]
    fn parse_example_carts() {
        let (_, carts) = parse_input(TEST_INPUT);
        assert_eq!(carts.len(), 2);
        assert_eq!(
            carts[0],
            Cart {
                direction: Dir::Right,
                position: Position { x: 2, y: 0 },
                intersection: Inter::Left
            }
        );
        assert_eq!(
            carts[1],
            Cart {
                direction: Dir::Down,
                position: Position { x: 9, y: 3 },
                intersection: Inter::Left
            }
        );
    }

    #[test]
    fn part1_examples() {
        let (board, carts) = parse_input(TEST_INPUT);
        assert_eq!(part1(&board, &carts), Position { x: 7, y: 3 });
    }

    #[test]
    fn part1_result() {
        let (board, carts) = parse_input(&get_input());
        assert_eq!(part1(&board, &carts), Position { x: 41, y: 22 });
    }

    #[test]
    fn part2_examples() {
        let (board, carts) = parse_input(TEST_INPUT2);
        assert_eq!(part2(&board, &carts), Position { x: 6, y: 4 });
    }

    #[test]
    fn part2_result() {
        let (board, carts) = parse_input(&get_input());
        assert_eq!(part2(&board, &carts), Position { x: 84, y: 90 });
    }
}
