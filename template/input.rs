type Input = Vec<u8>;

pub fn parse_input(input: &str) -> Input {
    let mut numbers = Vec::new();

    for line in input.lines() {
        let number = line.parse::<_>().unwrap();
        numbers.push(number);
    }

    numbers
}
