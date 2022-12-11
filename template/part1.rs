pub fn run(input: &str) {
    let lines = input.lines();

    let mut numbers = Vec::new();
    for line in lines {
        let number = line.parse::<i32>().unwrap();
        numbers.push(number);
    }

    println!("{:?}", numbers);

    todo!();
}
