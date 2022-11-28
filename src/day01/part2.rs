pub fn run() {
    let input = include_str!("part2.txt");
    let lines = input.lines();

    let mut numbers = Vec::new();
    for line in lines {
        let number = line.parse::<i32>().unwrap();
        numbers.push(number);
    }

    println!("{:?}", numbers);

    todo!();
}
