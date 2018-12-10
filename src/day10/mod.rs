extern crate regex;

use std::collections::HashSet;

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Point {
    /// Moves the point according to the velocity, returning a new point at the new location.
    fn tick(&self) -> Point {
        Point {
            position: (
                self.position.0 + self.velocity.0,
                self.position.1 + self.velocity.1,
            ),
            velocity: self.velocity,
        }
    }
}

/// Calculates the total grid size for the given input.
fn grid_size(points: &[Point]) -> i32 {
    (points.iter().map(|e| e.position.0).max().unwrap()
        - points.iter().map(|e| e.position.0).min().unwrap())
        * (points.iter().map(|e| e.position.1).max().unwrap()
            + points.iter().map(|e| e.position.1).min().unwrap())
}

pub fn parse_input(input: &str) -> Vec<Point> {
    let match_input =
        regex::Regex::new(r"^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>$")
            .unwrap();
    input
        .lines()
        .map(|line| -> Point {
            let capt = match_input.captures(&line).unwrap();
            Point {
                position: (capt[1].parse().unwrap(), capt[2].parse().unwrap()),
                velocity: (capt[3].parse().unwrap(), capt[4].parse().unwrap()),
            }
        })
        .collect()
}

/// Generates ASCII graphics for your eyes only :)
fn generate_board_graphics(board: &[Point]) -> String {
    // Determine the board boundaries + 1 free column/row in each dimension.
    let min_x = board.iter().map(|e| e.position.1).min().unwrap() - 1;
    let max_x = board.iter().map(|e| e.position.1).max().unwrap() + 1;
    let min_y = board.iter().map(|e| e.position.0).min().unwrap() - 1;
    let max_y = board.iter().map(|e| e.position.0).max().unwrap() + 1;

    let mut result = String::with_capacity(1 + ((max_x - min_x) * (max_y - min_y) + 1) as usize);
    result.push_str("\n");
    let points = board.iter().map(|e| e.position).collect::<HashSet<_>>();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if points.contains(&(y, x)) {
                result.push('#');
            } else {
                result.push('.');
            }
        }
        result.push_str("\n");
    }
    result
}

fn solve(input: &[Point]) -> (String, u32) {
    let mut seconds = 0;
    let mut board = input.iter().map(|p| p.clone()).collect::<Vec<_>>();
    let mut current_grid_size = grid_size(&board);
    loop {
        // Tick all the points.
        let new_board = board.iter().map(|p| p.tick()).collect::<Vec<_>>();

        // Keep going until the board grid size is increasing.
        let new_grid_size = grid_size(&new_board);
        if new_grid_size >= current_grid_size {
            break;
        }

        // The board is still shrinking.
        seconds += 1;
        current_grid_size = new_grid_size;
        board = new_board;
    }
    // Return a string with pretty ASCII art to the caller.
    (generate_board_graphics(&board), seconds)
}

pub fn part1(input: &[Point]) -> String {
    solve(input).0
}

pub fn part2(input: &[Point]) -> u32 {
    solve(input).1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("example.txt");

    #[test]
    fn part1_examples() {
        assert_eq!(
            part1(&parse_input(TEST_INPUT)),
            "
............
.#...#..###.
.#...#...#..
.#...#...#..
.#####...#..
.#...#...#..
.#...#...#..
.#...#...#..
.#...#..###.
............
"
        );
    }

    #[test]
    fn part1_result() {
        assert_eq!(
            part1(&parse_input(&get_input())),
            "
................................................................
.#....#..######..#....#..#####...#.......#####...#....#..#....#.
.##...#..#.......#....#..#....#..#.......#....#..#....#..#...#..
.##...#..#........#..#...#....#..#.......#....#...#..#...#..#...
.#.#..#..#........#..#...#....#..#.......#....#...#..#...#.#....
.#.#..#..#####.....##....#####...#.......#####.....##....##.....
.#..#.#..#.........##....#.......#.......#..#......##....##.....
.#..#.#..#........#..#...#.......#.......#...#....#..#...#.#....
.#...##..#........#..#...#.......#.......#...#....#..#...#..#...
.#...##..#.......#....#..#.......#.......#....#..#....#..#...#..
.#....#..######..#....#..#.......######..#....#..#....#..#....#.
................................................................
"
        );
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 3);
    }

    #[test]
    fn part2_result() {
        assert_eq!(part2(&parse_input(&get_input())), 10459);
    }
}
