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
    let expressions = parse_file(lines, parse_human_operands);
    sum_expression_results(&expressions)
}

fn part_2(lines: &Vec<String>) -> i64 {
    let expressions = parse_file(lines, parse_cephalopod_operands);
    sum_expression_results(&expressions)
}

fn sum_expression_results(expressions: &Vec<Expression>) -> i64 {
    expressions.iter().map(|e| e.evaluate()).sum()
}

fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(Debug)]
struct Expression {
    operands: Vec<i64>,
    operator: Operator
}


#[derive(Copy, Clone, Debug)]
enum Operator {
    PLUS,
    TIMES
}

impl Expression {
    fn evaluate(&self) -> i64 {
        match self.operator {
            Operator::PLUS => self.operands.iter().sum(),
            Operator::TIMES => self.operands.iter().product(),
        }
    }
}


fn parse_file<F>(lines: &Vec<String>, operand_parser: F) -> Vec<Expression> where F: Fn(&Vec<String>) -> Vec<Vec<i64>>{
    let operands_list = operand_parser(lines);
    let operators = parse_operators(lines);
    build_expressions(operands_list, operators)
}

fn build_expressions(operands_list: Vec<Vec<i64>>, operators: Vec<Operator>) -> Vec<Expression> {
    operands_list.into_iter()
        .enumerate()
        .map(|(index, operands)| Expression {operands, operator: operators[index]}).collect()
}

fn parse_human_operands(lines: &Vec<String>) -> Vec<Vec<i64>> {
    let operand_rows: Vec<Vec<i64>> = lines[..lines.len()-1]
        .iter()
        .map(|row| parse_human_operand_row(row))
        .collect();
    let mut result = vec![];
    for col in 0..operand_rows[0].len() {
        let mut operands = vec![];
        for row in 0..operand_rows.len() {
            operands.push(operand_rows[row][col]);
        }
        result.push(operands);
    }
    result
}

fn parse_human_operand_row(line: &String) -> Vec<i64> {
    line.split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect()
}

fn parse_operators(lines: &Vec<String>) -> Vec<Operator> {
    lines[lines.len() - 1]
        .split_whitespace()
        .map(|o| parse_operator(o))
        .collect()
}

fn parse_operator(operator: &str) -> Operator {
    match operator {
        "+" => Operator::PLUS,
        "*" => Operator::TIMES,
        _ => panic!("invalid operator {}", operator)
    }
}

fn parse_cephalopod_operands(lines: &Vec<String>) -> Vec<Vec<i64>> {
    let chars: Vec<Vec<char>> = lines.into_iter().map(|line| line.chars().collect()).collect();
    let mut result: Vec<Vec<i64>> = vec![];
    let mut current_operands: Vec<i64> = vec![];
    for col in 0..lines[0].len() {
        let mut num = 0;
        for row in 0..lines.len() - 1 {
            let c = chars[row][col];
            if c != ' ' {
                num = num * 10 + c.to_digit(10).unwrap();
            }
        }
        if num == 0 {
            result.push(current_operands);
            current_operands = vec![];
        } else {
            current_operands.push(num.into());
        }
    }
    result.push(current_operands);
    result
}
