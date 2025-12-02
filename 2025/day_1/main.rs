use std::{
    fs::File,
    io::{prelude::*, BufReader},
};


const FILENAME: &str = "input.txt";
const INIT_POS: i32 = 50;
const NUM_POS: i32 = 100;

fn main() {
    println!("part_1={}", part_1());
    println!("part_2={}", part_2());
}

fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn part_1() -> i32 {
    let file_lines = read_file(FILENAME);
    let lock_moves = parse_moves(file_lines);
    count_zeros_landed_on(lock_moves)
}

fn part_2() -> i32 {
    let file_lines = read_file(FILENAME);
    let lock_moves = parse_moves(file_lines);
    count_zeros_crossed(lock_moves)
}

enum Direction {
    Right,
    Left,
}
type Move = (Direction, i32);

fn parse_moves(lines: Vec<String>) -> Vec<Move> {
    lines.into_iter().map(|l| parse_move(&l)).collect()
}

fn parse_move(line: &str) -> Move {
    let first_char = &line[..1];
    let direction = match first_char {
        "R" => Direction::Right,
        "L" => Direction::Left,
        x => panic!("Invalid direction {}", x),
    };
    let num = line[1..].parse::<i32>().unwrap();
    (direction, num)
}

fn count_zeros_landed_on(moves: Vec<Move>) -> i32 {
    let mut pos = INIT_POS;
    let mut zeros = 0;
    for lock_move in moves {
        pos = make_move(pos, &lock_move);
        pos = wrap_pos(pos);
        if pos == 0 {
            zeros += 1;
        }
    }
    zeros
}

fn make_move(pos: i32, lock_move: &Move) -> i32 {
    match lock_move {
        (Direction::Right, num) => pos + num,
        (Direction::Left, num) => pos - num,
    }
}

fn wrap_pos(pos: i32) -> i32 {
    let wrapped_pos = pos % NUM_POS;
    if wrapped_pos < 0 { wrapped_pos + NUM_POS } else { wrapped_pos }
}

fn count_zeros_crossed(moves: Vec<Move>) -> i32 {
    let mut pos = INIT_POS;
    let mut zeros_crossed = 0;
    for lock_move in moves {
        let new_pos = make_move(pos, &lock_move);
        zeros_crossed += count_zeros_between(pos, new_pos);
        pos = wrap_pos(new_pos);
    }
    zeros_crossed
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
