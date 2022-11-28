pub fn run(day: u8, part: u8) {
    match day {
        1 => day01(part),
        _ => panic!("Invalid day number. Did you forget to generate this day using the script?"),
    }
}

use crate::day01::run as day01;
