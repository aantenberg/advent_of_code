use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

const FILENAME: &str = "input.txt";

fn main() {
    let lines = read_file(FILENAME);
    println!("Part 1 = {}", part_1(&lines));
    println!("Part 2 = {}", part_2(&lines));
}

fn part_1(lines: &Vec<String>) -> i64 {
    0
}

fn part_2(lines: &Vec<String>) -> i64 {
    0
}

fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
