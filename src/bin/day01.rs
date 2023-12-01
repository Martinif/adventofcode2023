use std::collections::BTreeMap;

use adventofcode2023::load_input;

const NUMBERS_WRITTEN: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9)
];

fn prepare_input(s: String) -> String {
    s
}

fn line2number_part1(line: &str) -> u32 {
    let numbers: Vec<_> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    numbers.first().unwrap() * 10 + numbers.last().unwrap()
}


fn line2number_part2(line: &str) -> u32 {
    let mut entries = BTreeMap::new();
    for (text, v) in NUMBERS_WRITTEN {
        let mut entries_for_number: BTreeMap<_, _> = line
            .match_indices(text)
            .map(|(idx, _text)| (idx, v))
            .collect();
        entries.append(&mut entries_for_number);
    }

    for (idx, c) in line.chars().enumerate() {
        if c.is_ascii_digit() {
            entries.insert(idx, c.to_digit(10).unwrap());
        }
    }
    (entries.first_key_value().unwrap().1) * 10 + (entries.last_key_value().unwrap().1)
}


fn part1(input: &String) -> u32 {
    input.lines().map(|line| line2number_part1(line)).sum()
}

fn part2(input: &String) -> u32 {
    input.lines().map(|line| line2number_part2(line)).sum()
}

fn main() {
    let data = load_input("input/01.txt");
    let input = prepare_input(data);
    println!("First task: {:?}", part1(&input));
    println!("Second task: {:?}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/01.test_part1.txt");
        let input = prepare_input(data);
        assert_eq!(part1(&input), 142);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/01.test_part2.txt");
        let input = prepare_input(data);
        assert_eq!(part2(&input), 281);
    }
}