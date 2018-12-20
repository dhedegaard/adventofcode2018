use pathfinding::prelude::dijkstra_all;
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

pub fn parse_input(input: &str) -> HashMap<Point, (Point, i32)> {
    let mut map = BTreeMap::new();
    explore(
        &mut map,
        Point::new(0, 0),
        &input.chars().collect::<Vec<_>>(),
        &mut 1,
    );
    dijkstra_all(&Point::new(0, 0), |pos| {
        map.get(pos)
            .into_iter()
            .flat_map(|neighbours| neighbours.iter().map(|n| (*n, 1)))
    })
}

fn explore(
    map: &mut BTreeMap<Point, BTreeSet<Point>>,
    start: Point,
    input: &[char],
    index: &mut usize,
) -> Vec<Point> {
    let mut exits = vec![start];
    loop {
        match input[*index] {
            '|' | ')' | '$' => return exits,
            '(' => {
                let mut new_exits = BTreeSet::new();
                while input[*index] != ')' {
                    let old_index = *index;
                    new_exits.extend(exits.iter().flat_map(|pos| {
                        *index = old_index + 1;
                        explore(map, *pos, input, index)
                    }));
                }
                exits = new_exits.into_iter().collect();
            }
            dir => {
                let delta_pos = match dir {
                    'N' => Point { x: 0, y: -1 },
                    'E' => Point { x: 1, y: 0 },
                    'S' => Point { x: 0, y: 1 },
                    'W' => Point { x: -1, y: 0 },
                    _ => unreachable!(),
                };
                for pos in &mut exits {
                    let newpos = Point::new(pos.x + delta_pos.x, pos.y + delta_pos.y);
                    map.entry(*pos).or_insert_with(BTreeSet::new).insert(newpos);
                    *pos = newpos;
                }
            }
        }
        *index += 1;
    }
}

pub fn part1(map: &HashMap<Point, (Point, i32)>) -> i32 {
    map.values().map(|(_, e)| *e).max().unwrap()
}

fn part2(map: &HashMap<Point, (Point, i32)>) -> usize {
    map.values().filter(|&(_, c)| *c >= 1000).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse_input("^WNE$")), 3);
        assert_eq!(part1(&parse_input("^ENWWW(NEEE|SSE(EE|N))$")), 10);
        assert_eq!(
            part1(&parse_input("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$")),
            18
        );
        assert_eq!(
            part1(&parse_input(
                "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"
            )),
            23
        );
        assert_eq!(
            part1(&parse_input(
                "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"
            )),
            31
        );
    }

    #[test]
    fn part1_result() {
        assert_eq!(part1(&parse_input(&get_input())), 3966);
    }

    #[test]
    fn part2_result() {
        assert_eq!(part2(&parse_input(&get_input())), 8173);
    }
}
