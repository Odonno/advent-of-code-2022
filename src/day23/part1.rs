use std::{
    collections::{HashMap, VecDeque},
    str::Lines,
};

#[derive(Debug, PartialEq)]
enum Tile {
    EmptyGround,
    Elf,
}

type X = i8;
type Y = i8;
type Position = (X, Y);

type Map = HashMap<Position, Tile>;

#[derive(Debug, Clone, PartialEq)]
enum MoveProposition {
    North,
    South,
    West,
    East,
}

type MovePropositions = HashMap<Position, MoveProposition>;

type NextPositionAppearances = HashMap<Position, u8>;

type MovePreferences = VecDeque<MoveProposition>;

const ROUNDS: u8 = 10;

pub fn run(input: &str) {
    let lines = input.lines();

    let mut map = parse_input(lines);
    let mut move_preferences = initialize_move_preferences();

    println!("== Initial State ==");
    display_map(&map);

    for round in 0..ROUNDS {
        // first half
        let move_propositions = get_move_propositions(&map, &move_preferences);
        let next_position_appearances = get_next_position_appearances(&move_propositions);

        // second half
        let move_propositions =
            get_single_move_propositions(&move_propositions, &next_position_appearances);

        for (position, move_proposition) in move_propositions {
            let (x, y) = position;

            let new_position = match move_proposition {
                MoveProposition::North => (x, y - 1),
                MoveProposition::South => (x, y + 1),
                MoveProposition::West => (x - 1, y),
                MoveProposition::East => (x + 1, y),
            };

            map.insert(position, Tile::EmptyGround);
            map.insert(new_position, Tile::Elf);
        }

        let current_move_preference = move_preferences.pop_front().unwrap();
        move_preferences.push_back(current_move_preference);

        println!("== End of Round {} ==", round + 1);
        display_map(&map);
    }

    let elf_positions = get_elf_positions(&map);

    let min_elf_x = elf_positions.iter().map(|(x, _)| x).min().unwrap();
    let max_elf_x = elf_positions.iter().map(|(x, _)| x).max().unwrap();

    let min_elf_y = elf_positions.iter().map(|(_, y)| y).min().unwrap();
    let max_elf_y = elf_positions.iter().map(|(_, y)| y).max().unwrap();

    let width = (max_elf_x - min_elf_x + 1) as u16;
    let height = (max_elf_y - min_elf_y + 1) as u16;

    let total_area = width * height;
    let number_of_elves = elf_positions.len() as u16;

    let total = total_area - number_of_elves;

    println!("{:?}", total);
}

fn initialize_move_preferences() -> VecDeque<MoveProposition> {
    let mut move_preferences = VecDeque::new();
    move_preferences.push_back(MoveProposition::North);
    move_preferences.push_back(MoveProposition::South);
    move_preferences.push_back(MoveProposition::West);
    move_preferences.push_back(MoveProposition::East);

    move_preferences
}

fn get_move_propositions(map: &Map, move_preferences: &MovePreferences) -> MovePropositions {
    let elf_positions = get_elf_positions(map);

    let mut propositions = MovePropositions::new();

    for elf_position in elf_positions {
        let (x, y) = elf_position;

        let adjacent_positions = vec![
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        let has_at_least_one_elf_nearby = adjacent_positions
            .iter()
            .any(|position| map.get(position) == Some(&Tile::Elf));

        if !has_at_least_one_elf_nearby {
            continue;
        }

        for move_preference in move_preferences {
            let positions_to_check = match move_preference {
                MoveProposition::North => [(x, y - 1), (x - 1, y - 1), (x + 1, y - 1)],
                MoveProposition::South => [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)],
                MoveProposition::West => [(x - 1, y), (x - 1, y - 1), (x - 1, y + 1)],
                MoveProposition::East => [(x + 1, y), (x + 1, y - 1), (x + 1, y + 1)],
            };

            let can_move = positions_to_check.iter().all(|position| {
                map.get(position).unwrap_or(&Tile::EmptyGround) == &Tile::EmptyGround
            });
            if can_move {
                propositions.insert(elf_position, move_preference.clone());
                break;
            }
        }
    }

    propositions
}

fn get_elf_positions(map: &Map) -> Vec<Position> {
    let elf_positions = map
        .iter()
        .filter(|(_, tile)| tile == &&Tile::Elf)
        .map(|(position, _)| position.clone())
        .collect::<Vec<_>>();

    elf_positions
}

fn get_next_position_appearances(move_propositions: &MovePropositions) -> NextPositionAppearances {
    let mut appearances = NextPositionAppearances::new();

    for (position, proposition) in move_propositions {
        let (x, y) = position.clone();

        let next_position = match proposition {
            MoveProposition::North => (x, y - 1),
            MoveProposition::South => (x, y + 1),
            MoveProposition::West => (x - 1, y),
            MoveProposition::East => (x + 1, y),
        };

        let appearance = appearances.entry(next_position).or_insert(0);
        *appearance += 1;
    }

    appearances
}

fn get_single_move_propositions(
    move_propositions: &MovePropositions,
    next_position_appearances: &NextPositionAppearances,
) -> MovePropositions {
    let mut single_move_propositions = MovePropositions::new();

    for (position, move_proposition) in move_propositions {
        let (x, y) = position.clone();

        let next_position = match move_proposition {
            MoveProposition::North => (x, y - 1),
            MoveProposition::South => (x, y + 1),
            MoveProposition::West => (x - 1, y),
            MoveProposition::East => (x + 1, y),
        };

        let appearance = next_position_appearances.get(&next_position).unwrap();
        if appearance == &1 {
            single_move_propositions.insert(position.clone(), move_proposition.clone());
        }
    }

    single_move_propositions
}

fn display_map(map: &Map) {
    let min_x = map.keys().map(|(x, _)| x).min().unwrap().clone();
    let max_x = map.keys().map(|(x, _)| x).max().unwrap().clone();

    let min_y = map.keys().map(|(_, y)| y).min().unwrap().clone();
    let max_y = map.keys().map(|(_, y)| y).max().unwrap().clone();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let position = (x, y);
            let tile = map.get(&position).unwrap_or(&Tile::EmptyGround);

            let char = match tile {
                Tile::EmptyGround => '.',
                Tile::Elf => '#',
            };

            print!("{}", char);
        }
        println!();
    }
}

fn parse_input(lines: Lines) -> Map {
    let mut map = Map::new();

    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            let position = (x as X, y as Y);

            let tile = match char {
                '.' => Tile::EmptyGround,
                '#' => Tile::Elf,
                _ => panic!("Invalid character"),
            };

            map.insert(position, tile);
        }
    }

    map
}
