use std::collections::HashSet;
use std::fs;
use std::str::{Lines, Split};

pub fn main() {
    println!("Day 4");

    let data: String =
        fs::read_to_string("src/inputs/day04/input.txt").expect("Could not read in file");
    let lines: Lines = data.lines();

    part_one(lines.clone());
    part_two(lines);
    println!();
}

fn part_one(lines: Lines) {
    let mut count: i32 = 0;
    for line in lines {
        let (elf1_set, elf2_set): (HashSet<u32>, HashSet<u32>) = get_elf_sets(line);

        if elf1_set.is_subset(&elf2_set) || elf2_set.is_subset(&elf1_set) {
            count += 1;
        }
    }
    println!("Part one: {count}");
}

fn part_two(lines: Lines) {
    let mut count: i32 = 0;
    for line in lines {
        let (elf1_set, elf2_set): (HashSet<u32>, HashSet<u32>) = get_elf_sets(line);
        let elf1_intersect: Vec<&u32> = elf1_set.intersection(&elf2_set).collect::<Vec<_>>();
        let elf2_intersect: Vec<&u32> = elf2_set.intersection(&elf1_set).collect::<Vec<_>>();

        if !elf1_intersect.is_empty() || !elf2_intersect.is_empty() {
            count += 1
        };
    }
    println!("Part two: {count}");
}

fn get_elf_sets(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let split: Split<char> = line.split(',');
    let vec: Vec<&str> = split.collect();

    let elf1_set: HashSet<u32> = create_set(vec.first().unwrap());
    let elf2_set: HashSet<u32> = create_set(vec.get(1).unwrap());
    (elf1_set, elf2_set)
}

fn create_set(elf: &str) -> HashSet<u32> {
    let section: Vec<&str> = elf.split('-').collect();
    let start: u32 = section.first().unwrap().parse().unwrap();
    let end: u32 = section.get(1).unwrap().parse().unwrap();

    let set: HashSet<u32> = (start..=end).collect();
    set
}
