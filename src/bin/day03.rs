use std::collections::HashSet;

use adventofcode2023::{gen_adjacent_indices_including_diagonal, load_input, pad_2d_char_vec, parse2d_char_vec};

fn prepare_input(s: String) -> Vec<Vec<char>> {
    pad_2d_char_vec(parse2d_char_vec(&s), '.')
}

fn part1(input: &Vec<Vec<char>>) -> u32 {
    let mut total_sum = 0;
    let mut found_partnumber_indices = HashSet::new();
    for row in 0..input.len() {
        for collumn in 0..input[0].len() {
            if input[row][collumn] != '.' && !input[row][collumn].is_ascii_digit() {
                // Gear Marker found
                let adjacent_indices = gen_adjacent_indices_including_diagonal(row, collumn);
                for (x, y) in adjacent_indices {
                    if input[x][y].is_ascii_digit() {
                        // part number found
                        // find start of partnumber
                        let mut start = y;
                        while input[x][start].is_ascii_digit() {
                            start -= 1;
                        }
                        start += 1;
                        // find end of partnumber
                        let mut end = y;
                        while input[x][end].is_ascii_digit() {
                            end += 1;
                        }
                        // extract part number & deduplication
                        if !found_partnumber_indices.contains(&(x, start)) {
                            found_partnumber_indices.insert((x, start));
                            let number_str: String = input[x][start..end].iter().collect();
                            // println!("Partnumber: {}", number_chars);
                            let partnumber = number_str.parse::<u32>().unwrap();
                            total_sum += partnumber;
                        }
                    }
                }
            }
        }
    }
    total_sum
}

fn part2(input: &Vec<Vec<char>>) -> u32 {
    let mut total_sum = 0;
    let mut found_partnumber_indices = HashSet::new();
    for row in 0..input.len() {
        for collumn in 0..input[0].len() {
            if input[row][collumn] == '*' {
                // Gear Marker found
                let adjacent_indices = gen_adjacent_indices_including_diagonal(row, collumn);
                let mut gear_ratios = Vec::new();
                for (x, y) in adjacent_indices {
                    if input[x][y].is_ascii_digit() {
                        // part number found
                        // find start of partnumber
                        let mut start = y;
                        while input[x][start].is_ascii_digit() {
                            start -= 1;
                        }
                        start += 1;
                        // find end of partnumber
                        let mut end = y;
                        while input[x][end].is_ascii_digit() {
                            end += 1;
                        }
                        // extract part number & deduplication
                        if !found_partnumber_indices.contains(&(x, start)) {
                            found_partnumber_indices.insert((x, start));
                            let number_str: String = input[x][start..end].iter().collect();
                            // println!("Partnumber: {}", number_chars);
                            let partnumber = number_str.parse::<u32>().unwrap();
                            gear_ratios.push(partnumber)
                        }
                    }
                }
                if gear_ratios.len() == 2 {
                    // it is in fact a gear
                    total_sum += gear_ratios[0] * gear_ratios[1]
                }
            }
        }
    }
    total_sum
}

fn main() {
    let data = load_input("input/03.txt");
    let input = prepare_input(data);
    println!("First task: {:?}", part1(&input));
    println!("Second task: {:?}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/03.test.txt");
        let input = prepare_input(data);
        assert_eq!(part1(&input), 4361);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/03.test.txt");
        let input = prepare_input(data);
        assert_eq!(part2(&input), 467835);
    }
}