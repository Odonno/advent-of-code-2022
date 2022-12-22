use regex::Regex;
use std::{collections::HashMap, str::Lines};

#[derive(Debug)]
enum Tile {
    Open,
    Wall,
}

type X = i16;
type Y = i16;
type Position = (X, Y);

type Map = HashMap<Position, Tile>;

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
enum PathMove {
    Forward(u8),
    Turn(Turn),
}

type Path = Vec<PathMove>;

#[derive(Debug)]
struct Input {
    map: Map,
    path: Path,
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, PartialEq)]
struct Region {
    start_x: X,
    start_y: Y,
    end_x: X,
    end_y: Y,
}

type Regions = Vec<Region>;

pub fn run(input: &str, use_sample: bool) {
    let lines = input.lines();

    let input = parse_input(lines);

    let region_size: u8 = if use_sample { 4 } else { 50 };

    let regions = extract_regions(&input.map, region_size);

    let mut current_direction = Direction::Right;

    let start_x = input
        .map
        .keys()
        .filter(|(_, tile_y)| tile_y == &0)
        .map(|(x, _)| x)
        .min()
        .unwrap()
        .clone();
    let mut current_position = (start_x, 0);

    for path_move in input.path {
        match path_move {
            PathMove::Forward(moves) => {
                for _ in 0..moves {
                    let (x, y) = current_position;

                    let mut next_position = match current_direction {
                        Direction::Left => (x - 1, y),
                        Direction::Up => (x, y - 1),
                        Direction::Right => (x + 1, y),
                        Direction::Down => (x, y + 1),
                    };
                    let mut next_direction = current_direction.clone();

                    let tile = input.map.get(&next_position);
                    if tile.is_none() {
                        let result = move_to_another_region(
                            &current_position,
                            &current_direction,
                            &regions,
                            use_sample,
                        );

                        next_position = result.0;
                        next_direction = result.1;
                    }

                    let tile = input.map.get(&next_position).unwrap();
                    match tile {
                        Tile::Open => {
                            current_position = next_position;
                            current_direction = next_direction;
                        }
                        Tile::Wall => {
                            break;
                        }
                    }
                }
            }
            PathMove::Turn(turn) => {
                current_direction = match turn {
                    Turn::Left => match current_direction {
                        Direction::Left => Direction::Down,
                        Direction::Up => Direction::Left,
                        Direction::Right => Direction::Up,
                        Direction::Down => Direction::Right,
                    },
                    Turn::Right => match current_direction {
                        Direction::Left => Direction::Up,
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                    },
                }
            }
        }
    }

    let facing = match current_direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };

    let (x, y) = current_position;

    let row = (y + 1) as u32;
    let column = (x + 1) as u32;

    let final_password = (1000 * row) + (4 * column) + facing;

    println!("Final password: {}", final_password);
}

