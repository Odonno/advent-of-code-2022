use std::vec;

pub fn run() {
    let input = include_str!("part1.txt");
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

    let max_calories = calories_list.iter().max().unwrap();

    println!("{}", max_calories);
}
