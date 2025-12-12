use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufReader, prelude::*},
    iter::FromIterator,
};
use priority_queue::PriorityQueue;

const FILENAME: &str = "input.txt";

fn main() {
    let lines = read_file(FILENAME);
    let machines = lines.iter().map(parse_to_machine).collect::<Vec<Machine>>();
    println!("Part 1 = {}", part_1(&machines));
    println!("Part 2 = {}", part_2(&machines));
}

fn part_1(machines: &Vec<Machine>) -> i64 {
    machines
        .iter()
        .map(|m| find_min_button_presses_for_lights(m))
        .sum()
}

fn part_2(machines: &Vec<Machine>) -> i64 {
    machines
        .iter()
        .map(|m| find_min_button_presses_for_joltages(m))
        .sum()
}

#[derive(Debug)]
struct Machine {
    light_target: u32,
    buttons: Vec<Button>,
    joltage_target: Vec<i64>,
}

#[derive(Debug)]
struct Button {
    toggle_indices: Vec<usize>,
}

fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse_to_machine(line: &String) -> Machine {
    let mut light_target: u32 = 0;
    let mut joltage_target: Vec<i64> = vec![];
    let mut buttons: Vec<Button> = vec![];
    for component in line.split(" ") {
        if component.starts_with("[") {
            light_target = parse_lights(component);
        } else if component.starts_with("{") {
            joltage_target = parse_joltages(component);
        } else if component.starts_with("(") {
            buttons.push(parse_button(component));
        }
    }
    buttons.sort_by_key(|button| button.toggle_indices.len());
    Machine {
        light_target,
        joltage_target,
        buttons,
    }
}

fn parse_lights(lights: &str) -> u32 {
    represent_as_binary_num(
        &lights[1..lights.len() - 1]
            .chars()
            .map(|c| c == '#')
            .collect(),
    )
}

fn represent_as_binary_num(vals: &Vec<bool>) -> u32 {
    let mut result = 0;
    for &b in vals.iter().rev() {
        result <<= 1;
        if b {
            result |= 1;
        }
    }
    result
}

fn parse_button(button: &str) -> Button {
    Button {
        toggle_indices: button[1..button.len() - 1]
            .split(",")
            .map(|num| num.parse::<usize>().unwrap())
            .collect(),
    }
}

fn parse_joltages(joltages: &str) -> Vec<i64> {
    joltages[1..joltages.len() - 1]
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect()
}

fn find_min_button_presses_for_lights(machine: &Machine) -> i64 {
    count_presses_to_target_for_lights(machine, vec![0], 0, HashSet::new())
}

fn count_presses_to_target_for_lights(
    machine: &Machine,
    curr_light_options: Vec<u32>,
    depth: i64,
    mut seen: HashSet<u32>,
) -> i64 {
    let target = machine.light_target;
    let mut new_light_options: Vec<u32> = vec![];
    for lights in &curr_light_options {
        if *lights == target {
            return depth;
        }
        for button in &machine.buttons {
            let new_lights = press_button_on_lights(*lights, button);
            if !seen.contains(&new_lights) {
                new_light_options.push(new_lights);
                seen.insert(new_lights);
            }
        }
    }
    if new_light_options == curr_light_options {
        panic!("Calling recursively with the same params, you've likely made a mistake :(");
    }
    count_presses_to_target_for_lights(machine, new_light_options, depth + 1, seen)
}

fn press_button_on_lights(lights: u32, button: &Button) -> u32 {
    let mut new_lights = lights;
    for &i in &button.toggle_indices {
        let mask = 1 << i;
        new_lights ^= mask;
    }
    new_lights
}

fn find_min_button_presses_for_joltages(machine: &Machine) -> i64 {
    let unresolveds: Vec<Unresolved> = machine
        .joltage_target
        .iter()
        .enumerate()
        .map(|(index, target)|
            Unresolved {
                variables: machine
                    .buttons
                    .iter()
                    .enumerate()
                    .filter(|(_, button)| button.toggle_indices.contains(&index))
                    .map(|(i, _)| i)
                    .collect::<Vec<usize>>(),
                sum: *target
            }
        )
        .collect();
    // count_presses_to_target_for_joltages(&machine)
    simplify_unresolveds(unresolveds)
}

