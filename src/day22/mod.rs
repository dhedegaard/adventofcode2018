use pathfinding::prelude::{absdiff, astar, Matrix};

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

pub fn parse_input(input: &str) -> (usize, (usize, usize)) {
    let mut lines = input.lines();
    let depth = lines.next().unwrap()[7..].parse::<usize>().unwrap();
    let mut target = lines.next().unwrap()[8..]
        .split(',')
        .map(|n| n.parse::<usize>().unwrap());
    (depth, (target.next().unwrap(), target.next().unwrap()))
}

fn calculate_levels(
    max_x: usize,
    max_y: usize,
    depth: usize,
    (tx, ty): (usize, usize),
) -> Matrix<usize> {
    let mut el = Matrix::new(max_x + 1, max_y + 1, 0);
    for x in 0..=max_x {
        for y in 0..=max_y {
            let index = if (x == 0 && y == 0) || (x == tx && y == ty) {
                0
            } else if y == 0 {
                x * 16807
            } else if x == 0 {
                y * 48271
            } else {
                el[&(x - 1, y)] * el[&(x, y - 1)]
            };
            el[&(x, y)] = (index + depth) % 20183;
        }
    }
    el.as_mut().iter_mut().for_each(|n| *n %= 3);
    el
}

pub fn part1(depth: usize, (tx, ty): (usize, usize)) -> usize {
    calculate_levels(tx, ty, depth, (tx, ty))
        .to_vec()
        .into_iter()
        .sum()
}

pub fn part2(depth: usize, (tx, ty): (usize, usize)) -> usize {
    let neither = 1;
    let torch = 2;
    let gear = 4;
    let allowed = [torch + gear, neither + gear, neither + torch];
    let map = calculate_levels(tx * 7, ty * 7, depth, (tx, ty));
    let (_, cost) = astar(
        &((0, 0), torch),
        |&((x, y), eq)| {
            map.neighbours(&(x, y), false)
                .filter(|&(nx, ny)| allowed[map[&(nx, ny)]] & eq == eq)
                .map(|(nx, ny)| (((nx, ny), eq), 1))
                .chain(std::iter::once((((x, y), allowed[map[&(x, y)]] - eq), 7)))
                .collect::<Vec<_>>()
        },
        |&((x, y), _)| absdiff(x, tx) + absdiff(y, ty),
        |&((x, y), eq)| x == tx && y == ty && eq == torch,
    )
    .unwrap();
    cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_results() {
        let (depth, target) = parse_input(&get_input());
        assert_eq!(part1(depth, target), 11810);
    }

    #[test]
    fn part2_results() {
        let (depth, target) = parse_input(&get_input());
        assert_eq!(part2(depth, target), 1015);
    }
}
