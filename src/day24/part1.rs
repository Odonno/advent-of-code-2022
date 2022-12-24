use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::Lines,
};

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Clone)]
struct BlizzardDirections {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Wall,
    Ground,
    Blizzards(BlizzardDirections),
}

type X = i8;
type Y = i8;
type Position = (X, Y);

type Map = HashMap<Position, Tile>;

type Paths = HashSet<Position>;

pub fn run(input: &str) {
    let lines = input.lines();

    let initial_map = parse_input(lines);

    let min_y = initial_map.keys().map(|(_, y)| y).min().unwrap().clone();
    let max_y = initial_map.keys().map(|(_, y)| y).max().unwrap().clone();

    let initial_position = initial_map
        .iter()
        .find(|((_, y), tile)| y == &min_y && tile == &&Tile::Ground)
        .map(|(position, _)| position.clone())
        .unwrap();

    let target_position = initial_map
        .iter()
        .find(|((_, y), tile)| y == &max_y && tile == &&Tile::Ground)
        .map(|(position, _)| position.clone())
        .unwrap();

    let mut minute = 0;
    let mut map = initial_map.clone();
    let mut paths = Paths::new();
    paths.insert(initial_position.clone());

    loop {
        map = alter_map(&map);

        paths = paths
            .iter()
            .flat_map(|path| try_next_moves(&map, path.clone()))
            .collect();

        minute += 1;

        let path_to_target = paths.iter().find(|path| path == &&target_position);
        if path_to_target.is_some() {
            break;
        }
    }

    println!("{:?}", minute);
}

/// Move all blizzards from the previous map to the current map
fn alter_map(previous_map: &Map) -> Map {
    let mut map = previous_map.clone();

    map = remove_blizzards(&map);

    for (position, tile) in previous_map.iter() {
        if let Tile::Blizzards(directions) = tile {
            if directions.left {
                let direction = Direction::Left;
                let new_position =
                    get_next_blizzard_position(position.clone(), &previous_map, direction.clone());

                let tile = map.get_mut(&new_position).unwrap();
                if let Tile::Blizzards(directions) = tile {
                    directions.left = true;
                }
                if tile == &Tile::Ground {
                    *tile = create_blizzard_tile(direction);
                }
            }

            if directions.right {
                let direction = Direction::Right;
                let new_position =
                    get_next_blizzard_position(position.clone(), &previous_map, direction.clone());

                let tile = map.get_mut(&new_position).unwrap();
                if let Tile::Blizzards(directions) = tile {
                    directions.right = true;
                }
                if tile == &Tile::Ground {
                    *tile = create_blizzard_tile(direction);
                }
            }

            if directions.up {
                let direction = Direction::Up;
                let new_position =
                    get_next_blizzard_position(position.clone(), &previous_map, direction.clone());

                let tile = map.get_mut(&new_position).unwrap();
                if let Tile::Blizzards(directions) = tile {
                    directions.up = true;
                }
                if tile == &Tile::Ground {
                    *tile = create_blizzard_tile(direction);
                }
            }

            if directions.down {
                let direction = Direction::Down;
                let new_position =
                    get_next_blizzard_position(position.clone(), &previous_map, direction.clone());

                let tile = map.get_mut(&new_position).unwrap();
                if let Tile::Blizzards(directions) = tile {
                    directions.down = true;
                }
                if tile == &Tile::Ground {
                    *tile = create_blizzard_tile(direction);
                }
            }
        }
    }

    map
}

/// Move to the next position (or wait)
fn try_next_moves(map: &Map, current_position: Position) -> Paths {
    let (current_x, current_y) = current_position;

    const POSSIBLE_MOVES: [Option<Direction>; 5] = [
        None,
        Some(Direction::Down),
        Some(Direction::Right),
        Some(Direction::Left),
        Some(Direction::Up),
    ];

    let mut next_paths = Paths::new();

    for possible_move in POSSIBLE_MOVES {
        let new_position = match possible_move {
            Some(direction) => match direction {
                Direction::Left => (current_x - 1, current_y),
                Direction::Right => (current_x + 1, current_y),
                Direction::Up => (current_x, current_y - 1),
                Direction::Down => (current_x, current_y + 1),
            },
            None => current_position,
        };

        let can_move = map.get(&new_position).unwrap_or(&Tile::Wall) == &Tile::Ground;
        if can_move {
            next_paths.insert(new_position);
        }
    }

    next_paths
}

