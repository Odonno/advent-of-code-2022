pub fn run() {
    let day = env!("DAY").parse::<u8>().unwrap();

    match day {
        1 => day01(),
        2 => day02(),
        3 => day03(),
        4 => day04(),
        5 => day05(),
        6 => day06(),
        7 => day07(),
        8 => day08(),
        9 => day09(),
        10 => day10(),
        11 => day11(),
        12 => day12(),
        13 => day13(),
        14 => day14(),
        15 => day15(),
        16 => day16(),
        17 => day17(),
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
use crate::day12::run as day12;
use crate::day13::run as day13;
use crate::day14::run as day14;
use crate::day15::run as day15;
use crate::day16::run as day16;
use crate::day17::run as day17;
