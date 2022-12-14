use std::collections::HashMap;

type CoordValue = u32;

#[derive(Debug)]
struct Position {
    x: CoordValue,
    y: CoordValue,
}

#[derive(Debug, PartialEq)]
enum Drawing {
    Air,
    Rock,
    Sand,
}

type Coord = (CoordValue, CoordValue);

type Map = HashMap<Coord, Drawing>;

pub fn run(input: &str) {
    let lines = input.lines();

    let paths = lines
        .into_iter()
        .map(|line| {
            let positions = line
                .split("->")
                .map(|position| {
                    let mut position = position.split(",");
                    let x = position.next().unwrap().trim().parse::<u32>().unwrap();
                    let y = position.next().unwrap().trim().parse::<u32>().unwrap();

                    Position { x, y }
                })
                .collect::<Vec<_>>();

            positions
        })
        .collect::<Vec<_>>();

    let mut map = Map::new();

    for path in paths {
        let path_lines = path.windows(2);

        for path_line in path_lines {
            let start = &path_line[0];
            let end = &path_line[1];

            let is_vertical = start.x == end.x;

            if is_vertical {
                let x = start.x;

                let min_y = start.y.min(end.y);
                let max_y = start.y.max(end.y);

                for y in min_y..=max_y {
                    map.insert((x, y), Drawing::Rock);
                }
            } else {
                let y = start.y;

                let min_x = start.x.min(end.x);
                let max_x = start.x.max(end.x);

                for x in min_x..=max_x {
                    map.insert((x, y), Drawing::Rock);
                }
            }
        }
    }

    let sand_start_position: Coord = (500, 0);
    map.insert(sand_start_position, Drawing::Sand);

    let min_x = map.keys().map(|coord| coord.0).min().unwrap();
    let max_x = map.keys().map(|coord| coord.0).max().unwrap();

    let min_y = map.keys().map(|coord| coord.1).min().unwrap();
    let max_y = map.keys().map(|coord| coord.1).max().unwrap();

    let floor_y = max_y + 2;

    for x in min_x..=max_x {
        for y in min_y..floor_y {
            map.entry((x, y)).or_insert(Drawing::Air);
        }
    }

    loop {
        match get_next_falling_sand_position(&map, sand_start_position, floor_y) {
            Some(next_falling_sand_position) => {
                if next_falling_sand_position == sand_start_position {
                    break;
                }

                map.insert(next_falling_sand_position, Drawing::Sand);
            }
            None => {
                break;
            }
        };
    }

    let total_sands_units = map
        .values()
        .into_iter()
        .filter(|drawing| **drawing == Drawing::Sand)
        .count();

    display_map(&map, floor_y);

    println!("{:?}", total_sands_units);
}

fn display_map(map: &HashMap<Coord, Drawing>, floor_y: CoordValue) {
    let min_x = map.keys().map(|coord| coord.0).min().unwrap();
    let max_x = map.keys().map(|coord| coord.0).max().unwrap();

    let min_y = map.keys().map(|coord| coord.1).min().unwrap();

    for y in min_y..=floor_y {
        for x in min_x..=max_x {
            if y == floor_y {
                print!("#");
                continue;
            }

            let drawing = map.get(&(x, y)).unwrap_or(&Drawing::Air);
            let drawing = match drawing {
                Drawing::Air => ".",
                Drawing::Rock => "#",
                Drawing::Sand => "o",
            };

            print!("{}", drawing);
        }

        println!();
    }
}

fn get_next_falling_sand_position(
    map: &HashMap<Coord, Drawing>,
    sand_start_position: Coord,
    floor_y: CoordValue,
) -> Option<Coord> {
    let mut position = (sand_start_position.0, sand_start_position.1);

    loop {
        let below_position = (position.0, position.1 + 1);

        let below_drawing = match below_position {
            position if position.1 < floor_y => Some(map.get(&position).unwrap_or(&Drawing::Air)),
            position if position.1 == floor_y => Some(&Drawing::Rock),
            _ => return None,
        };

        if below_drawing.is_none() {
            return None;
        }

        let below_drawing = below_drawing.unwrap();
        if below_drawing == &Drawing::Air {
            position = below_position;
            continue;
        }

        let diagonal_bottom_left_position = (position.0 - 1, position.1 + 1);
        let bottom_left_drawing = match diagonal_bottom_left_position {
            position if position.1 < floor_y => Some(map.get(&position).unwrap_or(&Drawing::Air)),
            position if position.1 == floor_y => Some(&Drawing::Rock),
            _ => return None,
        };

        if bottom_left_drawing.is_none() {
            return None;
        }

        let bottom_left_drawing = bottom_left_drawing.unwrap();
        if bottom_left_drawing == &Drawing::Air {
            position = diagonal_bottom_left_position;
            continue;
        }

        let diagonal_bottom_right_position = (position.0 + 1, position.1 + 1);
        let bottom_right_drawing = match diagonal_bottom_right_position {
            position if position.1 < floor_y => Some(map.get(&position).unwrap_or(&Drawing::Air)),
            position if position.1 == floor_y => Some(&Drawing::Rock),
            _ => return None,
        };

        if bottom_right_drawing.is_none() {
            return None;
        }

        let bottom_right_drawing = bottom_right_drawing.unwrap();
        if bottom_right_drawing == &Drawing::Air {
            position = diagonal_bottom_right_position;
            continue;
        }

        return Some(position);
    }
}
