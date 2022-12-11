use itertools::Itertools;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i8),
}

#[derive(Debug, Copy, Clone)]
enum Pixel {
    Lit,
    Dark,
}

#[derive(Debug)]
struct CrtRow {
    pixels: Vec<Pixel>,
}

const CRT_WIDTH: u8 = 40;
const CRT_HEIGHT: u8 = 6;

pub fn run(input: &str) {
    let lines = input.lines();

    const INITIAL_X: i32 = 1;
    let mut x = INITIAL_X;

    let mut cycle = 1;

    let mut pixels = Vec::new();

    for line in lines {
        let instruction = read_instruction(line);

        match instruction {
            Instruction::Noop => {
                draw_pixel(cycle, x, &mut pixels);

                cycle += 1;
            }
            Instruction::Addx(value) => {
                draw_pixel(cycle, x, &mut pixels);

                cycle += 1;

                draw_pixel(cycle, x, &mut pixels);

                x += value as i32;
                cycle += 1;
            }
        }
    }

    let crt_rows = pixels
        .iter()
        .chunks(CRT_WIDTH as usize)
        .into_iter()
        .take(CRT_HEIGHT as usize)
        .map(|chunk| {
            let pixels = chunk.cloned().collect::<Vec<_>>();
            CrtRow { pixels }
        })
        .collect::<Vec<_>>();

    display_crt(crt_rows);
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

fn is_pixel_drawn(cycle: i32, x: i32) -> bool {
    let pixel_drawn = (cycle - 1) % (CRT_WIDTH as i32);
    pixel_drawn == x - 1 || pixel_drawn == x || pixel_drawn == x + 1
}

fn draw_pixel(cycle: i32, x: i32, pixels: &mut Vec<Pixel>) {
    let pixel = if is_pixel_drawn(cycle, x) {
        Pixel::Lit
    } else {
        Pixel::Dark
    };

    pixels.push(pixel);
}

fn display_crt(crt_rows: Vec<CrtRow>) {
    for crt_row in crt_rows {
        for pixel in crt_row.pixels {
            match pixel {
                Pixel::Lit => print!("#"),
                Pixel::Dark => print!("."),
            }
        }

        println!();
    }
}
