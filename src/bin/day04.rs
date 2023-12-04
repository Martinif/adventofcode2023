use std::collections::HashMap;
use std::str::FromStr;

use adventofcode2023::load_input;

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    elf_numbers: Vec<u32>,
}

impl Card {
    fn count_hits(&self) -> usize {
        self.elf_numbers.iter()
            .filter(|elf_number| self.winning_numbers.contains(elf_number))
            .count()
    }
}

impl FromStr for Card {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (winning_number_str, elf_numbers_str) = s.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winning_numbers = winning_number_str.split_whitespace().map(|n| n.parse().unwrap()).collect();
        let elf_numbers = elf_numbers_str.split_whitespace().map(|n| n.parse().unwrap()).collect();
        Ok(Card { winning_numbers, elf_numbers })
    }
}

fn prepare_input(s: String) -> Vec<Card> {
    s.lines().map(|l| l.parse().unwrap()).collect()
}

fn part1(input: &Vec<Card>) -> u32 {
    let mut total: u32 = 0;
    for card in input {
        let hits = card.count_hits();
        if hits > 0 {
            total += 2_u32.pow((hits - 1) as u32);
        }
    }
    total
}

fn part2(input: &Vec<Card>) -> u32 {
    let cards_len = input.len();
    let mut total_cards = 0;
    // track how many copies we have for each card
    let mut card_copies: HashMap<_, _> = HashMap::with_capacity(cards_len);
    for idx in 0..cards_len {
        card_copies.insert(idx, 1); // we have at least one copy of each card
    }

    for (idx, card) in input.iter().enumerate() {
        let count = card_copies.get(&idx).expect("Each card has at least one copy").clone();
        total_cards += count;
        let hits = card.count_hits();
        // insert duplicates
        for duplicates_idx in idx + 1..idx + hits + 1 {
            *card_copies.entry(duplicates_idx).or_insert(0) += count;
        }
    }
    total_cards
}

fn main() {
    let data = load_input("input/04.txt");
    let input = prepare_input(data);
    println!("First task: {:?}", part1(&input));
    println!("Second task: {:?}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/04.test.txt");
        let input = prepare_input(data);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/04.test.txt");
        let input = prepare_input(data);
        assert_eq!(part2(&input), 30);
    }
}