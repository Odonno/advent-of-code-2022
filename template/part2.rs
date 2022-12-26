type PuzzleResult = u8;
const EXPECTED_SAMPLE_RESULT: PuzzleResult = 0;

pub fn run(input: &str, use_sample: bool) {
    let result = get_result(input);

    println!("Result: {}", result);
    if use_sample {
        assert_eq!(result, EXPECTED_SAMPLE_RESULT);
    }

    todo!();
}

fn get_result(input: &str) -> PuzzleResult {
    let lines = input.lines();

    let mut numbers = Vec::new();

    for line in lines {
        let number = line.parse::<u8>().unwrap();
        numbers.push(number);
    }

    numbers.into_iter().sum::<PuzzleResult>()
}
