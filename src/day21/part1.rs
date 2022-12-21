use std::{collections::HashMap, str::Lines};

type MonkeyName = String;

#[derive(Debug)]
enum Operator {
    Add,
    Substract,
    Multiply,
    Divide,
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    left: MonkeyName,
    right: MonkeyName,
}

#[derive(Debug)]
enum Job {
    Number(u16),
    Operation(Operation),
}

#[derive(Debug)]
struct Monkey {
    name: MonkeyName,
    job: Job,
}

type Result = i64;

pub fn run(input: &str) {
    let lines = input.lines();

    let monkeys = parse_input(lines);
    let mut remaining_monkeys = monkeys;

    let mut values: HashMap<String, Result> = HashMap::new();

    loop {
        let mut index = 0;

        while index < remaining_monkeys.len() {
            let monkey = &remaining_monkeys[index];

            if let Job::Number(value) = monkey.job {
                values.insert(monkey.name.clone(), value as Result);
                remaining_monkeys.remove(index);
                continue;
            }

            if let Job::Operation(operation) = &monkey.job {
                let left = values.get(&operation.left);
                let right = values.get(&operation.right);

                if left.is_some() && right.is_some() {
                    let left = *left.unwrap();
                    let right = *right.unwrap();

                    let result = match operation.operator {
                        Operator::Add => left + right,
                        Operator::Substract => left - right,
                        Operator::Multiply => left * right,
                        Operator::Divide => left / right,
                    };

                    values.insert(monkey.name.clone(), result);
                    remaining_monkeys.remove(index);
                    continue;
                }
            }

            index += 1;
        }

        if remaining_monkeys.is_empty() {
            break;
        }
    }

    let root_value = values.get("root").unwrap();

    println!("{}", root_value);
}

fn parse_input(lines: Lines) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    for line in lines {
        let mut parts = line.split(":");

        let name = parts.next().unwrap().to_string();

        let second_part = parts.next().unwrap().trim();
        let is_digit = second_part.chars().all(|c| c.is_digit(10));

        let job = if is_digit {
            let number = second_part.parse::<_>().unwrap();
            Job::Number(number)
        } else {
            let mut parts = second_part.split(" ");

            let left = parts.next().unwrap().to_string();
            let operator_str = parts.next().unwrap();
            let right = parts.next().unwrap().to_string();

            let operator = match operator_str {
                "+" => Operator::Add,
                "-" => Operator::Substract,
                "*" => Operator::Multiply,
                "/" => Operator::Divide,
                _ => panic!("Unknown operator"),
            };

            let operation = Operation {
                operator,
                left,
                right,
            };

            Job::Operation(operation)
        };

        let monkey = Monkey { name, job };
        monkeys.push(monkey);
    }

    monkeys
}
