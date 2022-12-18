use std::str::Lines;

type Axe = i8;

#[derive(Debug, PartialEq)]
struct Cube {
    x: Axe,
    y: Axe,
    z: Axe,
}

#[derive(Debug)]
enum CubeSide {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

type TotalSurfaceArea = u16;

pub fn run(input: &str) {
    let lines = input.lines();
    let cubes = parse_input(lines);

    let mut total_surface_area: TotalSurfaceArea = 0;

    for cube in cubes.iter() {
        let mut covered_sides = Vec::new();

        for another_cube in cubes.iter() {
            if cube == another_cube {
                continue;
            }

            if cube.y == another_cube.y && cube.z == another_cube.z {
                if cube.x == another_cube.x + 1 {
                    covered_sides.push(CubeSide::Left);
                } else if cube.x == another_cube.x - 1 {
                    covered_sides.push(CubeSide::Right);
                }
            }

            if cube.x == another_cube.x && cube.z == another_cube.z {
                if cube.y == another_cube.y + 1 {
                    covered_sides.push(CubeSide::Bottom);
                } else if cube.y == another_cube.y - 1 {
                    covered_sides.push(CubeSide::Top);
                }
            }

            if cube.x == another_cube.x && cube.y == another_cube.y {
                if cube.z == another_cube.z + 1 {
                    covered_sides.push(CubeSide::Back);
                } else if cube.z == another_cube.z - 1 {
                    covered_sides.push(CubeSide::Front);
                }
            }
        }

        const TOTAL_SIDES: u8 = 6;
        let exposed_sides = TOTAL_SIDES - covered_sides.len() as u8;

        total_surface_area += exposed_sides as TotalSurfaceArea;
    }

    println!("Total surface area: {}", total_surface_area);
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
