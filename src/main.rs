mod cli;
mod generate;
mod puzzle;

use clap::Parser;
use cli::{Args, Commands};

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Generate { day }) => generate::run(day),
        None => puzzle::run(),
    }
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
