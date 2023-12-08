use std::{fs::File, io::Read};
use crate::days::day01::*;
use crate::days::day02::*;
pub mod days;

fn read_input(filepath: &str) -> String {
    let mut fileinfo = match File::open(filepath) {
        Ok(f) => f,
        Err(_) => panic!("Can't open File"),
    };
    let mut contents = String::new();
    fileinfo.read_to_string(&mut contents).unwrap();
    contents.replace("\r\n", "\n").trim_end().to_string()
}

fn main() {
    println!(
        "Day 1 Test: {}",
        day1_part1(read_input("data/day01_input1_test.txt").as_str())
    );
    println!(
        "Day 1 Part 1: {}",
        day1_part1(read_input("data/day01_input1.txt").as_str())
    );
    println!(
        "Day 1 Part 2: {}",
        day1_part2(read_input("data/day01_input2_test.txt").as_str())
    );
    println!(
        "Day 1 Part 2: {}",
        day1_part2(read_input("data/day01_input2.txt").as_str())
    );
    println!(
        "Day 2 Test: {}",
        day2_part1(read_input("data/day02_input1_test.txt").as_str())
    );
    println!(
        "Day 2 Part 1: {}",
        day2_part1(read_input("data/day02_input1.txt").as_str())
    );
    println!(
        "Day 2 Part 2: {}",
        day2_part2(read_input("data/day02_input1_test.txt").as_str())
    );
    println!(
        "Day 2 Part 2: {}",
        day2_part2(read_input("data/day02_input1.txt").as_str())
    );
}