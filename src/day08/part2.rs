use itertools::Itertools;

#[derive(Debug)]
pub struct Tree {
    x: u32,
    y: u32,
    height: u8,
}

pub fn run(input: &str) {
    let lines = input.lines();

    let mut trees = Vec::new();

    for (y, line) in lines.enumerate() {
        let chars = line.chars();

        for (x, c) in chars.enumerate() {
            let height = c.to_digit(10).unwrap();

            let tree = Tree {
                x: x as u32,
                y: y as u32,
                height: height as u8,
            };
            trees.push(tree);
        }
    }

    let min_x = 0;
    let max_x = trees.iter().map(|tree| tree.x).max().unwrap();

    let min_y = 0;
    let max_y = trees.iter().map(|tree| tree.y).max().unwrap();

    let interior_trees = trees
        .iter()
        .filter(|tree| tree.x != min_x && tree.x != max_x && tree.y != min_y && tree.y != max_y)
        .collect::<Vec<_>>();

    let scenic_scores = interior_trees
        .iter()
        .map(|tree| {
            let left_trees = trees
                .iter()
                .filter(|other_tree| other_tree.x < tree.x && other_tree.y == tree.y)
                .sorted_by(|a, b| a.x.cmp(&b.x))
                .rev()
                .collect::<Vec<_>>();

            let viewing_distance_left = get_viewing_distance(tree, left_trees);

            let right_trees = trees
                .iter()
                .filter(|other_tree| other_tree.x > tree.x && other_tree.y == tree.y)
                .sorted_by(|a, b| a.x.cmp(&b.x))
                .collect::<Vec<_>>();

            let viewing_distance_right = get_viewing_distance(tree, right_trees);

            let top_trees = trees
                .iter()
                .filter(|other_tree| other_tree.x == tree.x && other_tree.y < tree.y)
                .sorted_by(|a, b| a.y.cmp(&b.y))
                .rev()
                .collect::<Vec<_>>();

            let viewing_distance_top = get_viewing_distance(tree, top_trees);

            let bottom_trees = trees
                .iter()
                .filter(|other_tree| other_tree.x == tree.x && other_tree.y > tree.y)
                .sorted_by(|a, b| a.y.cmp(&b.y))
                .collect::<Vec<_>>();

            let viewing_distance_bottom = get_viewing_distance(tree, bottom_trees);

            let scenic_score = viewing_distance_left
                * viewing_distance_right
                * viewing_distance_top
                * viewing_distance_bottom;

            scenic_score
        })
        .collect::<Vec<_>>();

    let highest_scenic_score = scenic_scores.iter().max().unwrap();

    println!("{:?}", highest_scenic_score);
}

fn get_viewing_distance(tree: &Tree, next_trees: Vec<&Tree>) -> u32 {
    let mut number_of_trees_visible = 0;

    for next_tree in next_trees {
        number_of_trees_visible += 1;

        if next_tree.height >= tree.height {
            break;
        }
    }

    number_of_trees_visible
}
