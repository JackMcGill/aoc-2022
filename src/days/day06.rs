use std::collections::HashSet;
use std::fs;

pub fn main() {
    println!("Day 06");

    let data: Vec<char> = fs::read_to_string("src/inputs/day06/input.txt")
        .expect("Could not read in file")
        .chars()
        .collect();

    println!("Part one: {}", detect_start_of_packet(data.clone(), 4));
    println!("Part two: {}", detect_start_of_packet(data, 14));

    println!();
}

fn detect_start_of_packet(transmission: Vec<char>, marker: usize) -> i32 {
    let mut index: usize = marker;
    for window in transmission.windows(marker) {
        let mut unique_chars: HashSet<char> = HashSet::new();
        for char in window {
            unique_chars.insert(*char);
        }
        if unique_chars.len() == marker {
            break;
        }
        index += 1
    }
    index as i32
}