fn move_to_another_region(
    current_position: &Position,
    current_direction: &Direction,
    regions: &Regions,
    use_sample: bool,
) -> (Position, Direction) {
    let (x, y) = current_position;

    let current_region = regions
        .iter()
        .find(|region| {
            let x_in_range = &region.start_x <= x && x <= &region.end_x;
            let y_in_range = &region.start_y <= y && y <= &region.end_y;

            x_in_range && y_in_range
        })
        .unwrap();

    let face_1 = &regions[0];
    let face_2 = &regions[1];
    let face_3 = &regions[2];
    let face_4 = &regions[3];
    let face_5 = &regions[4];
    let face_6 = &regions[5];

    let normalized_x = x - current_region.start_x;
    let inverted_x = current_region.end_x - x;

    let normalized_y = y - current_region.start_y;
    let inverted_y = current_region.end_y - y;

    if use_sample {
        if current_region == face_1 {
            match current_direction {
                Direction::Right => {
                    let next_region = face_6;

                    let new_position = (next_region.end_x, next_region.start_y + inverted_y);
                    let new_direction = Direction::Left;

                    return (new_position, new_direction);
                }
                Direction::Left => {
                    let next_region = face_3;

                    let new_position = (next_region.start_x + normalized_y, next_region.start_y);
                    let new_direction = Direction::Down;

                    return (new_position, new_direction);
                }
                Direction::Up => {
                    let next_region = face_2;

                    let new_position = (next_region.start_x + inverted_x, next_region.start_y);
                    let new_direction = Direction::Down;

                    return (new_position, new_direction);
                }
                _ => {
                    panic!("This should not happen!");
                }
            }
        }
        if current_region == face_2 {
            match current_direction {
                Direction::Left => {
                    let next_region = face_6;

                    let new_position = (next_region.start_x + inverted_y, next_region.end_y);
                    let new_direction = Direction::Up;

                    return (new_position, new_direction);
                }
                Direction::Up => {
                    let next_region = face_1;

                    let new_position = (next_region.start_x + inverted_x, next_region.start_y);
                    let new_direction = Direction::Down;

                    return (new_position, new_direction);
                }
                Direction::Down => {
                    let next_region = face_5;

                    let new_position = (next_region.start_x + inverted_x, next_region.end_y);
                    let new_direction = Direction::Up;

                    return (new_position, new_direction);
                }
                _ => {
                    panic!("This should not happen!");
                }
            }
        }
        if current_region == face_3 {
            match current_direction {
                Direction::Up => {
                    let next_region = face_1;

                    let new_position = (next_region.start_x, next_region.start_y + normalized_x);
                    let new_direction = Direction::Right;

                    return (new_position, new_direction);
                }
                Direction::Down => {
                    let next_region = face_5;

                    let new_position = (next_region.start_x, next_region.start_y + inverted_x);
                    let new_direction = Direction::Right;

                    return (new_position, new_direction);
                }
                _ => {
                    panic!("This should not happen!");
                }
            }
        }
        if current_region == face_4 {
            match current_direction {
                Direction::Right => {
                    let next_region = face_6;

                    let new_position = (next_region.start_x + inverted_y, next_region.start_y);
                    let new_direction = Direction::Down;

                    return (new_position, new_direction);
                }
                _ => {
                    panic!("This should not happen!");
                }
            }
        }
        if current_region == face_5 {
            match current_direction {
                Direction::Left => {
                    let next_region = face_3;

                    let new_position = (next_region.start_x + inverted_y, next_region.end_y);
                    let new_direction = Direction::Up;

                    return (new_position, new_direction);
                }
                Direction::Down => {
                    let next_region = face_2;

                    let new_position = (next_region.start_x + inverted_x, next_region.end_y);
                    let new_direction = Direction::Up;

                    return (new_position, new_direction);
                }
                _ => {
                    panic!("This should not happen!");
                }
            }
        }
        if current_region == face_6 {
            match current_direction {
                Direction::Up => {
                    let next_region = face_4;

                    let new_position = (next_region.end_x, next_region.start_y + inverted_y);
                    let new_direction = Direction::Left;

                    return (new_position, new_direction);
                }
                Direction::Right => {
                    let next_region = face_1;

                    let new_position = (next_region.end_x, next_region.start_y + inverted_y);
                    let new_direction = Direction::Left;

                    return (new_position, new_direction);
                }
                Direction::Down => {
                    let next_region = face_2;

                    let new_position = (next_region.start_x, next_region.start_y + inverted_x);
                    let new_direction = Direction::Right;

                    return (new_position, new_direction);
                }
                _ => {
                    panic!("This should not happen!");
                }
            }
        }
    }

    if current_region == face_1 {
        match current_direction {
            Direction::Left => {
                let next_region = face_4;

                let new_position = (next_region.start_x, next_region.start_y + inverted_y);
                let new_direction = Direction::Right;

                return (new_position, new_direction);
            }
            Direction::Up => {
                let next_region = face_6;

                let new_position = (next_region.start_x, next_region.start_y + normalized_x);
                let new_direction = Direction::Right;

                return (new_position, new_direction);
            }
            _ => {
                panic!("This should not happen!");
            }
        }
    }
    if current_region == face_2 {
        match current_direction {
            Direction::Up => {
                let next_region = face_6;

                let new_position = (next_region.start_x + normalized_x, next_region.end_y);
                let new_direction = Direction::Up;

                return (new_position, new_direction);
            }
            Direction::Right => {
                let next_region = face_5;

                let new_position = (next_region.end_x, next_region.start_y + inverted_y);
                let new_direction = Direction::Left;

                return (new_position, new_direction);
            }
            Direction::Down => {
                let next_region = face_3;

                let new_position = (next_region.end_x, next_region.start_y + normalized_x);
                let new_direction = Direction::Left;

                return (new_position, new_direction);
            }
            _ => {
                panic!("This should not happen!");
            }
        }
    }
    if current_region == face_3 {
        match current_direction {
            Direction::Left => {
                let next_region = face_4;

                let new_position = (next_region.start_x + normalized_y, next_region.start_y);
                let new_direction = Direction::Down;

                return (new_position, new_direction);
            }
            Direction::Right => {
                let next_region = face_2;

                let new_position = (next_region.start_x + normalized_y, next_region.end_y);
                let new_direction = Direction::Up;

                return (new_position, new_direction);
            }
            _ => {
                panic!("This should not happen!");
            }
        }
    }
    if current_region == face_4 {
        match current_direction {
            Direction::Left => {
                let next_region = face_1;

                let new_position = (next_region.start_x, next_region.start_y + inverted_y);
                let new_direction = Direction::Right;

                return (new_position, new_direction);
            }
            Direction::Up => {
                let next_region = face_3;

                let new_position = (next_region.start_x, next_region.start_y + normalized_x);
                let new_direction = Direction::Right;

                return (new_position, new_direction);
            }
            _ => {
                panic!("This should not happen!");
            }
        }
    }
    if current_region == face_5 {
        match current_direction {
            Direction::Right => {
                let next_region = face_2;

                let new_position = (next_region.end_x, next_region.start_y + inverted_y);
                let new_direction = Direction::Left;

                return (new_position, new_direction);
            }
            Direction::Down => {
                let next_region = face_6;

                let new_position = (next_region.end_x, next_region.start_y + normalized_x);
                let new_direction = Direction::Left;

                return (new_position, new_direction);
            }
            _ => {
                panic!("This should not happen!");
            }
        }
    }
    if current_region == face_6 {
        match current_direction {
            Direction::Left => {
                let next_region = face_1;

                let new_position = (next_region.start_x + normalized_y, next_region.start_y);
                let new_direction = Direction::Down;

                return (new_position, new_direction);
            }
            Direction::Right => {
                let next_region = face_5;

                let new_position = (next_region.start_x + normalized_y, next_region.end_y);
                let new_direction = Direction::Up;

                return (new_position, new_direction);
            }
            Direction::Down => {
                let next_region = face_2;

                let new_position = (next_region.start_x + normalized_x, next_region.start_y);
                let new_direction = Direction::Down;

                return (new_position, new_direction);
            }
            _ => {
                panic!("This should not happen!");
            }
        }
    }

    panic!("This should not happen!");
}

