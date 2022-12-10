#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i8),
}

#[derive(Debug)]
struct SignalStrength {
    cycle: i32,
    x: i32,
}

pub fn run() {
    let input = include_str!("part1.txt");
    let lines = input.lines();

    const INITIAL_X: i32 = 1;
    let mut x = INITIAL_X;

    let mut cycle = 1;

    let mut signal_strengths: Vec<SignalStrength> = Vec::new();

    const CYCLES_TO_CHECK: [i32; 6] = [20, 60, 100, 140, 180, 220];

    for line in lines {
        let instruction = read_instruction(line);

        match instruction {
            Instruction::Noop => {
                cycle += 1;

                if CYCLES_TO_CHECK.contains(&cycle) {
                    signal_strengths.push(SignalStrength { cycle, x });
                }
            }
            Instruction::Addx(value) => {
                let previous_x = x;

                x += value as i32;
                cycle += 2;

                if CYCLES_TO_CHECK.contains(&cycle) {
                    signal_strengths.push(SignalStrength { cycle, x });
                }
                if CYCLES_TO_CHECK.contains(&(cycle - 1)) {
                    signal_strengths.push(SignalStrength {
                        cycle: cycle - 1,
                        x: previous_x,
                    });
                }
            }
        }
    }

    let total = signal_strengths
        .iter()
        .map(|signal_strength| signal_strength.cycle * signal_strength.x)
        .sum::<i32>();

    println!("{:?}", total);
}

fn read_instruction(line: &str) -> Instruction {
    if line == "noop" {
        return Instruction::Noop;
    }

    let args = line.split_whitespace().collect::<Vec<_>>();

    if args[0] == "addx" {
        let x = args[1].parse::<i8>().unwrap();
        return Instruction::Addx(x);
    }

    panic!("Invalid instruction: {}", line);
}
