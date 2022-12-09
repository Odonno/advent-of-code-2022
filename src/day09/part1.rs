use std::collections::HashMap;

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

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

pub fn run() {
    let input = include_str!("part1.txt");
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

    let mut grid_visits: HashMap<(i32, i32), bool> = HashMap::new();

    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { x: 0, y: 0 };

    grid_visits.insert((tail_position.x, tail_position.y), true);

    for _move in moves {
        for _ in 0.._move.distance {
            match _move.direction {
                Direction::Up => {
                    head_position.y -= 1;
                }
                Direction::Down => {
                    head_position.y += 1;
                }
                Direction::Left => {
                    head_position.x -= 1;
                }
                Direction::Right => {
                    head_position.x += 1;
                }
            }

            tail_position = move_tail(&tail_position, &head_position);

            let already_visited = grid_visits.contains_key(&(tail_position.x, tail_position.y));
            if !already_visited {
                grid_visits.insert((tail_position.x, tail_position.y), true);
            }
        }
    }

    let total_of_visits = grid_visits.len();

    println!("{:?}", total_of_visits);
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
