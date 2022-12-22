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

#[derive(Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

pub fn run(input: &str) {
    let lines = input.lines();

    let input = parse_input(lines);

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

                    let tile = input.map.get(&next_position);
                    if tile.is_none() {
                        next_position =
                            move_outside_board(&current_position, &current_direction, &input.map);
                    }

                    let tile = input.map.get(&next_position).unwrap();
                    match tile {
                        Tile::Open => {
                            current_position = next_position;
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

fn move_outside_board(
    current_position: &Position,
    current_direction: &Direction,
    map: &Map,
) -> Position {
    let (x, y) = current_position;

    match current_direction {
        Direction::Right => {
            let min_x = map
                .keys()
                .filter(|(_, tile_y)| tile_y == y)
                .map(|(tile_x, _)| tile_x)
                .min()
                .unwrap()
                .clone();

            (min_x, y.clone())
        }
        Direction::Down => {
            let min_y = map
                .keys()
                .filter(|(tile_x, _)| tile_x == x)
                .map(|(_, tile_y)| tile_y)
                .min()
                .unwrap()
                .clone();

            (x.clone(), min_y)
        }
        Direction::Left => {
            let max_x = map
                .keys()
                .filter(|(_, tile_y)| tile_y == y)
                .map(|(tile_x, _)| tile_x)
                .max()
                .unwrap()
                .clone();

            (max_x, y.clone())
        }
        Direction::Up => {
            let max_y = map
                .keys()
                .filter(|(tile_x, _)| tile_x == x)
                .map(|(_, tile_y)| tile_y)
                .max()
                .unwrap()
                .clone();

            (x.clone(), max_y)
        }
    }
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
