pub fn run() {
    let day = env!("DAY").parse().unwrap();
    let part = env!("PART").parse().unwrap();

    match day {
        1 => day01(part),
        2 => day02(part),
        3 => day03(part),
        4 => day04(part),
        5 => day05(part),
        6 => day06(part),
        7 => day07(part),
        8 => day08(part),
        9 => day09(part),
        10 => day10(part),
        11 => day11(part),
        _ => panic!("Invalid day number. Did you forget to generate this day using the script?"),
    }
}

use crate::day01::run as day01;
use crate::day02::run as day02;
use crate::day03::run as day03;
use crate::day04::run as day04;
use crate::day05::run as day05;
use crate::day06::run as day06;
use crate::day07::run as day07;
use crate::day08::run as day08;
use crate::day09::run as day09;
use crate::day10::run as day10;
use crate::day11::run as day11;