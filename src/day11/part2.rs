use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

type Monkey = u8;
type ItemIndex = u8;
type WorryLevel = u128;

#[derive(Debug)]
struct InstructionTest {
    divisible_by: u8,
    monkey_receiver_when_true: Monkey,
    monkey_receiver_when_false: Monkey,
}

#[derive(Debug)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug)]
enum Operand {
    Value(u8),
    OldValue,
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    left: Operand,
    right: Operand,
}

#[derive(Debug)]
struct MonkeyInstruction {
    monkey: Monkey,
    starting_items: Vec<WorryLevel>,
    operation: Operation,
    test: InstructionTest,
}

const ROUNDS: u16 = 10000;

pub fn run(input: &str) {
    let lines = input.lines();

    let lines_array = lines.collect::<Vec<&str>>();

    let mut instructions = Vec::new();

    {
        let splitted_lines = lines_array
            .split(|&s| s.is_empty())
            .collect::<Vec<&[&str]>>();

        for lines in splitted_lines {
            let instruction = parse_instruction(lines);
            instructions.push(instruction);
        }
    }

    let mut item_worries: HashMap<ItemIndex, WorryLevel> = HashMap::new();
    let mut monkey_holding_items: HashMap<ItemIndex, Monkey> = HashMap::new();
    let mut item_inspections: HashMap<Monkey, u64> = HashMap::new();

    {
        let mut item_index = 0;

        for instruction in &instructions {
            for starting_item in &instruction.starting_items {
                item_worries.insert(item_index, *starting_item);
                monkey_holding_items.insert(item_index, instruction.monkey);

                item_index += 1;
            }
        }
    }

    let maximum_worry_level = instructions
        .iter()
        .map(|i| i.test.divisible_by as u128)
        .fold(1, |acc, x| acc * x);

    for _ in 0..ROUNDS {
        for instruction in &instructions {
            let currently_holding_items = monkey_holding_items
                .iter()
                .filter(|(_, &monkey)| monkey == instruction.monkey)
                .map(|(&item_index, _)| item_index)
                .collect::<Vec<_>>();

            for item_index in currently_holding_items {
                let monkey_inspection = item_inspections.entry(instruction.monkey).or_insert(0);
                *monkey_inspection += 1;

                let old_worry_level = item_worries.get(&item_index).unwrap().clone();
                let new_worry_level =
                    calculate_worry_level(old_worry_level, &instruction.operation);
                let new_worry_level = new_worry_level % maximum_worry_level as WorryLevel;

                item_worries
                    .entry(item_index)
                    .and_modify(|v| *v = new_worry_level);

                let test_result =
                    new_worry_level % (instruction.test.divisible_by as WorryLevel) == 0;
                let monkey_receiver = if test_result {
                    instruction.test.monkey_receiver_when_true
                } else {
                    instruction.test.monkey_receiver_when_false
                };

                monkey_holding_items
                    .entry(item_index)
                    .and_modify(|v| *v = monkey_receiver);
            }
        }
    }

    let two_most_inspections = item_inspections
        .values()
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .collect::<Vec<_>>();

    let monkey_business = two_most_inspections[0] * two_most_inspections[1];

    println!("{:?}", monkey_business);
}

fn parse_instruction(lines: &[&str]) -> MonkeyInstruction {
    let monkey_line = lines[0];
    let starting_items_line = lines[1];
    let operation_line = lines[2];
    let test_line = lines[3];
    let test_line_when_true = lines[4];
    let test_line_when_false = lines[5];

    let monkey = parse_monkey(monkey_line);
    let starting_items = parse_starting_items(starting_items_line);
    let operation = parse_operation(operation_line);
    let test = parse_test(test_line, test_line_when_true, test_line_when_false);

    MonkeyInstruction {
        monkey,
        starting_items,
        operation,
        test,
    }
}

fn parse_monkey(line: &str) -> Monkey {
    let regex = Regex::new(r"Monkey (\d+):").unwrap();
    let captures = regex.captures(line).unwrap();
    let monkey = captures.get(1).unwrap().as_str().parse().unwrap();

    monkey
}

fn parse_starting_items(line: &str) -> Vec<WorryLevel> {
    let regex = Regex::new(r"Starting items: (.*)").unwrap();
    let captures = regex.captures(line).unwrap();
    let starting_items_str = captures.get(1).unwrap().as_str();

    let starting_items = starting_items_str
        .split(", ")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<WorryLevel>>();

    starting_items
}

fn parse_operation(operation_line: &str) -> Operation {
    let regex = Regex::new(r"Operation: new = (.+) (.) (.+)").unwrap();
    let captures = regex.captures(operation_line).unwrap();
    let left = captures.get(1).unwrap().as_str();
    let operator = captures.get(2).unwrap().as_str();
    let right = captures.get(3).unwrap().as_str();

    let left = parse_operand(left);
    let operator = parse_operator(operator);
    let right = parse_operand(right);

    Operation {
        operator,
        left,
        right,
    }
}

fn parse_operand(str: &str) -> Operand {
    if str == "old" {
        return Operand::OldValue;
    }

    let value = str.parse().unwrap();
    return Operand::Value(value);
}

fn parse_operator(operator: &str) -> Operator {
    match operator {
        "+" => Operator::Addition,
        "-" => Operator::Subtraction,
        "*" => Operator::Multiplication,
        "/" => Operator::Division,
        _ => panic!("Invalid operator"),
    }
}

fn parse_test(
    test_line: &str,
    test_line_when_true: &str,
    test_line_when_false: &str,
) -> InstructionTest {
    let regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let captures = regex.captures(test_line).unwrap();
    let divisible_by = captures.get(1).unwrap().as_str().parse().unwrap();

    let regex = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
    let captures = regex.captures(test_line_when_true).unwrap();
    let monkey_receiver_when_true = captures.get(1).unwrap().as_str().parse().unwrap();

    let regex = Regex::new(r"If false: throw to monkey (\d+)").unwrap();
    let captures = regex.captures(test_line_when_false).unwrap();
    let monkey_receiver_when_false = captures.get(1).unwrap().as_str().parse().unwrap();

    InstructionTest {
        divisible_by,
        monkey_receiver_when_true,
        monkey_receiver_when_false,
    }
}

fn calculate_worry_level(old_worry_level: WorryLevel, operation: &Operation) -> WorryLevel {
    let left_value = match operation.left {
        Operand::OldValue => old_worry_level,
        Operand::Value(value) => value as WorryLevel,
    };

    let right_value = match operation.right {
        Operand::OldValue => old_worry_level,
        Operand::Value(value) => value as WorryLevel,
    };

    match operation.operator {
        Operator::Addition => left_value + right_value,
        Operator::Subtraction => left_value - right_value,
        Operator::Multiplication => left_value * right_value,
        Operator::Division => left_value / right_value,
    }
}
