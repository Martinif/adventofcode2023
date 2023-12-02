use std::str::FromStr;

use adventofcode2023::load_input;

const RED_MAX: u32 = 12;
const BLUE_MAX: u32 = 14;
const GREEN_MAX: u32 = 13;

struct Game {
    id: u32,
    selections: Vec<Selection>,
}

struct Selection {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug)]
struct ParseGameError;

impl FromStr for Selection {
    type Err = ParseGameError;

    // example: 1 red, 2 green, 6 blue
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let entries: Vec<(&str, &str)> = s.split(", ")
            .map(|count_description| count_description.split_once(' ').unwrap())
            .collect();
        for e in entries {
            match e.1 {
                "red" => red = e.0.parse().map_err(|_| ParseGameError)?,
                "green" => green = e.0.parse().map_err(|_| ParseGameError)?,
                "blue" => blue = e.0.parse().map_err(|_| ParseGameError)?,
                _ => unreachable!()
            }
        }
        Ok(Selection { red, green, blue })
    }
}

impl FromStr for Game {
    type Err = ParseGameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id, selections_str) = s.split_once(": ").unwrap();
        let id: u32 = game_id.split_once(' ').unwrap().1.parse().map_err(|_| ParseGameError)?;
        let selections: Vec<Selection> = selections_str.split("; ")
            .map(|selection| selection.parse().unwrap())
            .collect();
        Ok(Game { id, selections })
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        let valid = self.selections.iter().all(|selection|
            selection.red <= RED_MAX && selection.green <= GREEN_MAX && selection.blue <= BLUE_MAX);
        return valid;
    }

    fn get_power(&self) -> u32 {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for selection in &self.selections {
            if selection.red > red { red = selection.red }
            if selection.green > green { green = selection.green }
            if selection.blue > blue { blue = selection.blue }
        }
        red * green * blue
    }
}

fn prepare_input(s: String) -> Vec<Game> {
    s.lines().map(|l| l.parse().unwrap()).collect()
}

fn part1(games: &Vec<Game>) -> u32 {
    games.iter().filter(|game| game.is_valid()).map(|game| game.id).sum()
}

fn part2(games: &Vec<Game>) -> u32 {
    games.iter().map(|game| game.get_power()).sum()
}

fn main() {
    let data = load_input("input/02.txt");
    let input = prepare_input(data);
    println!("First task: {:?}", part1(&input));
    println!("Second task: {:?}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/02.test.txt");
        let input = prepare_input(data);
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/02.test.txt");
        let input = prepare_input(data);
        assert_eq!(part2(&input), 2286);
    }
}