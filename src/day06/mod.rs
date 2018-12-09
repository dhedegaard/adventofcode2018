use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

pub fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| -> Point {
            let mapped = line
                .to_owned()
                .split(", ")
                .take(2)
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            Point {
                x: mapped[0],
                y: mapped[1],
            }
        })
        .collect()
}

pub fn part1(input: &Vec<Point>) -> u32 {
    // Determine the boundaries of the map.
    let min_x = input.iter().map(|e| e.x).min().unwrap();
    let max_x = input.iter().map(|e| e.x).max().unwrap();
    let min_y = input.iter().map(|e| e.y).min().unwrap();
    let max_y = input.iter().map(|e| e.y).max().unwrap();

    // Build a map with a distance of 0 to the destinations.
    let mut map = input.iter().map(|key| (key, 0)).collect::<HashMap<_, _>>();

    // Contains the destinations that touches the edges of the map.
    let mut at_edge = HashSet::new();

    // Loop through all the points in the grid inside the boundaries, and calculate the closest points based on the existing map data.
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            // Determine the closest destination, and the distance to it.
            let mut min_dist = None;
            let mut closest = None;
            for &point in map.keys() {
                // The manhattan distance.
                let distance = (point.x - x).abs() + (point.y - y).abs();
                if min_dist.is_none() || distance < min_dist.unwrap() {
                    min_dist = Some(distance);
                    closest = Some(point);
                } else if min_dist.is_some() && min_dist.unwrap() == distance {
                    closest = None;
                }
            }

            if closest.is_some() {
                let closest = closest.unwrap();
                *map.entry(&closest).or_insert(0) += 1;

                // If we're at the edge of the map, mark the destination as `at_edge`.
                if x <= min_x || x >= max_x || y <= min_y || y >= max_y {
                    at_edge.insert(closest);
                }
            }
        }
    }

    map.iter()
        // Filter away the destinations that touches the edge of the map.
        .filter(|(point, _)| !at_edge.contains(point.to_owned()))
        .max_by_key(|&(_, distance)| distance)
        .unwrap()
        .1
        .to_owned()
}

pub fn part2(input: &Vec<Point>, less_than: i32) -> u32 {
    let min_x = input.iter().map(|e| e.x).min().unwrap();
    let max_x = input.iter().map(|e| e.x).max().unwrap();
    let min_y = input.iter().map(|e| e.y).min().unwrap();
    let max_y = input.iter().map(|e| e.y).max().unwrap();

    let mut count = 0;

    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let mut sum: i32 = 0;
            for &point in input.iter() {
                sum += (point.x - x).abs() + (point.y - y).abs();
            }

            if sum < less_than {
                count += 1;
            }
        }
    }
    count as u32
}

#[cfg(test)]
mod tests {
    use day06::{get_input, parse_input, part1, part2};

    const TEST_INPUT: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 17);
    }

    #[test]
    fn part1_result() {
        assert_eq!(part1(&parse_input(&get_input())), 4143);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2(&parse_input(TEST_INPUT), 32), 16);
    }

    #[test]
    fn part2_results() {
        assert_eq!(part2(&parse_input(&get_input()), 10_000), 35039);
    }
}
