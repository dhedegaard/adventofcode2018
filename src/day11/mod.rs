pub fn get_input() -> i32 {
    include_str!("input.txt").parse().unwrap()
}

pub fn calc_power_level(coord: (i32, i32), input: i32) -> i32 {
    let rack_id = coord.0 + 10;
    let power_level = ((rack_id * coord.1) + input) * rack_id;
    ((power_level / 100) % 10) - 5
}

pub fn part1(input: i32) -> (i32, i32) {
    let mut grid = Vec::with_capacity(300 * 300);
    for y in 0..300 {
        for x in 0..300 {
            grid.push(calc_power_level((x as i32, y as i32), input));
        }
    }
    let mut max = 0;
    let mut max_pos = (0, 0);
    for y in 0..300 - 3 {
        for x in 0..300 - 3 {
            let sum = grid[x + 300 * y]
                + grid[x + 1 + 300 * y]
                + grid[x + 2 + 300 * y]
                + grid[x + 300 * (y + 1)]
                + grid[x + 1 + 300 * (y + 1)]
                + grid[x + 2 + 300 * (y + 1)]
                + grid[x + 300 * (y + 2)]
                + grid[x + 1 + 300 * (y + 2)]
                + grid[x + 2 + 300 * (y + 2)];
            if sum > max {
                max_pos = (x as i32, y as i32);
                max = sum;
            }
        }
    }
    max_pos
}

pub fn part2(input: i32) -> (i32, i32, i32) {
    let mut grid = Vec::with_capacity(300 * 300);
    for y in 0..300 {
        for x in 0..300 {
            grid.push(calc_power_level((x as i32, y as i32), input));
        }
    }
    let mut max = 0;
    let mut max_pos = (0, 0);
    let mut max_grid_size = 0;
    for y in 0..300 {
        for x in 0..300 {
            let mut sum = 0;
            for grid_size in 1..(300 - std::cmp::max(y, x)) {
                // Sum the new row.
                for dx in x..x + grid_size {
                    sum += grid[dx + (y + grid_size) * 300];
                }
                // Sum the new column.
                for dy in y..y + grid_size {
                    sum += grid[(x + grid_size) + dy * 300];
                }
                // Rembmer the new point where the row/column meets.
                sum += grid[(x + grid_size) + (y + grid_size) * 300];
                if sum > max {
                    max_pos = (x as i32, y as i32);
                    max = sum;
                    max_grid_size = (grid_size + 1) as i32;
                }
            }
        }
    }
    (max_pos.0, max_pos.1, max_grid_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_power_level() {
        assert_eq!(calc_power_level((3, 5), 8), 4);
        assert_eq!(calc_power_level((122, 79), 57), -5);
        assert_eq!(calc_power_level((217, 196), 39), 0);
        assert_eq!(calc_power_level((101, 153), 71), 4);
    }

    #[test]
    fn part1_examples() {
        assert_eq!(part1(18), (33, 45));
        assert_eq!(part1(42), (21, 61));
    }

    #[test]
    fn part1_result() {
        assert_eq!(part1(get_input()), (235, 14));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2(18), (90, 269, 16));
    }

    #[test]
    fn part2_result() {
        assert_eq!(part2(get_input()), (237, 227, 14));
    }
}
