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

    let edge_trees = trees
        .iter()
        .filter(|tree| tree.x == min_x || tree.x == max_x || tree.y == min_y || tree.y == max_y)
        .collect::<Vec<_>>();

    let interior_trees = trees
        .iter()
        .filter(|tree| tree.x != min_x && tree.x != max_x && tree.y != min_y && tree.y != max_y)
        .collect::<Vec<_>>();

    let visible_interior_trees = interior_trees
        .iter()
        .filter(|tree| {
            let left_trees = trees
                .iter()
                .filter(|other_tree| other_tree.x < tree.x && other_tree.y == tree.y)
                .collect::<Vec<_>>();

            let is_visible_from_left = left_trees
                .iter()
                .all(|other_tree| other_tree.height < tree.height);

            let right_trees = trees
                .iter()
                .filter(|other_tree| other_tree.x > tree.x && other_tree.y == tree.y)
                .collect::<Vec<_>>();

            let is_visible_from_right = right_trees
                .iter()
                .all(|other_tree| other_tree.height < tree.height);

            let top_trees = trees
                .iter()
                .filter(|other_tree| other_tree.x == tree.x && other_tree.y < tree.y)
                .collect::<Vec<_>>();

            let is_visible_from_top = top_trees
                .iter()
                .all(|other_tree| other_tree.height < tree.height);

            let bottom_trees = trees
                .iter()
                .filter(|other_tree| other_tree.x == tree.x && other_tree.y > tree.y)
                .collect::<Vec<_>>();

            let is_visible_from_bottom = bottom_trees
                .iter()
                .all(|other_tree| other_tree.height < tree.height);

            return is_visible_from_left
                || is_visible_from_right
                || is_visible_from_top
                || is_visible_from_bottom;
        })
        .collect::<Vec<_>>();

    let total = edge_trees.len() + visible_interior_trees.len();

    println!("{:?}", total);
}
