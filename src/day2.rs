use std::collections::HashMap;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<String>) -> usize {
    let twos = input.iter().filter(|s| has_n(freqs(s), 2)).count();
    let threes = input.iter().filter(|s| has_n(freqs(s), 3)).count();
    twos * threes
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<String>) -> String {
    "".to_string()
}

fn freqs(label: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();

    label.chars().for_each(|c| {
        if map.contains_key(&c) {
            *map.get_mut(&c).unwrap() += 1;
        } else {
            map.insert(c, 1);
        }
    });

    map
}

fn has_n(char_freqs: HashMap<char, i32>, n: i32) -> bool {
    char_freqs.iter().any(|(_, &v)| v == n)
}
