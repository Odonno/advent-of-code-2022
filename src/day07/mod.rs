pub mod part1;
pub mod part2;

pub fn run(part: u8) {
    match part {
        1 => part1::run(),
        2 => part2::run(),
        _ => panic!("Invalid part number"),
    }
}
