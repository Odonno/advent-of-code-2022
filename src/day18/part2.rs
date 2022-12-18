use std::{
    collections::{HashSet, VecDeque},
    str::Lines,
};

type Axe = i8;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Cube {
    x: Axe,
    y: Axe,
    z: Axe,
}

type TotalSurfaceArea = u16;

pub fn run(input: &str) {
    let lines = input.lines();
    let cubes = parse_input(lines);

    let min_x = cubes.iter().map(|cube| cube.x).min().unwrap();
    let max_x = cubes.iter().map(|cube| cube.x).max().unwrap();

    let min_y = cubes.iter().map(|cube| cube.y).min().unwrap();
    let max_y = cubes.iter().map(|cube| cube.y).max().unwrap();

    let min_z = cubes.iter().map(|cube| cube.z).min().unwrap();
    let max_z = cubes.iter().map(|cube| cube.z).max().unwrap();

    const START_POSITION: (Axe, Axe, Axe) = (1, 1, 1);

    let mut next_positions_to_check = VecDeque::from(vec![START_POSITION]);

    let mut visited_positions = HashSet::new();
    visited_positions.insert(START_POSITION);

    let mut exterior_surface_area: TotalSurfaceArea = 0;

    while let Some(current_position) = next_positions_to_check.pop_front() {
        let positions_next_to_current = get_positions_next_to_current(current_position);

        for next_position in positions_next_to_current {
            let (x, y, z) = next_position;

            let is_outside = x < min_x - 1
                || x > max_x + 1
                || y < min_y - 1
                || y > max_y + 1
                || z < min_z - 1
                || z > max_z + 1;
            if is_outside {
                continue;
            }

            let has_hit_cube = cubes.iter().any(|cube| {
                cube.x == next_position.0 && cube.y == next_position.1 && cube.z == next_position.2
            });
            if has_hit_cube {
                exterior_surface_area += 1;
                visited_positions.insert(next_position);
                continue;
            }

            let is_already_visited = visited_positions.contains(&next_position);
            if !is_already_visited {
                visited_positions.insert(next_position);
                next_positions_to_check.push_back(next_position);
            }
        }
    }

    println!("Exterior surface area: {}", exterior_surface_area);
}

fn parse_input(lines: Lines) -> Vec<Cube> {
    let cubes = lines
        .map(|line| {
            let mut axes = line.split(",");

            let x = axes.next().unwrap().parse::<Axe>().unwrap();
            let y = axes.next().unwrap().parse::<Axe>().unwrap();
            let z = axes.next().unwrap().parse::<Axe>().unwrap();

            Cube { x, y, z }
        })
        .collect::<Vec<_>>();

    cubes
}

fn get_positions_next_to_current(position: (Axe, Axe, Axe)) -> Vec<(Axe, Axe, Axe)> {
    let (x, y, z) = position;

    vec![
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}
