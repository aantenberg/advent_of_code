use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

const FILENAME: &str = "input.txt";

fn main() {
    let banks = read_file(FILENAME);
    println!("Part 1 = {}", part_1(&banks));
    println!("Part 2 = {}", part_2(&banks));
}

fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn part_1(banks: &Vec<String>) -> i64 {
    let num_batteries = 3;
    banks.iter()
        .map(|bank| get_max_joltage(bank, num_batteries))
        .sum()
}

fn part_2(banks: &Vec<String>) -> i64 {
    let num_batteries = 12;
    banks.iter()
        .map(|bank| get_max_joltage(bank, num_batteries))
        .sum()
}

fn get_max_joltage(bank: &str, num_batteries: usize) -> i64 {
    let chars = bank.chars();
    if num_batteries == 1 {
        return chars.max().unwrap().to_digit(10).unwrap() as i64;
    }
    let chars: Vec<char> = chars.collect();
    let (mut max_joltage, mut max_joltage_index) = ('0', 0);
    for i in 0..=chars.len() - num_batteries {
        if chars[i] > max_joltage {
            (max_joltage, max_joltage_index) = (chars[i], i);
        }
    }
    let recursive_max_joltage = get_max_joltage(&bank[max_joltage_index + 1..], num_batteries - 1);
    format!("{}{}", max_joltage, recursive_max_joltage).parse::<i64>().unwrap()
}
