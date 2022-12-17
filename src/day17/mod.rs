pub mod part1;
pub mod part2;

pub fn run() {
    let part = env!("PART").parse::<u8>().unwrap();
    let use_sample = env!("USE_SAMPLE").parse::<bool>().unwrap();

    let input = if use_sample {
        include_str!("sample.txt")
    } else {
        include_str!("input.txt")
    };

    match part {
        1 => part1::run(input),
        2 => part2::run(input, use_sample),
        _ => panic!("Invalid part number"),
    }
}