fn count_presses_to_target_for_joltages(
    machine: &Machine,
) -> i64 {
    let mut num_iterations = 0;
    let mut queue = PriorityQueue::new();
    let mut seen = HashSet::new();
    let init_state = vec![0; machine.joltage_target.len()];
    seen.insert(init_state.clone());
    queue.push((init_state, 0), 0);
    let target = &machine.joltage_target;
    while queue.len() != 0 {
        num_iterations += 1;
        if num_iterations >= 10000000 {
            panic!("Too many iterations");
        }
        let Some(((curr_state, curr_steps), _)) = queue.pop() else {
            panic!("no items left in the queue")
        };
        if curr_state == *target {
            println!("found in {num_iterations} iterations");
            return curr_steps;
        }
        let largest_unfulfilled_target_index = target
                .iter()
                .enumerate()
                .filter(|(i, target_v)| **target_v > curr_state[*i])
                .min_by_key(|(i, target_v)| **target_v - curr_state[*i])
                .map(|(i, _)| i)
                .unwrap();
        for button in &machine.buttons {
            if button.toggle_indices.contains(&largest_unfulfilled_target_index) {
                let new_joltages = press_button_on_joltages(&curr_state, button);
                if could_be_valid_joltage_option(&new_joltages, target) {
                    if !seen.contains(&new_joltages) {
                        queue.push((new_joltages.clone(), curr_steps + 1), new_joltages.iter().sum());
                        seen.insert(new_joltages);
                    }
                }
            }
        }
    }
    panic!("Unsolvable machine");
}

fn press_button_on_joltages(joltages: &Vec<i64>, button: &Button) -> Vec<i64> {
    let mut new_joltages = joltages.clone();
    for &i in &button.toggle_indices {
        new_joltages[i] += 1;
    }
    new_joltages
}

fn could_be_valid_joltage_option(new_joltages: &Vec<i64>, target: &Vec<i64>) -> bool {
    for i in 0..new_joltages.len() {
        if new_joltages[i] > target[i] {
            return false;
        }
    }
    true
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Unresolved {
    variables: Vec<usize>,
    sum: i64,
}

impl Unresolved {
    fn diff(&self, other: &Unresolved) -> Self {
        let var_set: HashSet<usize> = HashSet::from_iter(self.variables.clone());
        let other_var_set: HashSet<usize> = HashSet::from_iter(other.variables.clone());
        Unresolved {
            variables: var_set.difference(&other_var_set).map(|v| *v).collect(),
            sum: self.sum - other.sum,
        }
    }
}

impl Clone for Unresolved {
    fn clone(&self) -> Self {
        Unresolved {
            variables: self.variables.clone(),
            sum: self.sum,
        }
    }
}

fn simplify(u1: &Unresolved, u2: &Unresolved) -> Option<(Unresolved, Unresolved)> {
    if u1.variables.len() == u2.variables.len() {
        return None;
    }
    let (longer_eq, shorter_eq) = if u1.variables.len() > u2.variables.len() {
        (u1, u2)
    } else {
        (u2, u1)
    };
    let shorter_var_set: HashSet<usize> = HashSet::from_iter(shorter_eq.variables.clone());
    let longer_var_set: HashSet<usize> = HashSet::from_iter(longer_eq.variables.clone());
    if shorter_var_set.is_subset(&longer_var_set) {
        let new_longer_eq = longer_eq.diff(&shorter_eq);
        Some((new_longer_eq, shorter_eq.clone()))
    } else {
        None
    }
}

fn simplify_unresolveds(mut unresolveds: Vec<Unresolved>) -> i64 {
    let mut optimized_any = true;
    while optimized_any {
        unresolveds.sort_by_key(|u| u.variables.len());
        unresolveds.reverse();
        optimized_any = false;
        let mut results: HashSet<Unresolved> = HashSet::from_iter(unresolveds.clone());
        'outer_loop: for i in 0..unresolveds.len() {
            for j in i + 1..unresolveds.len() {
                if let Some((optimized_1, _optimized_2)) =
                    simplify(&unresolveds[i], &unresolveds[j])
                {
                    // println!("Found an optimization! {:?} {:?} to {:?} {:?}", &unresolveds[i], &unresolveds[j], optimized_1, optimized_2);
                    results.insert(optimized_1);
                    results.remove(&unresolveds[i]);
                    optimized_any = true;
                    break 'outer_loop;
                }
            }
        }
        unresolveds = results.into_iter().collect();
    }
    let accumulated_sum: i64 = unresolveds
        .iter()
        .filter(|v| v.variables.len() == 1)
        .map(|v| v.sum)
        .sum();
    let filtered_unresolveds: Vec<Unresolved> = unresolveds
        .into_iter()
        .filter(|v| v.variables.len() != 1)
        .collect();
    let mut joltage_target: Vec<i64> = vec![];
    let mut table: HashMap<usize, Vec<usize>> = HashMap::new();
    for (index, optimized) in filtered_unresolveds.iter().enumerate() {
        joltage_target.push(optimized.sum);
        for v in &optimized.variables {
            table
                .entry(*v)
                .and_modify(|v| v.push(index))
                .or_insert(vec![index]);
        }
    }
    let buttons: Vec<Button> = table
        .values()
        .map(|toggle_indices| Button {
            toggle_indices: toggle_indices.clone(),
        })
        .collect();
    let machine = Machine {
        light_target: 0,
        buttons,
        joltage_target,
    };
    count_presses_to_target_for_joltages(&machine) + accumulated_sum
}
