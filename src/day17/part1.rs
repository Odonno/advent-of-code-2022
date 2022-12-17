#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

type Position = (u8, u32);
type Shape = Vec<Position>;

type VerticalChamber = Vec<Position>;

const CHAMBER_MIN_X: u8 = 0;
const CHAMBER_MAX_X: u8 = 6;

pub fn run(input: &str) {
    let directions = input
        .chars()
        .map(|char| match char {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .collect::<Vec<_>>();

    let max_direction_index = directions.len() as usize;
    let mut direction_index = 0;

    const MAX_ROCKS_FALLING: u16 = 2022;

    let mut chamber = VerticalChamber::new();

    for index in 0..MAX_ROCKS_FALLING {
        let shape = create_shape(index);

        let shape_width = shape.iter().map(|(x, _)| x).max().unwrap().clone();
        let shape_height = shape.iter().map(|(_, y)| y).max().unwrap().clone();

        let highest_rock_y = chamber.iter().map(|(_, y)| y).max();

        const START_X: u8 = 2;
        const START_Y: u32 = 3;

        let mut x = START_X;
        let mut y = match highest_rock_y {
            Some(highest_rock_y) => 1 + highest_rock_y + START_Y + shape_height,
            None => START_Y + shape_height,
        };

        let max_x = CHAMBER_MAX_X - shape_width;

        loop {
            // try move left/right (min/max based on walls/stopped rock)
            let direction = &directions[direction_index];
            x = match direction {
                Direction::Left => try_move_left(x, y, &chamber, &shape),
                Direction::Right => try_move_right(x, y, max_x, &chamber, &shape),
            };

            direction_index += 1;
            direction_index = direction_index % max_direction_index;

            // check has hit another rock OR has hit the floor
            let next_y = y as i32 - 1;

            let has_hit_rock_stopped = chamber.iter().any(|(rock_x, rock_y)| {
                shape.iter().any(|(shape_x, shape_y)| {
                    return *rock_x == x + shape_x && (*rock_y) as i32 == next_y - *shape_y as i32;
                })
            });

            if has_hit_rock_stopped {
                break;
            }

            let lowest_y = next_y - shape_height as i32;
            let has_hit_the_floor = lowest_y < 0;
            if has_hit_the_floor {
                break;
            }

            y = next_y as u32;
        }

        for (shape_x, shape_y) in shape {
            let position_in_chamber = (x + shape_x, y - shape_y);
            chamber.push(position_in_chamber);
        }

        if index < 10 {
            display_chamber(&chamber);
        }
    }

    let units_tall = chamber.iter().map(|(_, y)| y).max().unwrap().clone() + 1;
    println!("units_tall: {}", units_tall);
}

fn try_move_left(x: u8, y: u32, chamber: &VerticalChamber, shape: &Shape) -> u8 {
    if x == CHAMBER_MIN_X {
        return x;
    }

    let new_x = x - 1;

    for (shape_x, shape_y) in shape {
        let rock_in_chamber = chamber
            .iter()
            .find(|(rock_x, rock_y)| rock_x == &(new_x + shape_x) && rock_y == &(y - shape_y));

        if rock_in_chamber.is_some() {
            return x;
        }
    }

    new_x
}

fn try_move_right(x: u8, y: u32, max_x: u8, chamber: &VerticalChamber, shape: &Shape) -> u8 {
    let new_x = x + 1;

    if new_x > max_x {
        return x;
    }

    for (shape_x, shape_y) in shape {
        let rock_in_chamber = chamber
            .iter()
            .find(|(rock_x, rock_y)| rock_x == &(new_x + shape_x) && rock_y == &(y - shape_y));

        if rock_in_chamber.is_some() {
            return x;
        }
    }

    new_x
}

fn create_shape(index: u16) -> Shape {
    const TOTAL_SHAPES: u16 = 5;

    let index = index % TOTAL_SHAPES;
    let mut shape = Shape::new();

    match index {
        0 => {
            for x in 0..=3 {
                shape.push((x, 0));
            }
        }
        1 => {
            for x in 0..=2 {
                for y in 0..=2 {
                    if x == 1 || y == 1 {
                        shape.push((x, y));
                    }
                }
            }
        }
        2 => {
            for x in 0..=2 {
                for y in 0..=2 {
                    if x == 2 || y == 2 {
                        shape.push((x, y));
                    }
                }
            }
        }
        3 => {
            for y in 0..=3 {
                shape.push((0, y));
            }
        }
        4 => {
            for x in 0..=1 {
                for y in 0..=1 {
                    shape.push((x, y));
                }
            }
        }
        _ => panic!("Invalid index"),
    }

    shape
}

fn display_chamber(chamber: &VerticalChamber) {
    let max_y = chamber.iter().map(|(_, y)| y).max().unwrap_or(&1).clone();

    for y in (0..=max_y).rev() {
        print!("|");

        for x in CHAMBER_MIN_X..=CHAMBER_MAX_X {
            let element = chamber
                .iter()
                .find(|(rock_x, rock_y)| rock_x == &x && rock_y == &y);

            match element {
                Some(_) => print!("#"),
                None => print!("."),
            }
        }

        print!("|");
        println!();
    }

    print!("+");
    for _ in CHAMBER_MIN_X..=CHAMBER_MAX_X {
        print!("-");
    }
    print!("+");
    println!();
}
