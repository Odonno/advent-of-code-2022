use super::input::{SnafuIndex, SnafuNumber, SnafuUnit};
use itertools::Itertools;

type DecimalNumber = i64;

const SNAFU_BASE: DecimalNumber = 5;

type PuzzleResult = &'static str;
const EXPECTED_SAMPLE_RESULT: PuzzleResult = "2=-1=0";

pub fn run(input: &str, use_sample: bool) {
    let result = get_result(input);

    println!("Result: {}", result);
    if use_sample {
        assert_eq!(result, EXPECTED_SAMPLE_RESULT);
    }
}

fn get_result(input: &str) -> String {
    let snafu_numbers = super::input::parse_input(input);

    let numbers = snafu_numbers
        .iter()
        .map(convert_to_decimal)
        .collect::<Vec<_>>();

    let total = numbers.iter().sum::<DecimalNumber>();

    let snafu_result = convert_to_snafu(total);
    let snafu_result = get_snafu_number_display(&snafu_result);

    snafu_result
}

fn convert_to_decimal(snafu_number: &SnafuNumber) -> DecimalNumber {
    let mut number = 0;

    for (index, unit) in snafu_number.iter() {
        let value = match unit {
            SnafuUnit::Two => 2,
            SnafuUnit::One => 1,
            SnafuUnit::Zero => continue,
            SnafuUnit::Minus => -1,
            SnafuUnit::DoubleMinus => -2,
        };

        number += value * SNAFU_BASE.pow(*index as u32) as DecimalNumber;
    }

    number
}

fn convert_to_snafu(number: DecimalNumber) -> SnafuNumber {
    let mut snafu_number = SnafuNumber::new();

    let mut index: u8 = 24;
    let mut remainder = number;

    loop {
        let is_positive = remainder > 0;
        let minimum = SNAFU_BASE.pow(index as u32);

        let next_minimum = if index == 0 {
            0
        } else {
            let mut n = 0;

            for i in 0..index {
                n += 2 * SNAFU_BASE.pow(i as u32);
            }

            n
        };

        let abs_remainder = remainder.abs();

        let has_minimum = abs_remainder >= minimum;

        let should_skip = !has_minimum && snafu_number.is_empty();
        if should_skip {
            index -= 1;
            continue;
        }

        let mut total_units = 0;

        while remainder.abs() > next_minimum {
            if is_positive {
                remainder -= minimum;
            } else {
                remainder += minimum;
            }

            total_units += 1;
        }

        let unit = match is_positive {
            true => match total_units {
                0 => SnafuUnit::Zero,
                1 => SnafuUnit::One,
                2 => SnafuUnit::Two,
                _ => panic!("Unexpected number of units: {}", total_units),
            },
            false => match total_units {
                0 => SnafuUnit::Zero,
                1 => SnafuUnit::Minus,
                2 => SnafuUnit::DoubleMinus,
                _ => panic!("Unexpected number of units: {}", total_units),
            },
        };

        snafu_number.insert(index as SnafuIndex, unit);

        if index == 0 {
            break;
        }

        index -= 1;
    }

    snafu_number
}

fn get_snafu_number_display(snafu_result: &SnafuNumber) -> String {
    let mut str = String::new();

    for (_, unit) in snafu_result
        .iter()
        .sorted_by(|(index_a, _), (index_b, _)| index_a.cmp(index_b).reverse())
    {
        let char = match unit {
            SnafuUnit::Two => '2',
            SnafuUnit::One => '1',
            SnafuUnit::Zero => '0',
            SnafuUnit::Minus => '-',
            SnafuUnit::DoubleMinus => '=',
        };

        str += &char.to_string();
    }

    str
}
