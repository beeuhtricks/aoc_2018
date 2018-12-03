use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
    let mut uniq = HashSet::new();
    let mut frequency = 0;

    input.iter().cycle().find_map(|d| {
        if uniq.insert(d) {
            frequency += d;
            None
        } else {
            Some(frequency)
        }
    }).unwrap()
}
