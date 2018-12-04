extern crate regex;

use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct SleepInterval {
    start_minute: i32,
    end_minute: i32,
}

pub type GuardData = HashMap<i32, Vec<SleepInterval>>;

pub fn parse_input(input: &str) -> GuardData {
    let mut sorted_lines = input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    sorted_lines.sort();

    let match_minute_action = regex::Regex::new(r"^\[.+:(\d+)\] (.+)$").unwrap();
    let match_guard_id = regex::Regex::new(r"^.*Guard \#(\d+) begins.*$").unwrap();

    let mut guards = GuardData::new();
    let mut person = -1;
    let mut start_minute = -1;

    // Iterate on all the days in order, one guard is on duty each night.
    for line in sorted_lines {
        let captures = match_minute_action.captures(&line).unwrap();
        let minute = captures[1].parse::<i32>().unwrap();
        let action = captures[2].to_owned();

        match action.as_ref() {
            // The start of a new sleep interval.
            "falls asleep" => start_minute = minute,
            // A sleep interval is over, register the start/end minute.
            "wakes up" => guards.entry(person).or_default().push(SleepInterval {
                start_minute,
                end_minute: minute,
            }),
            // Determine the guard on duty, by the first line for the given day.
            _ => {
                let guard_match = match_guard_id.captures(&action).unwrap();
                person = guard_match[1].parse().unwrap();
            }
        }
    }

    // Returns the map of guards,
    guards
}

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

pub fn part1(input: &GuardData) -> i32 {
    // find the guard that slept the most minute in total.
    let (best_guard_id, sleep_intervals) = input
        .iter()
        .max_by_key(|(_, intervals)| -> i32 {
            // Sum the number of sleeping minutes for the given guard.
            intervals
                .iter()
                .map(|interval| interval.end_minute - interval.start_minute)
                .sum()
        }).unwrap();

    // Determine the number of days slept on each minute.
    let mut minute_counts: HashMap<i32, i32> = HashMap::new();
    for interval in sleep_intervals {
        for minute in interval.start_minute..interval.end_minute {
            *minute_counts.entry(minute).or_default() += 1;
        }
    }

    // Determine the minute with the highest number of days slept on.
    let best_minute = minute_counts
        .iter()
        .max_by_key(|&(_, value)| value)
        .unwrap()
        .0;

    best_minute * best_guard_id
}

pub fn part2(input: &GuardData) -> i32 {
    // (guard_id+minute) -> slept minutes.
    let mut guard_minute_count: HashMap<(i32, i32), i32> = HashMap::new();
    // Determine the number of times slept, pr minute, pr guard.
    for (guard_id, intervals) in input {
        for interval in intervals {
            for minute in interval.start_minute..interval.end_minute {
                *guard_minute_count.entry((*guard_id, minute)).or_default() += 1;
            }
        }
    }

    // Find the guard, and the minute, with the highest count of sleep.
    let (guard_id, minute) = guard_minute_count
        .iter()
        .max_by_key(|&(_, count)| count)
        .unwrap()
        .0;

    guard_id * minute
}

#[cfg(test)]
mod tests {
    use day04::{get_input, parse_input, part1, part2};

    const TEST_DATA: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn test_part1_examples() {
        let guard_data = parse_input(TEST_DATA);

        assert_eq!(part1(&guard_data), 10 * 24);
    }

    #[test]
    fn test_part1_result() {
        let guard_data = parse_input(&get_input());

        assert_eq!(part1(&guard_data), 39422);
    }

    #[test]
    fn test_part2_examples() {
        let guard_data = parse_input(TEST_DATA);

        assert_eq!(part2(&guard_data), 99 * 45);
    }

    #[test]
    fn test_part2_result() {
        let guard_data = parse_input(&get_input());

        assert_eq!(part2(&guard_data), 65474);
    }
}
