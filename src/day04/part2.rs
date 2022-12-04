#[derive(Debug)]
struct Pair {
    min: u32,
    max: u32,
}

#[derive(Debug)]
struct Assignment {
    pair_one: Pair,
    pair_two: Pair,
}

pub fn run() {
    let input = include_str!("part2.txt");
    let lines = input.lines();

    let mut assignments = Vec::new();
    for line in lines {
        let mut pairs = line.split(',');

        let pair_one_values = pairs.next().unwrap().split('-').collect::<Vec<_>>();

        let min = pair_one_values[0].parse::<u32>().unwrap();
        let max = pair_one_values[1].parse::<u32>().unwrap();
        let pair_one = Pair { min, max };

        let pair_two_values = pairs.next().unwrap().split('-').collect::<Vec<_>>();

        let min = pair_two_values[0].parse::<u32>().unwrap();
        let max = pair_two_values[1].parse::<u32>().unwrap();
        let pair_two = Pair { min, max };

        let assignment = Assignment { pair_one, pair_two };
        assignments.push(assignment);
    }

    let fully_contained_assigments = assignments
        .iter()
        .filter(|assignment| partially_contains(assignment));

    let total = fully_contained_assigments.count();

    println!("{:?}", total);
}

fn partially_contains(assignment: &Assignment) -> bool {
    let pair_one = &assignment.pair_one;
    let pair_two = &assignment.pair_two;

    return (pair_one.min <= pair_two.min && pair_two.min <= pair_one.max)
        || (pair_one.min <= pair_two.max && pair_two.max <= pair_one.max)
        || (pair_two.min <= pair_one.min && pair_one.min <= pair_two.max)
        || (pair_two.min <= pair_one.max && pair_one.max <= pair_two.max);
}
