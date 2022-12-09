use std::collections::vec_deque::VecDeque;
use std::fs;

pub fn main() {
    println!("Day 05");

    let data: String =
        fs::read_to_string("src/inputs/day05/input.txt").expect("Could not read in file");
    let lines = data.lines();

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
    let stacks: Vec<VecDeque<char>> = get_stacks(stacks_as_lines);

    part_one(stacks.clone(), instructions_as_lines.clone());
    part_two(stacks, instructions_as_lines);

    println!();
}

fn part_one(mut stacks: Vec<VecDeque<char>>, instructions_as_lines: Vec<&str>) {
    for i in instructions_as_lines {
        let (amount, from, to) = parse_instructions(i);
        for _ in 0..amount {
            let pop = stacks[from - 1].pop_front();
            stacks[to - 1].push_front(pop.unwrap());
        }
    }
    let mut tops: Vec<char> = Vec::new();
    for mut stack in stacks {
        match stack.pop_front() {
            Some(char) => tops.push(char),
            None => println!("no top"),
        };
    }
    print!("Part one: ");
    for t in tops {
        print!("{}", t);
    }
    println!();
}

fn part_two(mut stacks: Vec<VecDeque<char>>, instructions_as_lines: Vec<&str>) {
    for i in instructions_as_lines {
        let (amount, from, to) = parse_instructions(i);
        let mut drained = stacks[from - 1]
            .drain(..amount as usize)
            .collect::<VecDeque<_>>();
        for _ in 0..amount {
            let pop = drained.pop_back();
            stacks[to - 1].push_front(pop.unwrap());
        }
    }
    let mut tops: Vec<char> = Vec::new();
    for mut stack in stacks {
        match stack.pop_front() {
            Some(char) => tops.push(char),
            None => println!("no top"),
        };
    }
    print!("Part two: ");
    for t in tops {
        print!("{}", t);
    }
    println!();
}

fn parse_instructions(i: &str) -> (i32, usize, usize) {
    let replaced = i
        .replace("move ", "")
        .replace("from ", "")
        .replace("to ", "");
    let vec = replaced.split(' ').collect::<Vec<&str>>();

    let amount = vec[0].parse::<i32>().unwrap();
    let from = vec[1].parse::<usize>().unwrap();
    let to = vec[2].parse::<usize>().unwrap();

    (amount, from, to)
}

fn get_stacks(mut crate_lines: Vec<&str>) -> Vec<VecDeque<char>> {
    let last_char: char = crate_lines.pop().unwrap().trim().chars().last().unwrap();
    let length = last_char.to_digit(10).unwrap();

    let mut stacks = vec![VecDeque::<char>::new(); length as usize];

    for line in crate_lines {
        let line = ("  ".to_owned() + line).chars().collect::<Vec<char>>();
        for (i, chunk) in line.chunks(4).enumerate() {
            if chunk.len() == 4 && chunk[3] != ' ' {
                stacks[i].push_back(chunk[3]);
            };
        }
    }
    stacks
}
