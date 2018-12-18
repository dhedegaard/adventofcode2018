use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Acre {
    Open,
    Trees,
    Lumberyard,
}

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

pub fn parse_input(input: &str) -> Vec<Vec<Acre>> {
    let mut result = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(match c {
                '.' => Acre::Open,
                '|' => Acre::Trees,
                '#' => Acre::Lumberyard,
                _ => unreachable!(),
            })
        }
        result.push(row);
    }
    result
}

fn count_acretype_around_point(area: &[Vec<Acre>], x: usize, y: usize, acretype: Acre) -> usize {
    let mut count = 0;
    let y = y as i32;
    let x = x as i32;
    for dy in y - 1..=y + 1 {
        if dy < 0 || dy >= area.len() as i32 {
            continue;
        }
        let old_row = &area[dy as usize][..];
        for dx in x - 1..=x + 1 {
            if x == dx && y == dy || dx < 0 || dx >= old_row.len() as i32 {
                continue;
            }
            if old_row[dx as usize] == acretype {
                count += 1;
            }
        }
    }
    count
}

pub fn tick(area: &[Vec<Acre>]) -> Vec<Vec<Acre>> {
    let mut result = Vec::with_capacity(area.len());
    for (y, old_row) in area.iter().enumerate() {
        let mut row = Vec::with_capacity(old_row.len());
        for (x, old_acre) in old_row.iter().enumerate() {
            let lumberyard_count = count_acretype_around_point(&area, x, y, Acre::Lumberyard);
            let tree_count = count_acretype_around_point(&area, x, y, Acre::Trees);
            let new_acre = *match old_acre {
                Acre::Open => {
                    if tree_count >= 3 {
                        &Acre::Trees
                    } else {
                        old_acre
                    }
                }
                Acre::Trees => {
                    if lumberyard_count >= 3 {
                        &Acre::Lumberyard
                    } else {
                        old_acre
                    }
                }
                Acre::Lumberyard => {
                    if lumberyard_count >= 1 && tree_count >= 1 {
                        old_acre
                    } else {
                        &Acre::Open
                    }
                }
            };
            row.push(new_acre);
        }
        result.push(row);
    }
    result
}

fn total_acretype_count(area: &[Vec<Acre>], acretype: Acre) -> usize {
    area.iter()
        .map(|row| row.iter().filter(|&e| e == &acretype).count())
        .sum()
}

pub fn part1(area: &[Vec<Acre>]) -> usize {
    let mut area = area.to_vec();
    for _ in 0..10 {
        area = tick(&area);
    }
    total_acretype_count(&area, Acre::Trees) * total_acretype_count(&area, Acre::Lumberyard)
}

pub fn part2(area: &[Vec<Acre>]) -> usize {
    let mut area = area.to_vec();
    let mut minute = 1;
    let mut seen = HashMap::new();
    let cap = 1_000_000_000;
    seen.insert(area.clone(), minute);
    while minute <= cap {
        // Tick the area and proceed to the next minute.
        area = tick(&area);
        minute += 1;

        // Check to see if we've hit this area previously.
        if seen.contains_key(&area) {
            let cycle_start = seen[&area];
            let delta = minute - cycle_start;
            let remaining = cap - minute;
            let cycle_count = remaining / delta;
            // Automate as many cycles as possible.
            minute += cycle_count * delta;
        }
        seen.insert(area.clone(), minute);
    }
    total_acretype_count(&area, Acre::Trees) * total_acretype_count(&area, Acre::Lumberyard)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 1147);
    }

    #[test]
    fn part1_result() {
        assert_eq!(part1(&parse_input(&get_input())), 394420);
    }

    #[test]
    fn part2_result() {
        assert_eq!(part2(&parse_input(&get_input())), 174420);
    }
}
