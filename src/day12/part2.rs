use std::collections::HashMap;

type Position = (i8, i8);
type Elevation = u8;
type Heightmap = HashMap<Position, Elevation>;

type Path = Vec<Position>;

type ParentPosition = Position;
type ParentMap = HashMap<Position, ParentPosition>;

type Score = u32;
type ScoreMap = HashMap<Position, Score>;

pub fn run(input: &str) {
    let lines = input.lines();

    let mut heightmap: Heightmap = Heightmap::new();

    let mut target_position = (0, 0);

    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            let position = (x as i8, y as i8);

            if char == 'E' {
                target_position = position;
            }

            let elevation = match char {
                'S' => 0,
                'E' => 25,
                char => {
                    let ascii_value = char as u32;
                    const START_ASCII_INDEX: u32 = 97;

                    (ascii_value - START_ASCII_INDEX) as u8
                }
            };

            heightmap.insert(position, elevation);
        }
    }

    let starting_positions = heightmap
        .iter()
        .filter(|(_, elevation)| **elevation == 0)
        .map(|(position, _)| *position)
        .collect::<Vec<_>>();

    let best_paths = starting_positions
        .into_iter()
        .map(|origin_position| find_path(origin_position, target_position, &heightmap))
        .filter(|path| path.is_some())
        .map(|path| path.unwrap())
        .collect::<Vec<_>>();

    let best_path = best_paths
        .into_iter()
        .min_by(|path1, path2| path1.len().cmp(&path2.len()))
        .unwrap();

    let moves = best_path.len() - 1;

    println!("Moves: {}", moves);
}

fn calculate_distance(origin: Position, target: Position) -> Score {
    let (x1, y1) = origin;
    let (x2, y2) = target;

    let x_distance = (x1 - x2).abs() as u32;
    let y_distance = (y1 - y2).abs() as u32;

    x_distance + y_distance
}

fn is_accessible(position: Position, neighbor: Position, heightmap: &Heightmap) -> bool {
    let current_elevation = heightmap.get(&position).unwrap();
    let neighbor = heightmap.get(&neighbor);

    neighbor.is_some() && neighbor.unwrap() <= &(current_elevation + 1)
}

fn find_neighbors(position: Position, heightmap: &Heightmap) -> Vec<Position> {
    let (x, y) = position;

    let neighbor_positions: Vec<Position> = vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];

    let neighbors = neighbor_positions
        .into_iter()
        .filter(|neighbor| is_accessible(position, *neighbor, heightmap))
        .collect::<Vec<_>>();

    neighbors
}

fn reconstruct_path(parents: ParentMap, mut current: Position) -> Path {
    let mut path: Path = vec![current];

    while parents.contains_key(&current) {
        current = parents.get(&current).unwrap().clone();
        path.push(current);
    }

    path
}

fn find_path(origin: Position, target: Position, heightmap: &Heightmap) -> Option<Path> {
    let mut open_set: Vec<Position> = vec![origin];

    let mut g_scores: ScoreMap = ScoreMap::new();
    let mut f_scores: ScoreMap = ScoreMap::new();

    let mut parents: ParentMap = ParentMap::new();

    let total_distance = calculate_distance(origin, target);

    g_scores.insert(origin, 0);
    f_scores.insert(origin, total_distance);

    while !open_set.is_empty() {
        let current = open_set
            .iter()
            .min_by(|a, b| f_scores.get(a).unwrap().cmp(f_scores.get(b).unwrap()))
            .unwrap()
            .clone();

        if current == target {
            return Some(reconstruct_path(parents, current));
        }

        open_set.retain(|position| position != &current);

        let neighbors = find_neighbors(current, heightmap);

        for neighbor in neighbors {
            let tentative_g_score = g_scores.get(&current).unwrap() + 1;
            let neighbor_g_score = g_scores.get(&neighbor).unwrap_or(&Score::MAX).clone();

            if tentative_g_score < neighbor_g_score {
                parents.insert(neighbor, current);
                g_scores.insert(neighbor, tentative_g_score);

                let h_score = calculate_distance(neighbor, target);
                let f_score = tentative_g_score + h_score;
                f_scores.insert(neighbor, f_score);

                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor);
                }
            }
        }
    }

    None
}
