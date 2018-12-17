pub struct Grid {
    grid: Vec<Vec<char>>,
    min_y: usize,
    max_y: usize,
}

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

impl Grid {
    /// Let the water flow until there are no more changes in the grid.
    pub fn flow(&mut self) {
        while self.drip(0, 500) {}
    }

    fn drip(&mut self, start_y: usize, start_x: usize) -> bool {
        for y in start_y..self.grid.len() {
            // Flow down through the dots.
            if self.grid[y][start_x] == '|' || self.grid[y][start_x] == '.' {
                self.grid[y][start_x] = '|';
                continue;
            }

            let flow_y = y - 1;
            let mut made_progress = false;
            let mut hit_left_wall = false;
            let mut hit_right_wall = false;

            // Fill to the left, until we're not making progress.
            for x in (0..start_x).rev() {
                if self.grid[flow_y][x] == '#' {
                    hit_left_wall = true;
                    break;
                }
                self.grid[flow_y][x] = '|';
                if self.grid[y][x] != '#' && self.grid[y][x] != '~' {
                    if self.drip(flow_y, x) {
                        made_progress = true;
                    }
                    break;
                }
            }

            // Fill to the right, until we're not making progress.
            for x in start_x..2000 {
                if self.grid[flow_y][x] == '#' {
                    hit_right_wall = true;
                    break;
                }
                self.grid[flow_y][x] = '|';
                if self.grid[y][x] != '#' && self.grid[y][x] != '~' {
                    if self.drip(flow_y, x) {
                        made_progress = true;
                    }
                    break;
                }
            }

            // If we hit walls, then switch our '|'s to '~'s
            if hit_left_wall && hit_right_wall {
                made_progress = true;
                for x in (0..start_x).rev() {
                    if self.grid[flow_y][x] != '|' {
                        break;
                    }
                    self.grid[flow_y][x] = '~';
                }
                for x in start_x..self.grid[flow_y].len() {
                    if self.grid[flow_y][x] != '|' {
                        break;
                    }
                    self.grid[flow_y][x] = '~';
                }
            }
            return made_progress;
        }
        false
    }

    /// Find all the water.
    pub fn part1(&self) -> usize {
        self.grid
            .iter()
            .skip(self.min_y)
            .take(self.max_y - self.min_y + 1)
            .map(|row| row.iter().filter(|&c| c == &'~' || c == &'|').count())
            .sum()
    }

    /// Find all the settled water.
    pub fn part2(&self) -> usize {
        self.grid
            .iter()
            .skip(self.min_y)
            .take(self.max_y - self.min_y + 1)
            .map(|row| row.iter().filter(|&c| c == &'~').count())
            .sum()
    }

    pub fn from_input(input: &str) -> Grid {
        let re_input = regex::Regex::new(r"(\w)=(\d+), (\w)=(\d+)..(\d+)").unwrap();

        let mut grid = vec![vec!['.'; 2000]; 2000];
        let mut min_y = usize::max_value();
        let mut max_y = 0;

        for line in input.lines() {
            let caps = re_input.captures(line).unwrap();
            let c1 = caps[1].to_string();
            let c2 = caps[2].parse::<usize>().unwrap();
            let c4 = caps[4].parse::<usize>().unwrap();
            let c5 = caps[5].parse::<usize>().unwrap();

            if c1 == "x" {
                for (y, row) in grid.iter_mut().enumerate().take(c5+1).skip(c4) {
                    max_y = std::cmp::max(max_y, y);
                    min_y = std::cmp::min(min_y, y);
                    row[c2] = '#';
                }
            } else {
                min_y = std::cmp::min(min_y, c2);
                max_y = std::cmp::max(max_y, c2);
                for x in c4..=c5 {
                    grid[c2][x] = '#';
                }
            }
        }
        Grid { grid, min_y, max_y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1_example() {
        let mut grid = Grid::from_input(TEST_INPUT);
        grid.flow();
        assert_eq!(grid.part1(), 57);
    }

    #[test]
    fn part1_result() {
        let mut grid = Grid::from_input(&get_input());
        grid.flow();
        assert_eq!(grid.part1(), 34775);
    }

    #[test]
    fn part2_example() {
        let mut grid = Grid::from_input(TEST_INPUT);
        grid.flow();
        assert_eq!(grid.part2(), 29);
    }

    #[test]
    fn part2_result() {
        let mut grid = Grid::from_input(&get_input());
        grid.flow();
        assert_eq!(grid.part2(), 27086);
    }
}
