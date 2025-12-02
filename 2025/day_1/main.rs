use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

const FILENAME: &str = "input.txt";
const INIT_POS: i32 = 50;
const NUM_POS: i32 = 100;

fn main() {
    let file_lines = read_file(FILENAME);
    let parsed_moves = parse_moves(file_lines);
    let solution = count_zeros(parsed_moves);
    print_solution(solution);
}

fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

enum Move {
    Right(i32),
    Left(i32),
}

fn parse_moves(lines: Vec<String>) -> Vec<Move> {
    lines.into_iter().map(|l| parse_move(&l)).collect()
}

fn parse_move(line: &str) -> Move {
    let first_char = &line[..1];
    let num = line[1..].parse::<i32>().unwrap();
    match first_char {
        "R" => Move::Right(num),
        "L" => Move::Left(num),
        x => panic!("Invalid direction {}", x),
    }
}


struct Solution {
    zeros_landed_on: i32,
    zeros_crossed: i32,
}

fn count_zeros(moves: Vec<Move>) -> Solution {
    let mut pos = INIT_POS;
    let mut zeros_landed_on = 0;
    let mut zeros_crossed = 0;
    for lock_move in moves {
        let new_pos = make_move(pos, &lock_move);
        zeros_crossed += count_zeros_between(pos, new_pos);
        pos = wrap_pos(new_pos);
        if pos == 0 {
            zeros_landed_on += 1;
        }
    }
    Solution {
        zeros_landed_on,
        zeros_crossed
    }
}

fn make_move(pos: i32, lock_move: &Move) -> i32 {
    match lock_move {
        Move::Right(num) => pos + num,
        Move::Left(num) => pos - num,
    }
}

fn wrap_pos(pos: i32) -> i32 {
    let wrapped_pos = pos % NUM_POS;
    if wrapped_pos < 0 { wrapped_pos + NUM_POS } else { wrapped_pos }
}

fn count_zeros_between(old_pos: i32, new_pos: i32) -> i32 {
    // we know old_pos is in [0, NUM_POS)
    let has_not_crossed_zero = new_pos < old_pos && new_pos > 0;
    if has_not_crossed_zero {
        return 0;
    }
    let crosses_zero_downward = old_pos > 0 && new_pos <= 0;
    new_pos.abs() / NUM_POS + if crosses_zero_downward { 1 } else { 0 }
}

fn print_solution(solution: Solution) {
    let Solution { zeros_landed_on, zeros_crossed } = solution;
    println!("part_1={}", zeros_landed_on);
    println!("part_2={}", zeros_crossed);
}
