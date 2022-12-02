use std::fs;
use std::str::Lines;

pub fn main() {
    println!("Day 1");
    let data: String = fs::read_to_string("src/inputs/day01/input.txt").expect("Couldn't read in file");
    let lines: Lines = data.lines();

    let mut cals: i32 = 0;
    let mut elves: Vec<i32> = Vec::new();

    for line in lines {
        if !line.is_empty() {
            cals += to_i32(line);
        } else {
            elves.push(cals);
            cals = 0;
        }
    }

    let max: &i32 = elves.iter().max().unwrap();
    println!("Part one: {max}"); // PART ONE

    elves.sort();
    elves.reverse();

    let x: i32 = elves[0] + elves[1] + elves[2];
    println!("Part two: {x}"); // PART TWO
    println!();
}

fn to_i32(x: &str) -> i32 {
    x.parse::<i32>().unwrap()
}