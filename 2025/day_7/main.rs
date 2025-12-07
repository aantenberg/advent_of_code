use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::{HashMap}
};

const FILENAME: &str = "input.txt";

fn main() {
    let lines = read_file(FILENAME);
    let Solution {num_splits, num_timelines} = solve(&lines);
    println!("Part 1 = {}", num_splits);
    println!("Part 2 = {}", num_timelines);
}

fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


struct Solution {
    num_splits: i64,
    num_timelines: i64,
}

fn solve(lines: &Vec<String>) -> Solution {
    let start_index: usize = lines[0].chars().position(|c| c == 'S').unwrap();
    let mut beam_index_to_num_paths = HashMap::from([(start_index, 1)]);
    let mut num_splits = 0;
    for line in &lines[1..] {
        let chars: Vec<char> = line.chars().collect();
        let mut new_beam_indices = HashMap::new();
        for (index, num_paths) in beam_index_to_num_paths.into_iter() {
            match chars[index] {
                '.' => {add_num_paths_to_index(&mut new_beam_indices, index, num_paths);}
                '^' => {
                    add_num_paths_to_index(&mut new_beam_indices, index - 1, num_paths);
                    add_num_paths_to_index(&mut new_beam_indices, index + 1, num_paths);
                    num_splits += 1;
                },
                c => panic!("Invalid char {}", c)
            }
        }
        beam_index_to_num_paths = new_beam_indices;
    }
    let num_timelines = beam_index_to_num_paths.values().sum();
    Solution {
        num_splits,
        num_timelines
    }

}

fn add_num_paths_to_index(map: &mut HashMap<usize, i64>, index_to_modify: usize, num_paths: i64) {
    map.entry(index_to_modify).and_modify(|existing_paths| *existing_paths += num_paths).or_insert(num_paths);
}
