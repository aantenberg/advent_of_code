use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

const FILENAME: &str = "input.txt";

fn main() {
    let lines = read_file(FILENAME);
    let (ranges, values) = parse_input(&lines);
    let part_1 = count_values_in_ranges(&ranges, &values);
    println!("Part 1: {}", part_1);
    let part_2 = count_all_distinct_values_in_ranges(&ranges);
    println!("Part 2: {}", part_2)
}

fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

type Range = (i64, i64);

fn parse_input(lines: &[String]) -> (Vec<Range>, Vec<i64>) {
    let mut ranges = vec![];
    let mut values = vec![];
    let mut is_in_range_range = true;
    for line in lines {
        if line.is_empty() {
            is_in_range_range = false;
            continue;
        }
        if is_in_range_range {
            ranges.push(parse_range(line));
        } else {
            values.push(parse_value(line));
        }
    }
    ranges.sort_by(|range_1, range_2| range_1.0.cmp(&range_2.0));
    ranges = merge_sorted_ranges(&ranges);
    (ranges, values)
}

fn merge_sorted_ranges(sorted_ranges: &[Range]) -> Vec<Range> {
    let mut result = vec![];
    let mut range_builder: Range = sorted_ranges[0];
    for &range in sorted_ranges {
        let (builder_min, builder_max) = range_builder;
        let (curr_min, curr_max) = range;
        if curr_min <= builder_max {
            range_builder = (builder_min, curr_max.max(builder_max));
        } else {
            result.push(range_builder);
            range_builder = range
        }
    }
    result.push(range_builder);
    result
}

fn parse_range(line: &str) -> Range {
    let parts = line.split_once('-').unwrap();
    (parse_value(parts.0), parse_value(parts.1))
}

fn parse_value(line: &str) -> i64 {
    line.parse().unwrap()
}

fn count_values_in_ranges(ranges: &[Range], values: &[i64]) -> usize {
    values.iter().filter(|v| is_in_some_range(ranges, **v)).count()
}

fn is_in_some_range(ranges: &[Range], value: i64) -> bool {
    ranges.iter().any(|r| is_in_range(r, value))
}

fn is_in_range(range: &Range, value: i64) -> bool {
    let &(min, max) = range;
    value >= min && value <= max
}

fn count_all_distinct_values_in_ranges(ranges: &[Range]) -> i64 {
    ranges.iter().map(|(min, max)| max - min + 1).sum()
}
