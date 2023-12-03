use std::fs;
use std::path::Path;

pub fn load_input<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path).expect("Unable to read file")
}

pub fn parse2d_char_vec(s: &String) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect()
}

pub fn pad_2d_char_vec(mut v: Vec<Vec<char>>, padding: char) -> Vec<Vec<char>> {
    for row in &mut v {
        row.insert(0, padding);
        row.push(padding)
    }
    let line_padding = vec![padding; v[0].len()];
    v.insert(0, line_padding.clone());
    v.push(line_padding);
    v
}

pub fn gen_adjacent_indices_including_diagonal(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x - 1, y),
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
        (x, y + 1),
        (x - 1, y + 1),
    ]
}