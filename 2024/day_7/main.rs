use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

const FILENAME: &str = "input.txt";

fn main() {
    let file_lines = read_file(FILENAME);
    let parsed_equations = parse_equations(file_lines);
    println!("Solution 1: {}", part_1(&parsed_equations));
    println!("Solution 2: {}", part_2(&parsed_equations));
}

fn part_1(equations: &Vec<Equation>) -> i64 {
    let available_operations = vec![Operation::Addition, Operation::Multiplication];
    sum_satisfiable_equations(equations, &available_operations)
}

fn part_2(equations: &Vec<Equation>) -> i64 {
    let available_operations = vec![Operation::Addition, Operation::Multiplication, Operation::Concatenation];
    sum_satisfiable_equations(equations, &available_operations)
}

fn parse_equations(lines: Vec<String>) -> Vec<Equation> {
    lines.iter().map(|l| parse_equation(l)).collect()
}

fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn sum_satisfiable_equations(equations: &Vec<Equation>, available_operations: &Vec<Operation>) -> i64 {
    equations.iter()
        .filter(|e| e.is_satisfiable(&available_operations))
        .map(|e| e.result)
        .sum()
}

struct Equation {
    result: i64,
    operands: Vec<i64>,
}

enum Operation {
    Addition,
    Multiplication,
    Concatenation,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operation::Addition => write!(f, "+"),
            Operation::Multiplication => write!(f, "*"),
            Operation::Concatenation => write!(f, "||"),
        }
    }
}

fn parse_equation(line: &String) -> Equation {
    let parts : Vec<&str> = line.split(":").collect();
    let result = parts[0].parse::<i64>().unwrap();
    let operands = parts[1].trim().split(" ").map(|o| o.parse::<i64>().unwrap()).collect();
    Equation { result, operands }
}

impl Equation {
    fn is_satisfiable(&self, available_operations: &Vec<Operation>) -> bool {
        let result = self.result;
        let operands = &self.operands;
        if operands.len() == 1 {
            if operands[0] == result {
                // println!("{}", operands[0]);
                return true;
            }
            return false;
        }
        let (&last_operand, unused_operands) = operands.split_last().unwrap();
        for operation in available_operations {
            if operation.may_apply(result, last_operand) {
                let remaining_equation = Equation {
                    result: operation.apply_inverse(result, last_operand),
                    operands: unused_operands.to_vec(),
                };
                if remaining_equation.is_satisfiable(available_operations) {
                    // println!(" {} {} = {}", operation, last_operand, result);
                    return true;
                }
            }
        }
        return false;
    }
}

impl Operation {
    fn may_apply(&self, result: i64, operand: i64) -> bool {
        match self {
            Operation::Addition => result - operand >= 0,
            Operation::Multiplication => result % operand == 0,
            Operation::Concatenation => result.to_string().ends_with(&operand.to_string()),
        }
    }

    fn apply_inverse(&self, result: i64, operand: i64) -> i64 {
        match self {
            Operation::Addition => result - operand,
            Operation::Multiplication => result / operand,
            Operation::Concatenation => result / (10_i64.pow(operand.to_string().len() as u32)), // Take off the last n digits of the result where n is the length of the operand
        }
    }
}
