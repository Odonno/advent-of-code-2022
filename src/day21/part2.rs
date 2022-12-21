use std::{collections::HashMap, str::Lines};

type MonkeyName = String;

#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Add,
    Substract,
    Multiply,
    Divide,
    Match,
}

#[derive(Debug, Clone)]
struct Operation {
    operator: Operator,
    left: MonkeyName,
    right: MonkeyName,
}

#[derive(Debug, Clone)]
enum Job {
    Number(u16),
    Operation(Operation),
}

#[derive(Debug, Clone)]
struct Monkey {
    name: MonkeyName,
    job: Job,
}

#[derive(Debug, Clone)]
enum JobOwner {
    Me(MonkeyName),
    Monkey(Monkey),
}

type NumberResult = i128;

pub fn run(input: &str) {
    let lines = input.lines();

    let owners = parse_input(lines);
    let result = get_yell_number(&owners);

    println!("{:?}", result);
}

fn get_yell_number(owners: &Vec<JobOwner>) -> Option<NumberResult> {
    let min = i64::MIN;
    let max = i64::MAX;

    #[derive(Debug)]
    struct SearchExecution {
        value: i64,
        result: NumberResult,
    }

    #[derive(Debug)]
    struct Search {
        left: SearchExecution,
        right: SearchExecution,
    }

    let mut values = Search {
        left: SearchExecution {
            value: min,
            result: execute_jobs(min, owners, true).unwrap(),
        },
        right: SearchExecution {
            value: max,
            result: execute_jobs(max, owners, true).unwrap(),
        },
    };

    let mut excluded_values = Vec::new();

    loop {
        let mut x = (values.right.value + values.left.value) / 2;

        let is_excluded = excluded_values.contains(&x);
        if is_excluded {
            x = x - 1; // try left
        }

        let is_excluded = excluded_values.contains(&x);
        if is_excluded {
            x = x + 2; // try right
        }

        if is_excluded {
            todo!("How is that even possible?");
        }

        let execution = execute_jobs(x, owners, false);

        if let Ok(diff) = execution {
            if diff == 0 {
                return Some(x as NumberResult);
            }

            if values.right.result > values.left.result {
                values.right = SearchExecution {
                    value: x,
                    result: diff,
                };
            } else {
                values.left = SearchExecution {
                    value: x,
                    result: diff,
                };
            }
        } else {
            let diff = execution.unwrap_err().unwrap();

            if diff == 0 {
                excluded_values.push(x);
            }

            if values.right.result > values.left.result {
                values.right = SearchExecution {
                    value: x,
                    result: diff,
                };
            } else {
                values.left = SearchExecution {
                    value: x,
                    result: diff,
                };
            }
        }

        if values.right.value == values.left.value {
            break;
        }
    }

    None
}

fn execute_jobs(
    x: i64,
    owners: &Vec<JobOwner>,
    allow_div_error: bool,
) -> Result<NumberResult, Option<NumberResult>> {
    let mut remaining_owners = owners.clone();
    let mut values: HashMap<String, NumberResult> = HashMap::new();

    let mut has_div_error = false;

    loop {
        let mut index = 0;

        while index < remaining_owners.len() {
            let owner = &remaining_owners[index];

            if let JobOwner::Me(name) = owner {
                values.insert(name.clone(), x as NumberResult);
                remaining_owners.remove(index);
                continue;
            }

            if let JobOwner::Monkey(monkey) = owner {
                if let Job::Number(value) = monkey.job {
                    values.insert(monkey.name.clone(), value as NumberResult);
                    remaining_owners.remove(index);
                    continue;
                }

                if let Job::Operation(operation) = &monkey.job {
                    let left = values.get(&operation.left);
                    let right = values.get(&operation.right);

                    if left.is_some() && right.is_some() {
                        let left = *left.unwrap();
                        let right = *right.unwrap();

                        if monkey.name == "root" && operation.operator == Operator::Match {
                            let diff = (left - right).abs();

                            if !allow_div_error && has_div_error {
                                return Err(Some(diff));
                            }

                            return Ok(diff);
                        }

                        let result = match operation.operator {
                            Operator::Add => left + right,
                            Operator::Substract => left - right,
                            Operator::Multiply => left * right,
                            Operator::Divide => {
                                let rest = left.rem_euclid(right);
                                if rest != 0 {
                                    has_div_error = true;
                                }

                                left / right
                            }
                            _ => panic!("Invalid operator"),
                        };

                        values.insert(monkey.name.clone(), result);
                        remaining_owners.remove(index);
                        continue;
                    }
                }
            }

            index += 1;
        }

        if remaining_owners.is_empty() {
            break;
        }
    }

    Err(None)
}

fn parse_input(lines: Lines) -> Vec<JobOwner> {
    let mut owners = Vec::new();

    for line in lines {
        let mut parts = line.split(":");

        let name = parts.next().unwrap().to_string();

        let is_me = name == "humn";
        if is_me {
            owners.push(JobOwner::Me(name));
            continue;
        }

        let is_root = name == "root";

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

            let operator = if is_root {
                Operator::Match
            } else {
                match operator_str {
                    "+" => Operator::Add,
                    "-" => Operator::Substract,
                    "*" => Operator::Multiply,
                    "/" => Operator::Divide,
                    _ => panic!("Unknown operator"),
                }
            };

            let operation = Operation {
                operator,
                left,
                right,
            };

            Job::Operation(operation)
        };

        let monkey = Monkey { name, job };
        owners.push(JobOwner::Monkey(monkey));
    }

    owners
}