fn extract_regions(map: &Map, region_size: u8) -> Regions {
    const NUMBER_OF_REGIONS: usize = 6;

    let mut regions = Regions::new();

    let mut current_x = 0;
    let mut current_y = 0;

    while regions.len() < NUMBER_OF_REGIONS {
        let tile = map.get(&(current_x, current_y));

        if tile.is_none() {
            current_x += region_size as X;

            let max_x_on_row = map
                .keys()
                .filter(|(_, tile_y)| tile_y == &current_y)
                .map(|(tile_x, _)| tile_x)
                .max()
                .unwrap()
                .clone();

            if current_x > max_x_on_row {
                current_x = 0;
                current_y += region_size as Y;
            }

            continue;
        }

        let region = Region {
            start_x: current_x,
            start_y: current_y,
            end_x: (current_x + region_size as X - 1),
            end_y: (current_y + region_size as Y - 1),
        };

        regions.push(region);

        current_x += region_size as X;
    }

    regions
}

fn parse_input(lines: Lines) -> Input {
    let lines = lines.collect::<Vec<_>>();
    let parts = lines.split(|line| line.is_empty()).collect::<Vec<_>>();

    let map_part = parts[0];
    let map = parse_map(map_part);

    let path_part = parts[1];
    let path = parse_path(&path_part);

    Input { map, path }
}

fn parse_map(lines: &[&str]) -> Map {
    let mut map = Map::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let tile = match char {
                '.' => Tile::Open,
                '#' => Tile::Wall,
                ' ' => continue,
                _ => panic!("Invalid character"),
            };

            let position = (x as X, y as Y);
            map.insert(position, tile);
        }
    }

    map
}

fn parse_path(lines: &[&str]) -> Path {
    let line = lines[0];
    let regex = Regex::new(r"(\d+|[R|L])").unwrap();

    let mut path = Path::new();

    for capture in regex.captures_iter(line) {
        let capture = capture.get(0).unwrap().as_str();

        let path_move = match capture {
            "R" => PathMove::Turn(Turn::Right),
            "L" => PathMove::Turn(Turn::Left),
            _ => {
                let moves = capture.parse().unwrap();
                PathMove::Forward(moves)
            }
        };

        path.push(path_move);
    }

    path
}
