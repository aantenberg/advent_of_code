use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

const FILENAME: &str = "input.txt";

fn main() {
    let lines = read_file(FILENAME);
    let rolls: Vec<Vec<i32>> = parse_input(&lines);
    println!("Part 1 = {}", part_1(&rolls));
    println!("Part 2 = {}", part_2(&rolls));
}

fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn part_1(rolls: &Vec<Vec<i32>>) -> i32 {
    let (_rolls, count) = remove_valid_rolls(rolls);
    count
}

fn part_2(rolls: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    let (mut new_rolls, mut removed_count) = remove_valid_rolls(rolls);
    while removed_count > 0 {
        sum += removed_count;
        (new_rolls, removed_count) = remove_valid_rolls(&new_rolls);
    }
    sum
}

fn parse_input(lines: &[String]) -> Vec<Vec<i32>> {
    let mut rolls = vec![];
    for line in lines {
        let mut roll = vec![];
        for c in line.chars() {
            let i: i32 = match c {
                '.' => 0,
                '@' => 1,
                _ => panic!("Invalid character {}", c),
            };
            roll.push(i);
        }
        rolls.push(roll);
    }
    rolls
}

fn remove_valid_rolls(rolls: &Vec<Vec<i32>>) -> (Vec<Vec<i32>>, i32) {
    let mut removed_count = 0;
    let mut new_rolls = vec![vec![0; rolls[0].len()]; rolls.len()];
    for row in 0..rolls.len() {
        for col in 0..rolls[row].len() {
            if is_valid_roll(rolls, row, col) {
                removed_count += 1;
                new_rolls[row][col] = 0;
            } else {
                new_rolls[row][col] = rolls[row][col];
            }
        }
    }
    (new_rolls, removed_count)
}

fn is_valid_roll(rolls: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    if rolls[row][col] == 0 {
        return false;
    }
    let min_row = if row == 0 { 0 } else { row - 1 };
    let max_row = if row == rolls.len() - 1 { row } else { row + 1 };
    let min_col = if col == 0 { 0 } else { col - 1 };
    let max_col = if col == rolls[0].len() - 1 { col } else { col + 1 };
    let mut num_rolls_in_neighborhood = 0;
    for neighbor_row in min_row..=max_row {
        for neighbor_col in min_col..=max_col {
            num_rolls_in_neighborhood += rolls[neighbor_row][neighbor_col];
        }
    }
    let num_neighbor_rolls = num_rolls_in_neighborhood - rolls[row][col];
    (num_neighbor_rolls) < 4
}
