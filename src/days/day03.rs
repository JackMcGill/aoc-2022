use std::fs;
use std::str::Lines;

const PRIORITY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn main() {
    println!("Day three");

    let data: String = fs::read_to_string("src/inputs/day03/input.txt").expect("Could not read in file");
    let lines: Lines = data.lines();

    println!("Part 1: {}", part_one(lines.clone()));
    println!("Part 2: {}", part_two(lines));
    println!();
}

fn part_one(lines: Lines) -> usize {
    let mut count = 0;
    for line in lines {
        let first: &str = &line[..line.len() / 2];
        let second: &str = &line[line.len() / 2..];
        count += PRIORITY.find(get_common_between_two(first, second).unwrap()).unwrap() + 1;
    }
    count
}

fn part_two(lines: Lines) -> usize {
    let lines: Vec<&str> = lines.collect::<Vec<_>>(); // Override parameter lines as a Vec
    let mut count: usize = 0;
    for line in lines.chunks(3) {
        count += PRIORITY.find(get_common_between_three(line)).unwrap() + 1;
    }
    count
}

pub fn get_common_between_two(first: &str, second: &str) -> Option<char> {
    second.chars().find(|&char| first.contains(char))
}

pub fn get_common_between_three(lines: &[&str]) -> char {
    let mut common: Vec<char> = Vec::new();
    for char in lines[0].chars() {
        if lines[1].contains(char) {
            common.push(char);
        }
    }
    get_common_between_two(&String::from_iter(common), lines[2]).unwrap()
}