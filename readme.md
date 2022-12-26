# Advent of Code 2022

This repository contains the solution for the [Advent of Code of 2022](https://adventofcode.com/2022) using the Rust language exclusively.

Because this challenge is part of a learning process of the Rust language, the solution may certainly not be the perfect solution and you can still find room for improvement on most of them.

## Get started

### Run solution code

The cargo CLI is what you'll need to run the code. But only one puzzle at a time, and only one part of a single day. Example: you can run the part 1 of the day 7. You can run the code using the input file or even the sample file provided by the Advent of Code.

For that, you need to set the Env Variables that live in the `.cargo/config.toml` file:

```toml
[env]
DAY = "7"
PART = "1"
USE_SAMPLE = "false"
```

`DAY` and `PART` are the day/part you want to run and `USE_SAMPLE` detect whether to run the `input.txt` or the `sample.txt` file inside the `src/day7` folder.

When done, you can run the CLI using the following command:

```
cargo run
```

### Script generation

This project contains a script generation feature that provides the ability to easily create files from a template using a single command line:

```
cargo run -- generate
```

It will prompt you to give the day of the puzzle. Once done, it will generate a new folder inside `src/`.

The template you can find in the `template/` folder consists of multiple files:

- sample.txt - the sample text provided by Advent of Code website
- input.txt - the input text provided by Advent of Code website
- mod.rs - a dummy module used to redirect to the part1 or part2 `run` function
- input.rs - a module used exclusively to parse input, exporting a `parse_input` function
- part1.rs - the basic template which contains the part 1 `run` function
- part2.rs - the basic template which contains the part 2 `run` function

## Features & Improvements

- Display information on each code run
  - [x] Display day, part and if using sample
  - [x] Display the time it took to run the code
- Script generation
  - [x] Generate files (modules, input) from a template for each day
  - [x] Ask the day to create, using the current day of the month as default value
  - [x] Check if day folder already exist
  - [x] Update config.toml after generation
- Template
  - [x] Use a single `input.txt` file (alongside a `sample.txt` file)
  - [x] Use a global configuration file/env variables to run puzzle (`config.toml`)
  - [x] Create a file `input.rs` in the template with a function called `parse_input(input: &str)`
  - [x] Add an `assert_eq!` line at the end of each part template, only to test the sample result
- Be TDD-oriented (for next year)

## Language

Rust 1.64.0

## Dependencies

| Name      | Version | Usage                                                                                                                                                      |
| --------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| chrono    | 0.4.19  | Performance benchmark (time performance of the algorithm)                                                                                                  |
| clap      | 4.0.29  | Simplify creation of CLI (run algorithm, script generation, etc...)                                                                                        |
| colored   | 2.0.0   | Apply colors and style on the CLI output                                                                                                                   |
| dialoguer | 0.10.2  | Simplify CLI prompt (asking for the day puzzle to generate)                                                                                                |
| itertools | 0.10.5  | Used to get access to specific iterator methods. <br />Like the immutable `sorted_by` function instead of the mutable `sort` function...                   |
| rayon     | 1.5.1   | Used to execute code in parallel. <br />Useful only for the day 19 part 2 to get the response faster due to a suboptimal algorithm. If you know, you know. |
| regex     | 1.5.4   | Used to parse complex input. <br />Extremely useful in order to extract numbers, strings, etc...                                                           |
