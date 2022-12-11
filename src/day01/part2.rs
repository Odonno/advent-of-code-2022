use itertools::Itertools;
use std::vec;

pub fn run(input: &str) {
    let lines = input.lines();

    let mut elves = vec![];
    elves.push(vec![]);

    for line in lines {
        if line.is_empty() {
            elves.push(vec![]);
            continue;
        }

        let calories = line.parse::<i32>().unwrap();
        let current_elf = elves.last_mut().unwrap();

        current_elf.push(calories);
    }

    let calories_list = elves
        .iter()
        .map(|elf| elf.iter().sum::<i32>())
        .collect::<Vec<_>>();

    let sorted_calories_list = calories_list.into_iter().sorted().rev().collect::<Vec<_>>();
    let top3_calories = sorted_calories_list.into_iter().take(3).collect::<Vec<_>>();

    let total_carrying = top3_calories.iter().sum::<i32>();

    println!("{:?}", total_carrying);
}
