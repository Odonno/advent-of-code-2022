#[derive(Debug)]
struct Rucksack {
    compartment_one: String,
    compartment_two: String,
}

pub fn run(input: &str) {
    let lines = input.lines();

    let mut rucksacks = Vec::new();
    for line in lines {
        let length = line.len();
        let half_length = length / 2;

        let compartment_one = line[0..half_length].to_string();
        let compartment_two = line[half_length..length].to_string();

        rucksacks.push(Rucksack {
            compartment_one,
            compartment_two,
        });
    }

    let items = rucksacks
        .iter()
        .map(|r| {
            let compartment_one_chars = r.compartment_one.chars().collect::<Vec<_>>();
            let compartment_two_chars = r.compartment_two.chars().collect::<Vec<_>>();

            return find_char_in_both_compartments(compartment_one_chars, compartment_two_chars);
        })
        .collect::<Vec<_>>();

    let items_priority = items.iter().map(|item| {
        return get_item_priority(&item);
    });

    let total_priorities = items_priority.sum::<u32>();

    println!("{:?}", total_priorities);
}

fn find_char_in_both_compartments(
    compartment_one_chars: Vec<char>,
    compartment_two_chars: Vec<char>,
) -> char {
    for char_one in compartment_one_chars {
        for char_two in &compartment_two_chars {
            if char_one == *char_two {
                return char_one;
            }
        }
    }

    panic!("No matching char found");
}

fn get_item_priority(item: &char) -> u32 {
    let ascii_value = *item as u32;

    if item.is_lowercase() {
        const START_ASCII_INDEX: u32 = 97;
        const START_VALUE: u32 = 1;

        return ascii_value - START_ASCII_INDEX + START_VALUE;
    } else {
        const START_ASCII_INDEX: u32 = 65;
        const START_VALUE: u32 = 27;

        return ascii_value - START_ASCII_INDEX + START_VALUE;
    }
}
