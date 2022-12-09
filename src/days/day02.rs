use std::fs;
use std::str::Lines;

pub fn main() {
    println!("Day 2");
    let data: String =
        fs::read_to_string("src/inputs/day02/input.txt").expect("Could not read in file");
    let lines: Lines = data.lines();

    part_one(lines.clone());
    part_two(lines);
    println!();
}

fn part_one(lines: Lines) {
    let mut score: i32 = 0;
    for line in lines {
        score += match line {
            "A X" => 1 + 3,
            "B X" => 1,
            "C X" => 1 + 6,
            "A Y" => 2 + 6,
            "B Y" => 2 + 3,
            "C Y" => 2,
            "A Z" => 3,
            "B Z" => 3 + 6,
            "C Z" => 3 + 3,
            _ => 0,
        };
    }
    println!("Part one: {}", score);
}

fn part_two(lines: Lines) {
    let mut score: i32 = 0;
    for line in lines {
        score += match line {
            "A X" => 3,
            "B X" => 1,
            "C X" => 2,
            "A Y" => 3 + 1,
            "B Y" => 3 + 2,
            "C Y" => 3 + 3,
            "A Z" => 6 + 2,
            "B Z" => 6 + 3,
            "C Z" => 6 + 1,
            _ => 0,
        };
    }
    println!("Part two: {}", score);
}
