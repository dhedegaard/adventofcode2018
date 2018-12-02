extern crate time;

mod day01;
mod day02;

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
}
