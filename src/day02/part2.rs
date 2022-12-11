#[derive(Debug, PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Round {
    result: RoundResult,
    opponent: Shape,
}

#[derive(Debug)]
enum RoundResult {
    Win,
    Draw,
    Loss,
}

pub fn run(input: &str) {
    let lines = input.lines();

    let mut rounds = Vec::new();

    for line in lines {
        let mut shapes = line.split_whitespace();

        let opponent = match shapes.next().unwrap() {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Invalid shape"),
        };

        let result = match shapes.next().unwrap() {
            "X" => RoundResult::Loss,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => panic!("Invalid shape"),
        };

        rounds.push(Round { result, opponent });
    }

    let scores = rounds.iter().map(|round| {
        let shape = get_player_shape(round);

        let outcome_score = match round.result {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0,
        };

        let shape_score = match shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        return outcome_score + shape_score;
    });

    let total_score = scores.sum::<u32>();

    println!("{}", total_score);
}

fn get_player_shape(round: &Round) -> Shape {
    match round.result {
        RoundResult::Win => match round.opponent {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        RoundResult::Draw => round.opponent,
        RoundResult::Loss => match round.opponent {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
    }
}
