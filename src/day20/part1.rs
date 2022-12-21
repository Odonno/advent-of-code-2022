use std::str::Lines;

type Number = i16;

#[derive(Debug)]
struct Item {
    value: Number,
    original_position: usize,
}

pub fn run(input: &str) {
    let lines = input.lines();

    let mut arrangement = parse_input(lines);

    let arrangement_length = arrangement.len();

    let message_size = arrangement_length - 1;

    for arrangement_index in 0..arrangement_length {
        let current_index = arrangement
            .iter()
            .position(|n| n.original_position == arrangement_index)
            .unwrap();

        let number_to_move = arrangement[current_index].value;
        if number_to_move == 0 {
            continue;
        }

        let new_index = current_index as Number + number_to_move;
        let new_index = ((new_index as Number % message_size as Number) + message_size as Number)
            as usize
            % message_size;

        let number = arrangement.remove(current_index);
        arrangement.insert(new_index as usize, number);
    }

    let final_arrangement = arrangement;

    let zero_position = final_arrangement.iter().position(|x| x.value == 0).unwrap();

    let thousand_number = final_arrangement[(zero_position + 1000) % arrangement_length].value;
    let two_thousand_number = final_arrangement[(zero_position + 2000) % arrangement_length].value;
    let three_thousand_number =
        final_arrangement[(zero_position + 3000) % arrangement_length].value;

    let total = thousand_number + two_thousand_number + three_thousand_number;
    println!("{}", total);
}

fn parse_input(lines: Lines) -> Vec<Item> {
    let mut numbers: Vec<Item> = Vec::new();

    for (original_position, line) in lines.enumerate() {
        let value = line.parse::<_>().unwrap();

        numbers.push(Item {
            value,
            original_position,
        });
    }

    return numbers;
}
