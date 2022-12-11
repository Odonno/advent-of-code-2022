use regex::Regex;

#[derive(Debug, Clone)]
struct Stack {
    order: u8,
    crates: Vec<char>,
}

#[derive(Debug)]
struct Move {
    crates_to_move: u8,
    from: u8,
    to: u8,
}

pub fn run(input: &str) {
    let lines = input.lines();

    let lines_array = lines.collect::<Vec<&str>>();

    let splitted_lines = lines_array
        .split(|&s| s.is_empty())
        .collect::<Vec<&[&str]>>();

    let stack_configuration_lines = splitted_lines[0];
    let stacks = extract_stacks(stack_configuration_lines);

    let moves_lines = splitted_lines[1];
    let moves = extract_moves(moves_lines);

    // apply moves
    let mut stacks = stacks;

    for move_ in moves {
        let from_stack_index = stacks
            .clone()
            .iter()
            .position(|stack| stack.order == move_.from)
            .unwrap();

        let to_stack_index = stacks
            .clone()
            .iter()
            .position(|stack| stack.order == move_.to)
            .unwrap();

        let from_stack = stacks.get_mut(from_stack_index).unwrap();

        let crates_to_move = from_stack
            .crates
            .split_off(from_stack.crates.len() - move_.crates_to_move as usize);

        let to_stack = stacks.get_mut(to_stack_index).unwrap();

        to_stack.crates.extend(crates_to_move);
    }

    let message = stacks
        .iter()
        .map(|stack| stack.crates.last().unwrap().to_string())
        .collect::<Vec<_>>()
        .join("");

    println!("{}", message);
}

fn extract_stacks(stack_configuration_lines: &[&str]) -> Vec<Stack> {
    let first_line = stack_configuration_lines[0];
    let number_of_stacks = (first_line.len() + 1) / 4;

    let number_of_lines_for_stack_configuration = stack_configuration_lines.len();

    let stacks: Vec<Stack> = (1..number_of_stacks + 1)
        .map(|i| {
            let crates = (0..number_of_lines_for_stack_configuration - 1)
                .rev()
                .map(|line| {
                    let line = stack_configuration_lines[line];
                    let index = (i - 1) * 4 + 1;
                    line.chars().nth(index).unwrap()
                })
                .filter(|crate_name| crate_name != &' ')
                .collect::<Vec<_>>();

            let stack = Stack {
                order: i as u8,
                crates,
            };

            stack
        })
        .collect();

    stacks
}

fn extract_moves(moves_lines: &[&str]) -> Vec<Move> {
    let moves = moves_lines
        .iter()
        .map(|&s| {
            let regex_string = r"^move (\d+) from (\d+) to (\d+)$";
            let regex = Regex::new(regex_string).unwrap();

            let values = regex.captures(s).unwrap();

            let crates_to_move = values[1].parse::<u8>().unwrap();
            let from = values[2].parse::<u8>().unwrap();
            let to = values[3].parse::<u8>().unwrap();

            Move {
                crates_to_move,
                from,
                to,
            }
        })
        .collect::<Vec<_>>();

    moves
}
