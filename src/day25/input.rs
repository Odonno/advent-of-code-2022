use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum SnafuUnit {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

pub type SnafuIndex = u8;
pub type SnafuNumber = HashMap<SnafuIndex, SnafuUnit>;
pub type SnafuNumberList = Vec<SnafuNumber>;

pub fn parse_input(input: &str) -> SnafuNumberList {
    let mut list = SnafuNumberList::new();

    for line in input.lines() {
        let mut number = SnafuNumber::new();

        for (index, char) in line.chars().rev().enumerate() {
            let unit = match char {
                '2' => SnafuUnit::Two,
                '1' => SnafuUnit::One,
                '0' => SnafuUnit::Zero,
                '-' => SnafuUnit::Minus,
                '=' => SnafuUnit::DoubleMinus,
                _ => panic!("Invalid character: {}", char),
            };

            number.insert(index as SnafuIndex, unit);
        }

        list.push(number);
    }

    list
}
