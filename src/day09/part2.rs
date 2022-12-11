use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    distance: u8,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

pub fn run(input: &str) {
    let lines = input.lines();

    let mut moves = Vec::new();
    for line in lines {
        let args = line.split_whitespace().collect::<Vec<_>>();

        let direction = match args[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };

        let distance = args[1].parse::<u8>().unwrap();

        let _move = Move {
            direction,
            distance,
        };

        moves.push(_move);
    }

    let mut rope: HashMap<u8, Position> = HashMap::new();

    const ROPE_LENGTH: u8 = 10;

    for index in 0..ROPE_LENGTH {
        rope.insert(index, Position { x: 0, y: 0 });
    }

    let mut grid_visits: HashMap<(i32, i32), bool> = HashMap::new();
    grid_visits.insert((0, 0), true);

    for _move in moves {
        display_map(&rope);

        for _ in 0.._move.distance {
            let head_position = rope.get(&0).unwrap();
            let new_head_position = move_head(&head_position, &_move.direction);

            rope.entry(0).and_modify(|e| *e = new_head_position);

            for index in 1..ROPE_LENGTH {
                let head = rope.get(&(index - 1)).unwrap();
                let tail = rope.get(&index).unwrap();

                let new_position = move_tail(&tail, &head);

                rope.entry(index).and_modify(|e| *e = new_position);
            }

            const TAIL_INDEX: u8 = ROPE_LENGTH - 1;
            let tail_position = rope.get(&TAIL_INDEX).unwrap();

            let already_visited = grid_visits.contains_key(&(tail_position.x, tail_position.y));
            if !already_visited {
                grid_visits.insert((tail_position.x, tail_position.y), true);
            }

            display_map(&rope);
        }
    }

    let total_of_visits = grid_visits.len();

    println!("{:?}", total_of_visits);
}

fn display_map(rope: &HashMap<u8, Position>) {
    let all_ropes = rope
        .into_iter()
        .sorted_by(|(index, _), (index2, _)| index.partial_cmp(index2).unwrap())
        .collect::<Vec<_>>();

    const X_ABS: i32 = 10;
    const Y_ABS: i32 = 10;

    println!("----------------------------------");

    for y in -Y_ABS..Y_ABS {
        for x in -X_ABS..X_ABS {
            let rope_position = all_ropes
                .iter()
                .find(|(_, position)| position.x == x && position.y == y);

            match rope_position {
                Some(v) => {
                    if v.0 == &0 {
                        print!("H");
                    } else {
                        print!("{}", v.0);
                    }
                }
                None => print!("."),
            }
        }

        println!();
    }
}

fn move_head(head_position: &Position, direction: &Direction) -> Position {
    match direction {
        Direction::Up => Position {
            x: head_position.x,
            y: head_position.y - 1,
        },
        Direction::Down => Position {
            x: head_position.x,
            y: head_position.y + 1,
        },
        Direction::Left => Position {
            x: head_position.x - 1,
            y: head_position.y,
        },
        Direction::Right => Position {
            x: head_position.x + 1,
            y: head_position.y,
        },
    }
}

fn move_tail(tail_position: &Position, head_position: &Position) -> Position {
    if tail_position.x == head_position.x {
        if tail_position.y < head_position.y - 1 {
            return Position {
                x: tail_position.x,
                y: tail_position.y + 1,
            };
        }
        if tail_position.y > head_position.y + 1 {
            return Position {
                x: tail_position.x,
                y: tail_position.y - 1,
            };
        }
    }

    if tail_position.y == head_position.y {
        if tail_position.x < head_position.x - 1 {
            return Position {
                x: tail_position.x + 1,
                y: tail_position.y,
            };
        }
        if tail_position.x > head_position.x + 1 {
            return Position {
                x: tail_position.x - 1,
                y: tail_position.y,
            };
        }
    }

    if tail_position.x < head_position.x - 1 {
        if tail_position.y < head_position.y {
            return Position {
                x: tail_position.x + 1,
                y: tail_position.y + 1,
            };
        }

        return Position {
            x: tail_position.x + 1,
            y: tail_position.y - 1,
        };
    }

    if tail_position.x > head_position.x + 1 {
        if tail_position.y < head_position.y {
            return Position {
                x: tail_position.x - 1,
                y: tail_position.y + 1,
            };
        }

        return Position {
            x: tail_position.x - 1,
            y: tail_position.y - 1,
        };
    }

    if tail_position.y < head_position.y - 1 {
        if tail_position.x < head_position.x {
            return Position {
                x: tail_position.x + 1,
                y: tail_position.y + 1,
            };
        }

        return Position {
            x: tail_position.x - 1,
            y: tail_position.y + 1,
        };
    }

    if tail_position.y > head_position.y + 1 {
        if tail_position.x < head_position.x {
            return Position {
                x: tail_position.x + 1,
                y: tail_position.y - 1,
            };
        }

        return Position {
            x: tail_position.x - 1,
            y: tail_position.y - 1,
        };
    }

    return tail_position.clone();
}
