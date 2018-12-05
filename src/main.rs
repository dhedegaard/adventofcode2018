extern crate time;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    {
        let input = day01::parse_input(&day01::raw_input());
        let before = time::now();
        let result = day01::part1(&input);
        let diff = time::now() - before;
        println!("day01,\tpart1: {},\ttook {}", result, diff);
    }
    {
        let input = day01::parse_input(&day01::raw_input());
        let before = time::now();
        let result = day01::part2(&input);
        let diff = time::now() - before;
        println!("day01,\tpart2: {},\ttook {}", result, diff);
    }
    {
        let input = day02::get_input();
        let before = time::now();
        let result = day02::part1(&input);
        let diff = time::now() - before;
        println!("day02,\tpart1: {},\ttook {}", result, diff);
    }
    {
        let input = day02::get_input();
        let before = time::now();
        let result = day02::part2(&input);
        let diff = time::now() - before;
        println!("day02,\tpart2: {},\ttook {}", result, diff);
    }
    {
        let input = day03::parse_input(&day03::get_input());
        let before = time::now();
        let result = day03::part1(&input);
        let diff = time::now() - before;
        println!("day03,\tpart1: {},\ttook {}", result, diff);
    }
    {
        let input = day03::parse_input(&day03::get_input());
        let before = time::now();
        let result = day03::part2(&input);
        let diff = time::now() - before;
        println!("day03,\tpart2: {},\ttook {}", result, diff);
    }
    {
        let input = day04::parse_input(&day04::get_input());
        let before = time::now();
        let result = day04::part1(&input);
        let diff = time::now() - before;
        println!("day04,\tpart1: {},\ttook {}", result, diff);
    }
    {
        let input = day04::parse_input(&day04::get_input());
        let before = time::now();
        let result = day04::part2(&input);
        let diff = time::now() - before;
        println!("day04,\tpart2: {},\ttook {}", result, diff);
    }
}
