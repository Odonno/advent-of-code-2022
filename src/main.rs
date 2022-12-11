mod import;

fn main() {
    let day = env!("DAY").parse().unwrap();
    let part = env!("PART").parse().unwrap();

    import::run(day, part);
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