fn get_next_blizzard_position(position: Position, map: &Map, direction: Direction) -> Position {
    match direction {
        Direction::Left => {
            let (current_x, current_y) = position;
            let new_position = (current_x - 1, current_y);

            if map.get(&new_position) == Some(&Tile::Wall) {
                return map
                    .iter()
                    .filter(|((_, y), tile)| y == &current_y && tile != &&Tile::Wall)
                    .sorted_by(|((x1, _), _), ((x2, _), _)| x1.partial_cmp(x2).unwrap().reverse())
                    .nth(0)
                    .unwrap()
                    .0
                    .clone();
            }

            return new_position;
        }
        Direction::Right => {
            let (current_x, current_y) = position;
            let new_position = (current_x + 1, current_y);

            if map.get(&new_position) == Some(&Tile::Wall) {
                return map
                    .iter()
                    .filter(|((_, y), tile)| y == &current_y && tile != &&Tile::Wall)
                    .sorted_by(|((x1, _), _), ((x2, _), _)| x1.partial_cmp(x2).unwrap())
                    .nth(0)
                    .unwrap()
                    .0
                    .clone();
            }

            return new_position;
        }
        Direction::Up => {
            let (current_x, current_y) = position;
            let new_position = (current_x, current_y - 1);

            if map.get(&new_position) == Some(&Tile::Wall) {
                return map
                    .iter()
                    .filter(|((x, _), tile)| x == &current_x && tile != &&Tile::Wall)
                    .sorted_by(|((_, y1), _), ((_, y2), _)| y1.partial_cmp(y2).unwrap().reverse())
                    .nth(0)
                    .unwrap()
                    .0
                    .clone();
            }

            return new_position;
        }
        Direction::Down => {
            let (current_x, current_y) = position;
            let new_position = (current_x, current_y + 1);

            if map.get(&new_position) == Some(&Tile::Wall) {
                return map
                    .iter()
                    .filter(|((x, _), tile)| x == &current_x && tile != &&Tile::Wall)
                    .sorted_by(|((_, y1), _), ((_, y2), _)| y1.partial_cmp(y2).unwrap())
                    .nth(0)
                    .unwrap()
                    .0
                    .clone();
            }

            return new_position;
        }
    }
}

/// Remove blizzard tiles on current map
fn remove_blizzards(map: &Map) -> Map {
    map.iter()
        .map(|(position, tile)| match tile {
            Tile::Blizzards(_) => (position.clone(), Tile::Ground),
            _ => (position.clone(), tile.clone()),
        })
        .collect()
}

fn create_blizzard_tile(direction: Direction) -> Tile {
    match direction {
        Direction::Left => Tile::Blizzards(BlizzardDirections {
            left: true,
            right: false,
            up: false,
            down: false,
        }),
        Direction::Right => Tile::Blizzards(BlizzardDirections {
            left: false,
            right: true,
            up: false,
            down: false,
        }),
        Direction::Up => Tile::Blizzards(BlizzardDirections {
            left: false,
            right: false,
            up: true,
            down: false,
        }),
        Direction::Down => Tile::Blizzards(BlizzardDirections {
            left: false,
            right: false,
            up: false,
            down: true,
        }),
    }
}

fn parse_input(lines: Lines) -> Map {
    let mut map = Map::new();

    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            let position = (x as i8, y as i8);

            let tile = match char {
                '#' => Tile::Wall,
                '.' => Tile::Ground,
                '<' => create_blizzard_tile(Direction::Left),
                '>' => create_blizzard_tile(Direction::Right),
                '^' => create_blizzard_tile(Direction::Up),
                'v' => create_blizzard_tile(Direction::Down),
                _ => panic!("Invalid character in input"),
            };

            map.insert(position, tile);
        }
    }

    map
}
