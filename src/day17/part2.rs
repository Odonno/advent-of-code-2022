use std::{cmp::max, collections::HashMap};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
type X = u8;
type Y = u64;
type Position = (X, Y);

type Shape = Vec<Position>;

type VerticalChamber = HashMap<Y, Vec<X>>;

const CHAMBER_MIN_X: u8 = 0;
const CHAMBER_MAX_X: u8 = 6;

const TOTAL_SHAPES: u8 = 5;

pub fn run(input: &str, use_sample: bool) {
    let directions = input
        .chars()
        .map(|char| match char {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .collect::<Vec<_>>();

    let shapes = [
        create_shape(0),
        create_shape(1),
        create_shape(2),
        create_shape(3),
        create_shape(4),
    ];

    const MAX_ROCKS_FALLING: u64 = 1_000_000_000_000;

    let mut remaining_rocks_falling = MAX_ROCKS_FALLING;

    let mut shape_index = 0;
    let mut highest_rock_y = None;
    let mut direction_index = 0;
    let mut chamber = VerticalChamber::new();

    let take_until_cycle = |shape_index: &u8, direction_index: &usize, _: &Y| {
        let cycle_shape_index = 0;
        let cycle_direction_index = if use_sample { 2 } else { 7 };

        shape_index == &cycle_shape_index && direction_index == &cycle_direction_index
    };

    let before_start_cycle = cycle(
        &shapes,
        &directions,
        shape_index,
        highest_rock_y,
        direction_index,
        &chamber,
        take_until_cycle,
    );

    shape_index = before_start_cycle.0;
    highest_rock_y = before_start_cycle.1;
    direction_index = before_start_cycle.2;
    chamber = before_start_cycle.3;
    remaining_rocks_falling -= before_start_cycle.4;

    let one_cycle = cycle(
        &shapes,
        &directions,
        shape_index,
        highest_rock_y,
        direction_index,
        &chamber,
        take_until_cycle,
    );

    shape_index = one_cycle.0;
    highest_rock_y = one_cycle.1;
    direction_index = one_cycle.2;
    chamber = one_cycle.3;
    remaining_rocks_falling -= one_cycle.4;

    let y_up_per_cycle = one_cycle.1.unwrap() - before_start_cycle.1.unwrap();
    let falls_per_cycle = one_cycle.4;
    let cycles_left = remaining_rocks_falling / falls_per_cycle;

    let y_up_for_all_cycles = y_up_per_cycle * cycles_left;

    highest_rock_y = match highest_rock_y {
        Some(highest_rock_y) => Some(y_up_for_all_cycles + highest_rock_y),
        None => panic!("No rocks in chamber?"),
    };

    remaining_rocks_falling = remaining_rocks_falling % falls_per_cycle;

    let take_until_end = |_: &u8, _: &usize, falls: &Y| falls == &remaining_rocks_falling;

    let mut chamber_for_last_cycle = VerticalChamber::new();
    for chamber_row in chamber {
        chamber_for_last_cycle.insert(chamber_row.0 + y_up_for_all_cycles, chamber_row.1);
    }

    let last_cycle = cycle(
        &shapes,
        &directions,
        shape_index,
        highest_rock_y,
        direction_index,
        &chamber_for_last_cycle,
        take_until_end,
    );

    let units_tall = last_cycle.1.unwrap() + 1;

    println!("units_tall: {}", units_tall);
}

type Shapes = [(Shape, X, Y); TOTAL_SHAPES as usize];

fn cycle(
    shapes: &Shapes,
    directions: &Vec<Direction>,
    shape_index: u8,
    highest_rock_y: Option<Y>,
    direction_index: usize,
    chamber: &VerticalChamber,
    take_until: impl Fn(&u8, &usize, &Y) -> bool,
) -> (u8, Option<Y>, usize, VerticalChamber, Y) {
    let mut highest_rock_y = highest_rock_y;
    let mut shape_index = shape_index;
    let mut direction_index = direction_index;
    let mut chamber = chamber.clone();

    let number_of_directions = directions.len() as usize;

    let mut falls = 0;

    loop {
        shape_index = (shape_index % TOTAL_SHAPES as u8) as u8;
        let (shape, shape_width, shape_height) = &shapes[shape_index as usize];

        let current_shape_index = shape_index;
        let current_direction_index = direction_index;

        const START_X: X = 2;
        const START_Y: Y = 3;

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
            direction_index = direction_index % number_of_directions;

            // check has hit another rock OR has hit the floor
            let has_hit_the_floor = y == 0;
            if has_hit_the_floor {
                break;
            }

            let next_y = y - 1;

            let has_hit_rock_stopped = shape.iter().any(|(shape_x, shape_y)| {
                let chamber_x = x + shape_x;
                let chamber_y = next_y - shape_y;

                let chamber_row = chamber.get(&chamber_y);
                match chamber_row {
                    Some(chamber_row) => chamber_row.contains(&chamber_x),
                    None => false,
                }
            });

            if has_hit_rock_stopped {
                break;
            }

            y = next_y as Y;
        }

        for (shape_x, shape_y) in shape {
            let (chamber_x, chamber_y) = (x + shape_x, y - shape_y);

            chamber
                .entry(chamber_y)
                .or_insert(Vec::new())
                .push(chamber_x);
        }

        highest_rock_y = match highest_rock_y {
            Some(highest_rock_y) => Some(max(highest_rock_y, y)),
            None => Some(y),
        };

        shape_index += 1;
        falls += 1;

        if take_until(&current_shape_index, &current_direction_index, &falls) {
            break;
        }
    }

    (shape_index, highest_rock_y, direction_index, chamber, falls)
}

fn try_move_left(x: X, y: Y, chamber: &VerticalChamber, shape: &Shape) -> X {
    if x == CHAMBER_MIN_X {
        return x;
    }

    let new_x = x - 1;

    let has_hit_rock_stopped = shape.iter().any(|(shape_x, shape_y)| {
        let chamber_x = new_x + shape_x;
        let chamber_y = y - shape_y;

        let chamber_row = chamber.get(&chamber_y);
        match chamber_row {
            Some(chamber_row) => chamber_row.contains(&chamber_x),
            None => false,
        }
    });
    if has_hit_rock_stopped {
        return x;
    }

    new_x
}

fn try_move_right(x: X, y: Y, max_x: X, chamber: &VerticalChamber, shape: &Shape) -> X {
    let new_x = x + 1;

    if new_x > max_x {
        return x;
    }

    let has_hit_rock_stopped = shape.iter().any(|(shape_x, shape_y)| {
        let chamber_x = new_x + shape_x;
        let chamber_y = y - shape_y;

        let chamber_row = chamber.get(&chamber_y);
        match chamber_row {
            Some(chamber_row) => chamber_row.contains(&chamber_x),
            None => false,
        }
    });
    if has_hit_rock_stopped {
        return x;
    }

    new_x
}

fn create_shape(index: u8) -> (Shape, X, Y) {
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

    let width = shape.iter().map(|(x, _)| x).max().unwrap().clone();
    let height = shape.iter().map(|(_, y)| y).max().unwrap().clone();

    (shape, width, height)
}
