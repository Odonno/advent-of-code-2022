use itertools::Itertools;

#[derive(Debug)]
struct Group {
    rucksack_one: String,
    rucksack_two: String,
    rucksack_three: String,
}

pub fn run() {
    let input = include_str!("part2.txt");
    let lines = input.lines();

    let groups = lines
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let lines = chunk.collect_vec();

            let rucksack_one = lines[0].to_string();
            let rucksack_two = lines[1].to_string();
            let rucksack_three = lines[2].to_string();

            return Group {
                rucksack_one,
                rucksack_two,
                rucksack_three,
            };
        })
        .collect::<Vec<_>>();

    let items = groups
        .iter()
        .map(|group| {
            return find_char_in_group(group);
        })
        .collect::<Vec<_>>();

    let items_priority = items.iter().map(|item| {
        return get_item_priority(&item);
    });

    let total_priorities = items_priority.sum::<u32>();

    println!("{:?}", total_priorities);
}

fn find_char_in_group(group: &Group) -> char {
    let rucksack_one_chars = group.rucksack_one.chars().collect::<Vec<_>>();
    let rucksack_two_chars = group.rucksack_two.chars().collect::<Vec<_>>();
    let rucksack_three_chars = group.rucksack_three.chars().collect::<Vec<_>>();

    for char_one in rucksack_one_chars {
        for char_two in &rucksack_two_chars {
            if char_one != *char_two {
                continue;
            }

            for char_three in &rucksack_three_chars {
                if char_one == *char_three {
                    return char_one;
                }
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
