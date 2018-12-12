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
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

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
    {
        let input = day06::get_input();
        let before = time::now();
        let result = day06::part1(&day06::parse_input(&input));
        let diff = time::now() - before;
        table.add_row(row![6, 1, result, diff]);
    }
    {
        let input = day06::get_input();
        let before = time::now();
        let result = day06::part2(&day06::parse_input(&input), 10_000);
        let diff = time::now() - before;
        table.add_row(row![6, 2, result, diff]);
    }
    {
        let input = day07::get_input();
        let before = time::now();
        let result = day07::part1(&mut day07::parse_input(&input));
        let diff = time::now() - before;
        table.add_row(row![7, 1, result, diff]);
    }
    {
        let input = day07::get_input();
        let before = time::now();
        let result = day07::part2(&day07::parse_input_part2(&input), 5, 60);
        let diff = time::now() - before;
        table.add_row(row![7, 2, result, diff]);
    }
    {
        let input = day08::get_input();
        let before = time::now();
        let result = day08::part1(&day08::parse_input(&input));
        let diff = time::now() - before;
        table.add_row(row![8, 1, result, diff]);
    }
    {
        let input = day08::get_input();
        let before = time::now();
        let result = day08::part2(&day08::parse_input(&input));
        let diff = time::now() - before;
        table.add_row(row![8, 2, result, diff]);
    }
    {
        let input = day09::get_input();
        let before = time::now();
        let result = day09::part1(input.0, input.1);
        let diff = time::now() - before;
        table.add_row(row![9, 1, result, diff]);
    }
    {
        let input = day09::get_input();
        let before = time::now();
        let result = day09::part2(input.0, input.1);
        let diff = time::now() - before;
        table.add_row(row![9, 2, result, diff]);
    }
    {
        let input = day10::get_input();
        let before = time::now();
        let result = day10::part1(&day10::parse_input(&input));
        let diff = time::now() - before;
        table.add_empty_row();
        table.add_row(row![10, 1, result.trim(), diff]);
        table.add_empty_row();
    }
    {
        let input = day10::get_input();
        let before = time::now();
        let result = day10::part2(&day10::parse_input(&input));
        let diff = time::now() - before;
        table.add_row(row![10, 2, result, diff]);
    }
    {
        let input = day11::get_input();
        let before = time::now();
        let result = day11::part1(input);
        let diff = time::now() - before;
        table.add_row(row![11, 1, format!("{:?}", result), diff]);
    }
    {
        let input = day11::get_input();
        let before = time::now();
        let result = day11::part2(input);
        let diff = time::now() - before;
        table.add_row(row![11, 2, format!("{:?}", result), diff]);
    }
    {
        let (state, instructions) = day12::parse_input(&day12::get_input());
        let before = time::now();
        let result = day12::part1(state, &instructions);
        let diff = time::now() - before;
        table.add_row(row![12, 1, format!("{:?}", result), diff]);
    }
    {
        let (state, instructions) = day12::parse_input(&day12::get_input());
        let before = time::now();
        let result = day12::part2(state, &instructions);
        let diff = time::now() - before;
        table.add_row(row![12, 2, format!("{:?}", result), diff]);
    }

    table.printstd();
}
