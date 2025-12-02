use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

const FILENAME: &str = "input.txt";

type Range = (i64, i64);

fn main() {
    let lines = read_file(FILENAME);
    let ranges = parse_input(&lines[0]);
    println!("Part 1 = {}", part_1(&ranges));
    println!("Part 2 = {}", part_2(&ranges));
}

fn part_1(ranges: &Vec<Range>) -> i64 {
    sum_invalids_in_ranges(ranges, is_invalid_pt_1)
}

fn part_2(ranges: &Vec<Range>) -> i64 {
    sum_invalids_in_ranges(&ranges, is_invalid_pt_2)
}


fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse_input(line: &String) -> Vec<Range> {
    line.split(",").map(|s| parse_range(s)).collect()
}

fn parse_range(range: &str) -> Range {
    let range_vec = range.split("-").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    (range_vec[0], range_vec[1])
}

fn sum_invalids_in_ranges<F>(ranges: &Vec<Range>, invalidity_fn: F) -> i64 where F: Fn(i64) -> bool + Copy {
    ranges.iter().map(|r| sum_invalids_in_range(r, invalidity_fn)).sum()
}

fn sum_invalids_in_range<F>(range: &Range, invalidity_fn: F) -> i64 where F: Fn(i64) -> bool + Copy {
    let &(start, end) = range;
    let mut sum = 0;
    for i in start..=end {
        if invalidity_fn(i) {
            sum += i;
        }
    }
    sum
}

fn is_invalid_pt_1(num: i64) -> bool {
    let num_digits = num.ilog10() + 1;
    if num_digits % 2 != 0 {
        return false;
    }
    let half_digits = num_digits / 2;
    let first_n_digits = num / 10i64.pow(half_digits);
    let last_n_digits = num % 10i64.pow(half_digits);
    first_n_digits == last_n_digits
}

fn is_invalid_pt_2(num: i64) -> bool {
    let num_digits = (num.ilog10() + 1) as usize;
    let num_str = num.to_string();
    for substr_len in 1..=num_digits / 2 {
        if num_digits % substr_len == 0 {
            let first_piece: &str = &num_str[0..substr_len];
            if first_piece.repeat(num_digits / substr_len) == num_str {
                return true;
            }
        }
    }
    return false;
}
