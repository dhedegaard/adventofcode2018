#[macro_use]
extern crate prettytable;
extern crate time;

use prettytable::format;
use prettytable::Table;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!["Day", "Part", "Result", "Duration"]);

    {
        let input = day01::parse_input(&day01::raw_input());
        let before = time::now();
        let result = day01::part1(&input);
        let diff = time::now() - before;
        table.add_row(row![1, 1, result, diff]);
    }
    {
        let input = day01::parse_input(&day01::raw_input());
        let before = time::now();
        let result = day01::part2(&input);
        let diff = time::now() - before;
        table.add_row(row![1, 2, result, diff]);
    }
    {
        let input = day02::get_input();
        let before = time::now();
        let result = day02::part1(&input);
        let diff = time::now() - before;
        table.add_row(row![2, 1, result, diff]);
    }
    {
        let input = day02::get_input();
        let before = time::now();
        let result = day02::part2(&input);
        let diff = time::now() - before;
        table.add_row(row![2, 2, result, diff]);
    }
    {
        let input = day03::parse_input(&day03::get_input());
        let before = time::now();
        let result = day03::part1(&input);
        let diff = time::now() - before;
        table.add_row(row![3, 1, result, diff]);
    }
    {
        let input = day03::parse_input(&day03::get_input());
        let before = time::now();
        let result = day03::part2(&input);
        let diff = time::now() - before;
        table.add_row(row![3, 2, result, diff]);
    }
    {
        let input = day04::parse_input(&day04::get_input());
        let before = time::now();
        let result = day04::part1(&input);
        let diff = time::now() - before;
        table.add_row(row![4, 1, result, diff]);
    }
    {
        let input = day04::parse_input(&day04::get_input());
        let before = time::now();
        let result = day04::part2(&input);
        let diff = time::now() - before;
        table.add_row(row![4, 2, result, diff]);
    }
    {
        let input = day05::get_input();
        let before = time::now();
        let result = day05::part1(&input).len();
        let diff = time::now() - before;
        table.add_row(row![5, 1, result, diff]);
    }
    {
        let input = day05::get_input();
        let before = time::now();
        let result = day05::part2(&input);
        let diff = time::now() - before;
        table.add_row(row![5, 2, result, diff]);
    }

    table.printstd();
}
