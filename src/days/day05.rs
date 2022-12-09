use std::fs;
use std::str::Lines;

pub fn main() {
    println!("Day 05");

    let data: String = fs::read_to_string("src/inputs/day05/example.txt").expect("Could not read in file");
    let lines: Lines = data.lines();

    let mut stacks_as_lines: Vec<&str> = Vec::new();
    let mut instructions_as_lines: Vec<&str> = Vec::new();
    let mut instructions_found: bool = false;
    for line in lines {
        if line.is_empty() {
            instructions_found = true;
            continue;
        }
        if !instructions_found {
            stacks_as_lines.push(line)
        } else {
            instructions_as_lines.push(line)
        }
    }

    let stacks: Vec<Vec<char>> = get_crates(stacks_as_lines);

    part_one(stacks, instructions_as_lines);

    println!();
}

fn part_one(mut stacks: Vec<Vec<char>>, instructions_as_lines: Vec<&str>) {
    // let mut reversed: Vec<Vec<char>> = Vec::new();
    for row in &mut stacks {
        row.reverse()
    }
    println!("before");
    for g in &stacks[0] {
        println!(":{g}");
    }
    for i in instructions_as_lines {
        let (amount, from, to): (i32, i32, i32) = parse_instructions(i);

        println!("move {amount} from {from} to {to}");

        for _ in 0..amount {
            // println!("Moving {amount} times");
            let pop = stacks[from as usize - 1].pop().unwrap();
            stacks[to as usize - 1].push( pop);
        }
        for g in &stacks[0] {
            println!(":{g}");
        }
        let f = stacks[0].pop().unwrap();
        println!("f = {f} (should be C)");
    }
}

fn parse_instructions(i: &str) -> (i32, i32, i32) {
    let replaced = i.replace("move ", "").replace("from ", "").replace("to ", "");
    let vec = replaced.split(' ').collect::<Vec<&str>>();

    let amount = to_i32(vec[0]);
    let from = to_i32(vec[1]);
    let to = to_i32(vec[2]);

    (amount, from, to)
}

fn get_crates(mut crate_lines: Vec<&str>) -> Vec<Vec<char>> {
    let last_char: char = crate_lines.pop().unwrap().chars().last().unwrap();
    let length = last_char.to_digit(10).unwrap();

    let mut crates = vec![Vec::<char>::new(); length as usize];

    for line in crate_lines {
        let line = line.replace("   ", " _ ").replace(['[', ']', ' '], "");
        for (i, c) in line.chars().enumerate() {
            if c != '_' {
                crates[i].push(c);
            }
        }
    }
    crates
}

fn to_i32(x: &str) -> i32 {
    x.parse::<i32>().unwrap()
}